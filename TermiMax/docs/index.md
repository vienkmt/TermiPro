# TermiMax Flutter Documentation Index

Welcome to TermiMax Flutter documentation. This directory contains comprehensive guides for the professional serial monitor application.

## Documentation Structure

### 1. [README.md](/README.md) - Quick Start Guide
**Location**: Project root (`README.md`)
**Lines**: 302 LOC
**Purpose**: Getting started, installation, build commands, overview

**Covers**:
- Project overview and features
- Quick start and prerequisites
- Installation and running instructions
- Architecture overview
- Configuration reference
- Dependencies and troubleshooting

**Read this first** for getting the project set up and running.

---

### 2. [project-overview-pdr.md](./project-overview-pdr.md) - Product Requirements
**Location**: `docs/project-overview-pdr.md`
**Lines**: 313 LOC
**Purpose**: Product vision, features, requirements, roadmap

**Covers**:
- Executive summary and product vision
- Implemented features (detailed)
- Planned features and roadmap (v1.1, v1.2, v2.0)
- Technical and non-functional requirements
- Acceptance criteria
- Architecture decisions with rationale
- Testing strategy and success metrics
- Known limitations and constraints

**Read this** to understand what's implemented, what's planned, and why decisions were made.

---

### 3. [system-architecture.md](./system-architecture.md) - Architecture & Design
**Location**: `docs/system-architecture.md`
**Lines**: 509 LOC
**Purpose**: System design, threading model, data flow, performance

**Covers**:
- Complete architecture overview (diagram)
- Layer breakdown (UI, FFI, Rust, OS)
- Threading model and synchronization
- State management (Dart and Rust)
- Data flow diagrams (connection, RX, TX)
- Batching system for performance
- Configuration and build process
- Error handling architecture
- Performance characteristics
- Security considerations
- Extensibility points

**Read this** to understand how the system works internally, especially for modifying backend logic.

---

### 4. [code-standards.md](./code-standards.md) - Coding Conventions
**Location**: `docs/code-standards.md`
**Lines**: 780 LOC (largest file)
**Purpose**: Coding standards, naming conventions, best practices

**Covers**:

**Dart/Flutter**:
- File organization and structure
- Naming conventions (variables, classes, constants)
- Code structure and method ordering
- Documentation standards
- Error handling patterns
- Import organization
- State management patterns
- Theme and styling guidelines
- Performance best practices

**Rust**:
- File organization
- Naming conventions
- Error handling and Result pattern
- Thread safety (Mutex, AtomicBool)
- Documentation
- Code style and formatting
- Performance considerations
- Testing examples

**Code review checklist** and automation tools included.

**Read this** before writing code to ensure consistency.

---

### 5. [codebase-summary.md](./codebase-summary.md) - Code Overview
**Location**: `docs/codebase-summary.md`
**Lines**: 527 LOC
**Purpose**: Codebase structure, components, file guide

**Covers**:
- Repository structure (complete tree)
- Core components (SerialScreen, models, theme, widgets, Rust backend)
- Data flow diagram
- File size analysis
- Dependency graph
- Key algorithms (parsing, batching, port filtering)
- Configuration files explained
- Build process
- Testing structure
- Performance characteristics
- Quick navigation guide

**Read this** to understand the codebase layout and find specific components.

---

## Getting Started Paths

### I'm a new developer to the project
1. Start with **README.md** - understand what the project is
2. Read **project-overview-pdr.md** - understand features and requirements
3. Skim **system-architecture.md** - understand how components interact
4. Refer to **code-standards.md** when writing code

### I need to add a feature
1. Check **project-overview-pdr.md** for scope and acceptance criteria
2. Review **system-architecture.md** for affected components
3. Check **codebase-summary.md** to find relevant files
4. Code following **code-standards.md** conventions

### I need to debug an issue
1. Check **system-architecture.md** data flow diagrams
2. Find component in **codebase-summary.md** quick navigation
3. Review **code-standards.md** error handling patterns
4. Check project root **README.md** troubleshooting section

### I need to optimize performance
1. Review **system-architecture.md** performance characteristics
2. Check **code-standards.md** performance best practices
3. Analyze bottlenecks in **system-architecture.md**

### I need to change the UI
1. Review **codebase-summary.md** "Sidebar Widgets" section
2. Check **code-standards.md** styling guidelines
3. Modify relevant file in `lib/widgets/` or `lib/screens/`

---

## Quick Reference

| Task | Location | Read |
|------|----------|------|
| Build and run | README.md | Setup section |
| Understand requirements | project-overview-pdr.md | Features, Roadmap |
| Add serial command | system-architecture.md | Extensibility Points |
| Add UI component | code-standards.md | Flutter State Management |
| Find a file | codebase-summary.md | File Structure |
| Understand data flow | system-architecture.md | Data Flow Diagrams |
| Debug connection | README.md | Troubleshooting |
| Code style | code-standards.md | Naming & Structure |
| Thread safety | system-architecture.md | Threading Model |
| Performance tips | code-standards.md | Performance Best Practices |

---

## File Statistics

| File | Lines | Size | Type |
|------|-------|------|------|
| README.md | 302 | 11KB | Quick start |
| project-overview-pdr.md | 313 | 10KB | Requirements |
| system-architecture.md | 509 | 13KB | Design |
| code-standards.md | 780 | 16KB | Standards |
| codebase-summary.md | 527 | 16KB | Overview |
| **Total** | **2,431** | **66KB** | **Documentation** |

All files stay under the 800 LOC limit (largest is 780 LOC).

---

## Document Versions

| File | Version | Updated |
|------|---------|---------|
| README.md | 1.0 | Jan 2025 |
| project-overview-pdr.md | 1.0 | Jan 13, 2025 |
| system-architecture.md | 1.0 | Jan 13, 2025 |
| code-standards.md | 1.0 | Jan 13, 2025 |
| codebase-summary.md | 1.0 | Jan 13, 2025 |

---

## Key Concepts Quick Links

### Architecture
- [FFI Bridge Layer](./system-architecture.md#2-ffi-bridge-layer)
- [Rust Backend Layer](./system-architecture.md#3-rust-backend-layer)
- [Threading Model](./system-architecture.md#threading-model)

### Components
- [SerialScreen](./codebase-summary.md#2-serial-screen-main-ui)
- [Data Models](./codebase-summary.md#3-data-models)
- [Theme System](./codebase-summary.md#4-theme-system)
- [Serial Backend](./codebase-summary.md#6-rust-backend)

### Features
- [Data Transmission](./project-overview-pdr.md#2-data-transmission)
- [Data Reception](./project-overview-pdr.md#3-data-reception--display)
- [High-Performance Data Handling](./project-overview-pdr.md#4-high-performance-data-handling)
- [Batching System](./system-architecture.md#batching-system-performance-optimization)

### Standards
- [Dart Naming](./code-standards.md#naming-conventions)
- [Rust Naming](./code-standards.md#naming-conventions-1)
- [Error Handling](./code-standards.md#error-handling)
- [Documentation](./code-standards.md#documentation)

---

## Contributing

When making changes to the codebase:

1. **Code**: Follow guidelines in [code-standards.md](./code-standards.md)
2. **Architecture**: Understand impact using [system-architecture.md](./system-architecture.md)
3. **Documentation**: Update relevant doc files if behavior changes
4. **Testing**: Add tests as per standards
5. **Review**: Ensure code follows conventions

---

## Support & Questions

For questions about specific areas:

- **Building/Setup**: See README.md → Quick Start section
- **What to build**: See project-overview-pdr.md → Planned Features
- **How to build it**: See system-architecture.md → Extensibility Points
- **Code style**: See code-standards.md → relevant section
- **Where's the code**: See codebase-summary.md → Quick Navigation

---

**Last Updated**: January 13, 2025
**Documentation Status**: Complete for v1.0.0
**Next Update**: When moving to v1.1.0 development
