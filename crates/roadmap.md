# Core Rust Roadmap

## Vision

The goal is to migrate all non-UI logic from C++ to Rust. The C++ layer will be reduced to Qt UI code and thin FFI wrappers. Rust becomes the single source of truth for business logic, data processing, and system interactions.

## Current State

All 6 phases are implemented across 17 workspace crates with 80+ source files organized in subdirectories.

## Crates Overview

| Crate | Phase | Status | Modules |
|-------|-------|--------|---------|
| `error` | Foundation | Complete | `core`, `ext` |
| `hashing` | Foundation | Complete | `core`, `checksums` |
| `archive` | Foundation | Complete | `core`, `streaming` |
| `gzip` | Foundation | Complete | `core` |
| `filesystem` | Foundation | Complete | `core`, `watcher` |
| `json` | Foundation | Complete | `core`, `schema` |
| `markdown` | Foundation | Complete | `core`, `ext` |
| `string_utils` | Foundation | Complete | `core`, `format` |
| `http_client` | Network | Complete | `core`, `api` |
| `launcher_api` | Network | Complete | `modrinth`, `curseforge`, `ftb`, `technic`, `atlauncher`, `models` |
| `download_manager` | Network | Complete | `core`, `models`, `utils` |
| `instance_manager` | Instance Mgmt | Complete | `manifest`, `component`, `config`, `resolver` |
| `mod_metadata` | Mod Ecosystem | Complete | `core`, `install` |
| `auth` | Authentication | Complete | `models`, `microsoft`, `mojang`, `elyby`, `token_store` |
| `settings` | Settings | Complete | `core`, `migration` |
| `memory_profiler` | Foundation | Complete | `core`, `report` |
| `process_launcher` | Instance Mgmt | Complete | `core`, `models` |
| `core` | Integration | Complete | `ffi/*`, `cxx_bridge` |

## Phase 1: Foundation

### Hashing and Checksums
SHA-256, SHA-512, MD5 computation over byte slices, file streams, and async readers. ChecksumSet for multi-algorithm verification.

### Archive Processing
ZIP and TAR.GZ extraction, creation, merging, streaming extraction with progress callbacks, path traversal protection.

### Filesystem Operations
Read/write/append, directory management, path normalization, filename sanitization, file watching.

### JSON Processing
Parse, serialize, validate JSON schemas, binary JSON detection, hex conversion, deep merge.

### String Utilities
Natural comparison, human-readable sizes, UUID generation, URL truncation, HTML list patching, formatting.

### Markdown
HTML rendering with tables/footnotes/strikethrough/tasklists, plain text extraction, heading extraction, word count.

## Phase 2: Network Layer

### HTTP Client
Configurable user-agent, timeout, headers. Retry with exponential backoff, rate-limit detection, resume support, streaming downloads.

### API Clients
Unified `ApiClient` trait with implementations for Modrinth, CurseForge, FTB, Technic, ATLauncher. Search packs, fetch versions, download URLs.

### Download Manager
File downloads with SHA-256 verification, batch and parallel downloads, progress tracking, format utilities.

## Phase 3: Instance Management

### Manifest Resolution
Fetch Mojang version manifest, resolve version details, list releases/snapshots.

### Component Graph
Dependency graph with topological sort, cycle detection, conflict checking. Components for Minecraft, Forge, NeoForge, Fabric, Quilt, LiteLoader.

### Instance Configuration
JSON-based instance config with memory, resolution, Java args. Create, load, save, list, delete, duplicate instances.

### Library Resolution
Resolve classpath, evaluate OS rules, determine main class per loader.

## Phase 4: Mod Ecosystem

### Mod Metadata
Strongly typed structs for Modrinth and CurseForge projects/versions. Parsing, filtering by loader/MC version, date sorting.

### Mod Installation
Common `ModInstaller` trait with Simple/CurseForge/Modrinth implementations. Download, hash verify, install, uninstall.

## Phase 5: Authentication

### Microsoft OAuth
Full device code flow: authorize URL, code exchange, XBL auth, XSTS auth, Minecraft login, profile fetch, token refresh.

### Mojang Yggdrasil
Authenticate, refresh, validate, invalidate, signout. Full Yggdrasil API coverage.

### Ely.by
Authenticate and refresh for the Ely.by auth server.

### Token Storage
JSON-based secure token persistence with save/load/remove, expiration checking.

## Phase 6: Settings and Persistence

### Settings Management
INI file parsing/serialization, typed getters/setters, aliases, defaults, atomic saves.

### Data Migration
Versioned migration framework with `DataMigration` trait, `MigrationManager`. Built-in v1→v2 and v2→v3 migrations.

## CI Pipeline

- **rust-fmt**: Code formatting check
- **rust-clippy**: Lint with deny warnings
- **rust-test**: Release build + all tests
- **rust-doc**: Documentation with deny warnings
- **rust-audit**: Security vulnerability audit
- **rust-miri**: Undefined behavior detection
- **rust-outdated**: Dependency freshness check
- **rust-unused-deps**: Unused dependency detection

## Integration Strategy

Each phase produces a Rust library that links statically into the launcher binary. The C++ layer includes a generated header and calls Rust functions through `extern "C"` wrappers in `core/src/ffi/`.

## Testing Strategy

Each Rust module includes unit tests. The CI pipeline runs all tests independently of the C++ build. Additional verification via clippy, miri, audit, and doc checks.

## Completion Criteria

The migration is complete when all business logic lives in Rust. C++ contains only Qt UI code and FFI wrappers. Every Rust module has >90% test coverage.
