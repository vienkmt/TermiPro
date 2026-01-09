// MQTT Client module for TermiPro
// Using rumqttc for async MQTT 5.0 support

use parking_lot::Mutex;
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet, QoS, Transport};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

// ===================== MQTT STRUCTS =====================

/// MQTT Connection configuration from frontend
#[derive(Debug, Deserialize, Clone)]
pub struct MqttConfig {
    pub connection_id: String,
    pub broker_host: String,
    pub broker_port: u16,
    pub client_id: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub clean_session: bool,
    pub keep_alive_secs: u16,
    pub protocol: String, // "tcp", "tls", "ws", "wss"
    // Last Will and Testament (LWT)
    pub lwt_topic: Option<String>,
    pub lwt_message: Option<String>,
    pub lwt_qos: Option<u8>,
    pub lwt_retain: Option<bool>,
}

/// MQTT message data emitted to frontend (also used for import/export)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MqttMessage {
    pub connection_id: String,
    pub topic: String,
    pub payload: Vec<u8>,
    pub qos: u8,
    pub retain: bool,
    pub timestamp: u64,
    pub direction: String, // "tx" or "rx"
}

/// MQTT connection status event
#[derive(Debug, Serialize, Clone)]
pub struct MqttConnectionStatus {
    pub connection_id: String,
    pub status: String, // "connecting", "connected", "disconnected", "error"
    pub message: Option<String>,
    pub timestamp: u64,
}

/// Handle for managing a single MQTT connection
#[allow(dead_code)]
pub struct MqttConnectionHandle {
    pub client: AsyncClient,
    pub running: Arc<AtomicBool>,
    pub config: MqttConfig,
    pub subscriptions: Arc<Mutex<Vec<String>>>,
}

impl MqttConnectionHandle {
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

/// State for managing all MQTT connections
pub struct MqttState {
    pub connections: Arc<Mutex<HashMap<String, MqttConnectionHandle>>>,
    pub runtime: tokio::runtime::Runtime,
}

impl Default for MqttState {
    fn default() -> Self {
        Self {
            connections: Arc::new(Mutex::new(HashMap::new())),
            runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(2)
                .enable_all()
                .build()
                .expect("Failed to create MQTT runtime"),
        }
    }
}

// ===================== HELPER FUNCTIONS =====================

/// Get current timestamp in milliseconds
pub fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Convert QoS u8 to rumqttc QoS enum
fn to_qos(qos: u8) -> QoS {
    match qos {
        0 => QoS::AtMostOnce,
        1 => QoS::AtLeastOnce,
        _ => QoS::ExactlyOnce,
    }
}

/// Parse hex string to bytes (e.g., "48 65 6C 6C 6F" -> [72, 101, 108, 108, 111])
pub fn parse_hex_string(hex: &str) -> Result<Vec<u8>, String> {
    hex.split_whitespace()
        .map(|s| u8::from_str_radix(s, 16).map_err(|e| format!("Invalid hex: {}", e)))
        .collect()
}

// ===================== MQTT OPERATIONS =====================

/// Connect to MQTT broker
pub async fn connect_mqtt(
    config: MqttConfig,
    app: AppHandle,
) -> Result<(MqttConnectionHandle, EventLoop), String> {
    let mut mqtt_options = MqttOptions::new(
        &config.client_id,
        &config.broker_host,
        config.broker_port,
    );

    // Basic configuration
    mqtt_options.set_keep_alive(Duration::from_secs(config.keep_alive_secs as u64));
    mqtt_options.set_clean_session(config.clean_session);

    // Authentication - set credentials if username is provided
    // ThingsBoard and similar IoT platforms only need username (access token), password can be empty
    if let Some(user) = &config.username {
        if !user.is_empty() {
            let pass = config.password.as_deref().unwrap_or("");
            mqtt_options.set_credentials(user, pass);
        }
    }

    // Last Will and Testament (LWT)
    if let (Some(topic), Some(message)) = (&config.lwt_topic, &config.lwt_message) {
        if !topic.is_empty() {
            let qos = to_qos(config.lwt_qos.unwrap_or(0));
            let retain = config.lwt_retain.unwrap_or(false);
            mqtt_options.set_last_will(rumqttc::LastWill::new(
                topic,
                message.as_bytes().to_vec(),
                qos,
                retain,
            ));
        }
    }

    // Transport based on protocol
    match config.protocol.as_str() {
        "tls" => {
            // TLS with rustls (CA certs only, no self-signed)
            let tls_config = rumqttc::TlsConfiguration::default();
            mqtt_options.set_transport(Transport::Tls(tls_config));
        }
        "ws" => {
            mqtt_options.set_transport(Transport::Ws);
        }
        "wss" => {
            let tls_config = rumqttc::TlsConfiguration::default();
            mqtt_options.set_transport(Transport::Wss(tls_config));
        }
        _ => {
            // Default TCP, no special transport needed
        }
    }

    // Create client with capacity 100
    let (client, eventloop) = AsyncClient::new(mqtt_options, 100);

    let running = Arc::new(AtomicBool::new(true));
    let subscriptions = Arc::new(Mutex::new(Vec::new()));

    // Emit connecting status
    let _ = app.emit(
        "mqtt-status",
        MqttConnectionStatus {
            connection_id: config.connection_id.clone(),
            status: "connecting".to_string(),
            message: Some(format!("Connecting to {}:{}", config.broker_host, config.broker_port)),
            timestamp: get_timestamp(),
        },
    );

    let handle = MqttConnectionHandle {
        client,
        running,
        config,
        subscriptions,
    };

    Ok((handle, eventloop))
}

/// Run the MQTT eventloop (should be spawned as a background task)
pub async fn run_eventloop(
    mut eventloop: EventLoop,
    connection_id: String,
    running: Arc<AtomicBool>,
    app: AppHandle,
) {
    while running.load(Ordering::Relaxed) {
        match eventloop.poll().await {
            Ok(event) => {
                match event {
                    Event::Incoming(Packet::Publish(publish)) => {
                        // Received message from subscribed topic
                        let msg = MqttMessage {
                            connection_id: connection_id.clone(),
                            topic: publish.topic.clone(),
                            payload: publish.payload.to_vec(),
                            qos: publish.qos as u8,
                            retain: publish.retain,
                            timestamp: get_timestamp(),
                            direction: "rx".to_string(),
                        };
                        let _ = app.emit("mqtt-data", &msg);
                    }
                    Event::Incoming(Packet::ConnAck(connack)) => {
                        // Connection acknowledged
                        if connack.code == rumqttc::ConnectReturnCode::Success {
                            let _ = app.emit(
                                "mqtt-status",
                                MqttConnectionStatus {
                                    connection_id: connection_id.clone(),
                                    status: "connected".to_string(),
                                    message: None,
                                    timestamp: get_timestamp(),
                                },
                            );
                        } else {
                            let _ = app.emit(
                                "mqtt-status",
                                MqttConnectionStatus {
                                    connection_id: connection_id.clone(),
                                    status: "error".to_string(),
                                    message: Some(format!("Connection rejected: {:?}", connack.code)),
                                    timestamp: get_timestamp(),
                                },
                            );
                        }
                    }
                    Event::Incoming(Packet::SubAck(_)) => {
                        // Subscription acknowledged - no need to emit event
                    }
                    Event::Incoming(Packet::PubAck(_)) |
                    Event::Incoming(Packet::PubRec(_)) |
                    Event::Incoming(Packet::PubComp(_)) => {
                        // Publish acknowledged - QoS 1/2 handshake
                    }
                    Event::Incoming(Packet::Disconnect) => {
                        let _ = app.emit(
                            "mqtt-status",
                            MqttConnectionStatus {
                                connection_id: connection_id.clone(),
                                status: "disconnected".to_string(),
                                message: Some("Broker initiated disconnect".to_string()),
                                timestamp: get_timestamp(),
                            },
                        );
                    }
                    _ => {}
                }
            }
            Err(e) => {
                // Connection error - emit status and continue (auto-reconnect)
                let _ = app.emit(
                    "mqtt-status",
                    MqttConnectionStatus {
                        connection_id: connection_id.clone(),
                        status: "error".to_string(),
                        message: Some(format!("Connection error: {}", e)),
                        timestamp: get_timestamp(),
                    },
                );

                // Small delay before retry
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }

    // Emit disconnected when loop exits
    let _ = app.emit(
        "mqtt-status",
        MqttConnectionStatus {
            connection_id,
            status: "disconnected".to_string(),
            message: Some("Connection closed".to_string()),
            timestamp: get_timestamp(),
        },
    );
}

/// Subscribe to a topic
pub async fn subscribe_topic(
    client: &AsyncClient,
    topic: &str,
    qos: u8,
) -> Result<(), String> {
    client
        .subscribe(topic, to_qos(qos))
        .await
        .map_err(|e| format!("Subscribe error: {}", e))
}

/// Unsubscribe from a topic
pub async fn unsubscribe_topic(client: &AsyncClient, topic: &str) -> Result<(), String> {
    client
        .unsubscribe(topic)
        .await
        .map_err(|e| format!("Unsubscribe error: {}", e))
}

/// Publish a message to a topic
pub async fn publish_message(
    client: &AsyncClient,
    topic: &str,
    payload: Vec<u8>,
    qos: u8,
    retain: bool,
) -> Result<(), String> {
    client
        .publish(topic, to_qos(qos), retain, payload)
        .await
        .map_err(|e| format!("Publish error: {}", e))
}

/// Disconnect from broker
pub async fn disconnect(client: &AsyncClient) -> Result<(), String> {
    client
        .disconnect()
        .await
        .map_err(|e| format!("Disconnect error: {}", e))
}
