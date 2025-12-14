# OxalSLIEN

**ALIEN-inspired Claim Simulator in Rust**

OxalSLIEN simulates claim verification using concepts from the **ALIEN theorem**:
- Acceptance probability with multiple provers and retry budget
- Provability levels
- Finalization likelihood

## Features
- Simple CLI for submitting claims
- ALIEN-inspired acceptance probability calculation
- Provability Level simulation (n = 1..5)
- Status output: Low / Likely / Highly likely to finalize

## Requirements
- Rust 1.70+ (stable)

## Build and Run

```bash
# Clone repo
git clone <repo-url>
cd oxalslien

# Build
cargo build --release

# Run
cargo run
