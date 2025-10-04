# 🦀 Chaos Engineering Framework

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)](https://github.com/NLlemain/chaos-engineering-rs)

**A lightweight, cross-platform chaos engineering framework for testing service resilience through controlled failure injection.**

Built in Rust for performance and safety, this framework helps you discover system weaknesses before they impact production. Test how your services handle real-world failures like network issues, resource exhaustion, and process crashes.

## ✨ Features

- **🌐 Cross-Platform**: Full support for Windows, macOS, and Linux with platform-native chaos injection
- **⚡ High Performance**: Rust-powered with async/await, minimal overhead (~15MB memory, <1% CPU)
- **🎯 7 Chaos Types**: Network latency, packet loss, TCP resets, CPU starvation, memory pressure, disk I/O, and process kills
- **📋 YAML Configuration**: Simple, declarative test scenarios with multi-phase support
- **🔄 Reproducible Tests**: Deterministic chaos with configurable random seeds
- **📊 Multiple Output Formats**: Real-time CLI progress, JSON, Markdown reports, and Prometheus metrics
- **🛡️ Safe by Design**: Input validation, no shell injection, clear privilege separation
- **🧪 Testing Built-In**: Example target services and comprehensive test scenarios included

## 🚀 Quick Start

### Prerequisites

- **Rust 1.70+** - [Install Rust](https://www.rust-lang.org/tools/install)
- **Platform-specific** (for network chaos):
  - Linux: `iproute2`, `iptables` (usually pre-installed)
  - macOS: `dnctl`, `pfctl` (built-in, requires sudo)
  - Windows: No additional requirements (uses application-level simulation)

### Installation

```bash
git clone https://github.com/NLlemain/chaos-engineering-rs
cd chaos-engineering-rs
cargo build --release
```

Build takes ~30 seconds. Binaries are located in `target/release/`.

### Your First Test

```bash
# 1. Start a test HTTP service
./target/release/axum_http_service

# 2. In another terminal, run a quick chaos test (3 minutes)
./target/release/chaos run scenarios/quick_test.yaml --verbose

# 3. Try the comprehensive stress test (15 minutes)
./target/release/chaos run scenarios/stress_test.yaml --verbose --output-json results.json
```

That's it! You'll see real-time progress and a summary of how your service handled the chaos.

## 📦 Chaos Injectors

| Injector | Description | Implementation |
|----------|-------------|----------------|
| **network_latency** | Adds configurable delay to network packets (mean + jitter) | Linux: `tc/netem` • macOS: `dnctl` • Windows: app-level |
| **packet_loss** | Randomly drops packets at specified rate | Linux: `tc/netem` • macOS: `dnctl` • Windows: app-level |
| **tcp_reset** | Forcibly terminates TCP connections | Linux: `iptables` • macOS: `pfctl` • Windows: app-level |
| **cpu_starvation** | Saturates CPU cores at specified intensity | Cross-platform busy loops |
| **memory_pressure** | Allocates memory to target usage percentage | Cross-platform heap allocation |
| **disk_slow** | Introduces I/O latency to disk operations | Cross-platform synchronous I/O |
| **process_kill** | Terminates and optionally restarts processes | Platform-specific signals/APIs |

All injectors work on **Windows, macOS, and Linux** with platform-optimized implementations.

## 📝 Creating Test Scenarios

Chaos tests are defined in simple YAML files with a multi-phase structure:

```yaml
name: "HTTP Service Resilience Test"
targets:
  - name: "web_api"
    type: "process"
    process_name: "axum_http_service"

phases:
  # Phase 1: Establish baseline metrics
  - name: "baseline"
    duration: "30s"
    
  # Phase 2: Inject network latency
  - name: "network_stress"
    duration: "60s"
    injections:
      - type: "network_latency"
        target: "web_api"
        delay: "100ms"
        jitter: "20ms"
  
  # Phase 3: Combined resource pressure
  - name: "resource_stress"
    duration: "60s"
    parallel: true
    injections:
      - type: "cpu_starvation"
        intensity: 0.7
      - type: "memory_pressure"
        target_usage: 0.8
        
  # Phase 4: Verify recovery
  - name: "recovery"
    duration: "30s"
```

### Running Tests

```bash
# Validate scenario before running
./target/release/chaos validate scenarios/my_test.yaml

# Run with real-time output
./target/release/chaos run scenarios/my_test.yaml --verbose

# Generate reports in multiple formats
./target/release/chaos run scenarios/my_test.yaml \
  --output-json results.json \
  --output-markdown report.md \
  --verbose
```

### Available Output Formats

- **CLI**: Real-time progress bars with colored output
- **JSON**: Machine-readable for CI/CD integration
- **Markdown**: Human-readable test reports
- **Prometheus**: Metrics export for monitoring dashboards

## 📚 Example Scenarios

### Quick Test (3 minutes)

Perfect for CI/CD pipelines or quick validation:

```yaml
name: "Quick Smoke Test"
phases:
  - name: "baseline"
    duration: "1m"
    
  - name: "cpu_stress"
    duration: "1m"
    injections:
      - type: "cpu_starvation"
        intensity: 0.5  # 50% CPU load
        
  - name: "recovery"
    duration: "1m"
```

Run with: `./target/release/chaos run scenarios/quick_test.yaml --verbose`

### Comprehensive Stress Test (15 minutes)

Progressive failure injection for thorough resilience testing:

```yaml
name: "Comprehensive Stress Test"
phases:
  - name: "baseline"
    duration: "2m"
    
  - name: "light_network_latency"
    duration: "2m"
    injections:
      - type: "network_latency"
        delay: "20ms"
        jitter: "5ms"
        
  - name: "moderate_stress"
    duration: "3m"
    parallel: true
    injections:
      - type: "cpu_starvation"
        intensity: 0.6
      - type: "network_latency"
        delay: "50ms"
        jitter: "10ms"
        
  - name: "heavy_chaos"
    duration: "4m"
    parallel: true
    injections:
      - type: "cpu_starvation"
        intensity: 0.8
      - type: "memory_pressure"
        target_usage: 0.85
      - type: "packet_loss"
        loss_rate: 0.15
        
  - name: "recovery"
    duration: "4m"
```

Run with: `./target/release/chaos run scenarios/stress_test.yaml --verbose --output-json results.json`

## 🏗️ Architecture

The framework is organized into five focused crates:

```
chaos-engineering-rs/
├── chaos_cli/          Command-line interface and user interaction
├── chaos_core/         Core chaos injection engine and injectors
├── chaos_scenarios/    YAML parser, test orchestration, and phase management
├── chaos_targets/      Target discovery and example test services
└── chaos_metrics/      Metrics collection, aggregation, and export

+ scenarios/            Pre-built test scenarios
+ scripts/              Helper scripts for common tasks
```

**Architecture Highlights:**
- **Async-First**: Built on Tokio for concurrent multi-target chaos injection
- **Modular Design**: Each crate has a single responsibility
- **Type-Safe**: Leverages Rust's type system for correctness
- **Extensible**: Easy to add new injectors and metrics exporters

## 🔧 CLI Commands

```bash
# List all available chaos injectors with descriptions
./target/release/chaos list

# Validate a scenario file (syntax and configuration)
./target/release/chaos validate scenarios/my_test.yaml

# Run a chaos test
./target/release/chaos run scenarios/my_test.yaml [OPTIONS]

# Run with multiple output formats
./target/release/chaos run scenarios/stress_test.yaml \
  --verbose \
  --output-json results.json \
  --output-markdown report.md

# Dry run (validate without executing)
./target/release/chaos run scenarios/my_test.yaml --dry-run
```

## 🧪 Test Services

The framework includes three example target services for testing:

### HTTP Service (Axum)
```bash
./target/release/axum_http_service
# Runs on http://localhost:3000
```
Modern async HTTP service built with Axum framework.

### TCP Echo Server
```bash
./target/release/tcp_echo_server
# Listens on port 8080
```
Simple TCP echo service for network chaos testing.

### WebSocket Feed
```bash
./target/release/websocket_feed
# WebSocket server on port 9000
```
Streaming WebSocket server for connection resilience tests.

## 🚀 Helper Scripts

Convenient PowerShell scripts for common workflows (works on Windows, macOS with PowerShell Core, Linux with PowerShell Core):

### Simple Test
```powershell
.\scripts\simple_test.ps1
```
- Starts test HTTP service
- Runs 3-minute chaos test
- Displays results
- Automatic cleanup

### Stress Test
```powershell
.\scripts\stress_test.ps1
```
- Comprehensive 15-minute test
- Multi-phase progressive stress
- Generates JSON and Markdown reports
- Verifies recovery

## 🖥️ Platform Support

| Chaos Type | Linux | macOS | Windows |
|------------|:-----:|:-----:|:-------:|
| **CPU Starvation** | ✅ | ✅ | ✅ |
| **Memory Pressure** | ✅ | ✅ | ✅ |
| **Disk I/O Slow** | ✅ | ✅ | ✅ |
| **Process Kill/Restart** | ✅ | ✅ | ✅ |
| **Network Latency** | ✅ tc/netem | ✅ dnctl | ✅ app-level |
| **Packet Loss** | ✅ tc/netem | ✅ netem | ✅ app-level |
| **TCP Resets** | ✅ iptables | ✅ pfctl | ✅ app-level |

**Fully Cross-Platform**: All chaos injectors work on Windows, macOS, and Linux with platform-optimized implementations.

### Platform-Specific Notes

**Linux** (Recommended for Production Testing)
- Uses kernel-level tools for most realistic network chaos
- Requires `sudo` for network injections
- Install: `sudo apt install iproute2 iptables` (usually pre-installed)

**macOS**
- Uses native `dnctl` and `pfctl` for network chaos
- Requires `sudo` for network injections
- All tools built-in, no installation needed

**Windows**
- Application-level network chaos simulation
- No elevated privileges required for network chaos
- Ideal for local development testing

## ⚡ Performance Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| **Binary Size** | ~6 MB | Optimized release build |
| **Build Time** | ~30 seconds | Full `--release` build from clean |
| **Memory Footprint** | ~15 MB | Idle, excluding injected chaos |
| **CPU Overhead** | <1% | Framework overhead during chaos |
| **Startup Time** | <100ms | From invocation to first injection |
| **Compilation** | 0 warnings | Clean build on stable Rust |

**Designed for minimal overhead** - the framework itself won't be the bottleneck in your tests.

## 🛡️ Safety & Security

### Best Practices

**Chaos engineering is powerful - use it responsibly:**

1. **Start in Non-Production**: Always test new scenarios in dev/staging environments first
2. **Progressive Intensity**: Begin with low chaos intensity and short durations
3. **Active Monitoring**: Watch system metrics during chaos tests
4. **Have a Kill Switch**: Know how to stop tests immediately (`Ctrl+C`)
5. **Staging Data Only**: Never test with production customer data
6. **Document Findings**: Record what breaks and how to fix it

### Privilege Requirements

| Operation | Linux/macOS | Windows | Reason |
|-----------|-------------|---------|--------|
| Network chaos | `sudo` | User | Linux/macOS modify kernel networking |
| CPU/Memory/Disk | User | User | Standard process operations |
| Process kill (own) | User | User | Normal process management |
| Process kill (other) | `sudo` | Admin | Cross-user process termination |

### Security by Design

- ✅ **Input Validation**: All YAML configs validated before execution
- ✅ **No Shell Injection**: Uses Rust's safe `Command` API
- ✅ **Privilege Separation**: Clear boundary between user and root operations
- ✅ **Audit Logging**: All chaos actions logged with timestamps
- ✅ **Deterministic Behavior**: Reproducible tests with seed values
- ✅ **Zero External Dependencies**: Core functionality requires no external services

For vulnerability reporting, see [SECURITY.md](SECURITY.md).

## 📖 Documentation

- **[QUICKSTART.md](QUICKSTART.md)** - Get up and running in 5 minutes
- **[SECURITY.md](SECURITY.md)** - Security considerations and vulnerability reporting
- **[LICENSE-MIT](LICENSE-MIT)** - MIT License details
- **[CHANGES.md](CHANGES.md)** - Recent changes and improvements

## 🤝 Contributing

Contributions are welcome! Here's how to get started:

1. **Fork** the repository
2. **Create** a feature branch: `git checkout -b feature/amazing-feature`
3. **Make** your changes with clear commit messages
4. **Test** your changes: `cargo test --all`
5. **Format** code: `cargo fmt --all`
6. **Lint** code: `cargo clippy --all -- -D warnings`
7. **Submit** a pull request

### Development Guidelines

- Write tests for new features
- Maintain cross-platform compatibility
- Document public APIs with rustdoc comments
- Follow existing code style and patterns
- Keep commits focused and atomic

## 🔄 CI/CD Ready

While this project doesn't currently have CI/CD configured, it's designed to integrate easily:

**Recommended GitHub Actions Setup:**
- Multi-platform testing (Linux, macOS, Windows)
- Rust stable and nightly builds
- Clippy linting and format checking
- Security audit with `cargo-audit`
- Automated release builds
- Documentation generation

## 📜 License

This project is dual-licensed under:

- **MIT License** - See [LICENSE-MIT](LICENSE-MIT) for details
- **Apache License 2.0** - Optional alternative licensing

Choose the license that best fits your needs.

## 🎯 Project Goals & Philosophy

**Why this project exists:**

Modern distributed systems are complex and failures are inevitable. Rather than discovering weaknesses in production, this framework helps you:

- **Find problems early** through controlled chaos in test environments
- **Build confidence** in your system's resilience and recovery mechanisms
- **Document failure modes** with reproducible test scenarios
- **Validate SLOs** under adverse conditions

**Design principles:**

- ⚡ **Performance First**: Rust-native with minimal overhead
- 🔒 **Safe by Default**: Type-safe, validated inputs, clear privilege boundaries
- 🌍 **Cross-Platform**: Works the same everywhere (with platform-optimized implementations)
- 📦 **Batteries Included**: Example services and scenarios ready to run
- 🧩 **Simple & Composable**: YAML configs, modular architecture

## 🌟 Inspired By

This framework draws inspiration from excellent chaos engineering tools:

- [Chaos Mesh](https://chaos-mesh.org/) - Kubernetes-native chaos engineering platform
- [Pumba](https://github.com/alexei-led/pumba) - Chaos testing for Docker
- [Litmus](https://litmuschaos.io/) - Cloud-native chaos engineering
- [Chaos Monkey](https://netflix.github.io/chaosmonkey/) - Netflix's original chaos tool

## 💬 Support & Contact

- **🐛 Found a bug?** [Open an issue](https://github.com/NLlemain/chaos-engineering-rs/issues)
- **💡 Feature request?** [Open an issue](https://github.com/NLlemain/chaos-engineering-rs/issues) or submit a PR
- **❓ Questions?** Check [QUICKSTART.md](QUICKSTART.md) or open a discussion
- **💼 Connect:** [LinkedIn](https://www.linkedin.com/in/ninian-lemain-888524330/)
- **📧 Email:** [ninianlmm@gmail.com](mailto:ninianlmm@gmail.com)

## ⚠️ Project Status

**Active Development** - This framework is functional and tested for small to medium services. 

**Use Cases:**
- ✅ Local development testing
- ✅ CI/CD integration
- ✅ Staging environment validation
- ✅ Small to medium production systems (with caution)
- ⚠️ Large-scale production (consider more mature tools like Chaos Mesh)

**What's Working:**
- All 7 chaos injectors across all platforms
- Multi-phase test scenarios
- Multiple output formats
- Example target services
- Comprehensive documentation

**Future Enhancements:**
- Kubernetes integration
- Distributed chaos coordination
- Advanced metrics and observability
- Web UI for test management
- Plugin system for custom injectors

---

## 🦀 Happy Chaos Testing!

**Remember:** The goal isn't to break things - it's to learn how systems fail so you can build them better.

*"Everything fails all the time." - Werner Vogels, CTO of Amazon*
