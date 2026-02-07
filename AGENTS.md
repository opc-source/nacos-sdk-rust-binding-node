# AGENTS.md

This file provides guidance to AI coding agents when working with code in this repository.

## Project Overview

This is a Node.js native addon providing bindings to the Nacos service discovery and configuration management system. It uses [napi-rs](https://napi.rs/) to bridge Rust (`nacos-sdk-rust`) with Node.js/TypeScript, offering better performance than the official JavaScript SDK which lacks gRPC support.

## Architecture

### Hybrid Rust/Node.js Structure

- **Rust Layer** (`src/*.rs`): Core Nacos client implementation using `nacos-sdk-rust`
  - `lib.rs`: Entry point, `ClientOptions` struct
  - `config.rs`: Configuration management (`NacosConfigClient`)
  - `naming.rs`: Service discovery (`NacosNamingClient`)
  - `plugin.rs`: Config filter plugin interface for encryption/decryption

- **TypeScript Definitions** (`index.d.ts`): Auto-generated from Rust code via `napi-rs` during build

### Key Data Flows

1. **Config Client**: `getConfig()` → Rust SDK → Nacos Server → async response
2. **Naming Client**: `registerInstance()` → Rust SDK → Nacos Server → async response
3. **Subscriptions**: Rust SDK maintains long-lived gRPC connections; pushes updates via `ThreadsafeFunction` callbacks to JS

### Type Conventions

- Rust `Option<T>` maps to TypeScript `T | undefined | null`
- Async Rust methods return JavaScript Promises
- Callbacks use `ThreadsafeFunction` for thread-safe JS invocation from Rust
- Nacos errors are converted to JS `Error` with reason string

## Build System

This project uses:
- **Cargo**: Rust dependency management and compilation
- **napi-rs**: Native Node.js addon framework
- **Yarn 3.4.1**: Package manager (specified in `packageManager` field)

### Common Commands

```bash
# Install dependencies
yarn install

# Build debug native addon (faster, for development)
yarn build:debug

# Build release native addon (optimized)
yarn build

# Run tests (requires compiled .node file)
yarn test

# Collect build artifacts for distribution
yarn artifacts

# Create universal macOS binary (combines x64 + arm64)
yarn universal
```

### Cross-Compilation Targets

The CI builds for multiple platforms (see `.github/workflows/CI.yml`):
- macOS: `x86_64-apple-darwin`, `aarch64-apple-darwin`, `universal-apple-darwin`
- Linux: `x86_64-unknown-linux-gnu/musl`, `aarch64-unknown-linux-gnu/musl`, `armv7-unknown-linux-gnueabihf`
- Windows: `x86_64-pc-windows-msvc`, `i686-pc-windows-msvc`, `aarch64-pc-windows-msvc`
- Android: `aarch64-linux-android`, `armv7-linux-androideabi`

Target-specific builds: `yarn build --target <target-triple>`

### Build Outputs

- Compiled native addon: `nacos-sdk-rust-binding-node.<platform>.node`
- TypeScript definitions: `index.d.ts` (auto-generated)

## Testing

Uses [AVA](https://github.com/avajs/ava) test framework with 3-minute timeout:

```bash
# Run all tests
yarn test

# Run a specific test file
yarn test __test__/index.spec.mjs
```

Note: Tests require the native addon to be built first (`index.node` must exist).

## Environment Variables

- `NACOS_CLIENT_LOGGER_LEVEL`: Log level (default: `INFO`, logs to `$HOME/logs/nacos/`)
- `NACOS_CLIENT_COMMON_THREAD_CORES`: Client thread pool size (default: `1`)
- `NACOS_CLIENT_NAMING_PUSH_EMPTY_PROTECTION`: Protect against empty service list pushes (default: `true`)

See [nacos-sdk-rust docs](https://github.com/nacos-group/nacos-sdk-rust) for more environment variables.

## API Usage Patterns

### Config Client

```javascript
const { NacosConfigClient } = require('nacos-sdk-rust-binding-node');

const client = new NacosConfigClient({
  serverAddr: '127.0.0.1:8848',
  namespace: 'my-namespace',
  // Optional auth:
  username: 'user',
  password: 'pass'
});

const content = await client.getConfig('dataId', 'group');
await client.addListener('dataId', 'group', (err, config) => {
  // Handle push updates
});
```

### Naming Client

```javascript
const { NacosNamingClient } = require('nacos-sdk-rust-binding-node');

const client = new NacosNamingClient({
  serverAddr: '127.0.0.1:8848',
  namespace: 'my-namespace'
});

await client.registerInstance('service', 'group', {
  ip: '192.168.1.1',
  port: 8080
});
```

## Important Implementation Notes

- **removeListener/unSubscribe**: These are NOOPs - the underlying Rust SDK doesn't implement removal, but this is typically not problematic as listeners are long-lived
- **Clients are stateful**: Each client maintains persistent gRPC connections; create once and reuse for the application lifecycle
- **Auth methods**: Supports HTTP token auth (username/password) or Alibaba Cloud RAM (access_key/access_secret)
- **Build dependency**: require `nacos-sdk-rust` and `napi`

## Troubleshooting

- If build fails, ensure Rust toolchain is up to date: `rustup update stable && cargo update`
- For cross-compilation issues, refer to the Docker images and setup in CI workflow
- Native addon loading errors usually indicate platform/architecture mismatch between build target and runtime
