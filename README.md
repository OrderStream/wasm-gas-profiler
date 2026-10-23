# wasm-gas-profiler ⚡🔍

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![OrderStream Polar](https://img.shields.io/badge/Polar-OrderStream-blue.svg)](https://polar.sh/OrderStream)
[![Stellar XLM/USDC](https://img.shields.io/badge/Stellar-USDC%2FXLM-black.svg)](#direct-web3-settlement-wallets)
[![Base USDC](https://img.shields.io/badge/Base-USDC-0052FF.svg)](#direct-web3-settlement-wallets)

**Instruction-level WASM opcode analyzer, CPU fuel profiler, and memory slope monitor for Soroban smart contracts on Stellar.**

---

## 🏛️ Executive Capabilities

- **Instruction-Level Gas Modeling**: Calculates granular CPU fuel costs for every WASM opcode matching Soroban Protocol 22 host cost parameters.
- **Storage Host Call Hotspotting**: Detects expensive `storage_get`, `storage_put`, and `extend_ttl` operations in contract execution paths.
- **Loop & Unbounded Iteration Risk Analysis**: Pinpoints nested loops containing storage mutations that could trigger out-of-gas evictions or DoS vulnerabilities.
- **Flamegraph & CI/CD Regression Exports**: Generates flamegraph stack samples and machine-readable JSON metrics for CI gas threshold regression checks.

---

## 💳 Enterprise Pricing & Retainers

| Tier | Price | Scope | Deliverable |
| :--- | :--- | :--- | :--- |
| **Open Source / Free** | **$0** | Single Contracts | Standalone CLI opcode profiler & flamegraph generator |
| **Starter CI Suite** | **$499 / mo** | Up to 10 Repos | Automated CI gas regression checks across PRs |
| **Team Platform** | **$1,499 / mo** | Up to 50 Repos | CI gas regression bounds, storage TTL telemetry alarms, 24/7 incident portal |
| **Enterprise Platform** | **$4,999 / mo** | Monorepos & Chains | Hosted live memory profiling dashboard, custom WASM runtime bytecode instrumentation, SLA |
| **One-Time Invariant & Gas Audit** | **$2,500 / audit** | Single Protocol | Full instruction-level opcode teardown and official SHA-256 Verified Audit Certificate |

---

## ⚡ Direct Web3 Settlement Wallets

OrderStream accepts direct crypto payments with **0% intermediary platform cuts**:

* **Stellar Protocol (USDC / XLM)**:  
  `GABFQIK63R2NETJM7T673EAMZN4RJLLGP3OFUEJU5ZVTGWUKULZJNL6`  
  *(Required Memo: `400566831`)*

* **EVM Base Network (USDC / ETH)**:  
  `0x5de2b62434754bfd39ef350a3a867ed3eced5074`

* **Credit Card & Invoicing**:  
  Subscribe online via [Polar.sh/OrderStream](https://polar.sh/OrderStream).

---

## 🚀 CLI Usage

```bash
# Build binary
cargo build --release

# Profile contract WASM binary
./target/release/wasm-gas-profiler profile \
  --wasm target/wasm32-unknown-unknown/release/contract.wasm \
  --out-json gas_profile.json \
  --out-flamegraph flame.txt
```

---

## 🛡️ GitHub Action Integration

Add the WASM Gas Profiler gate to your CI pipeline:

```yaml
- name: Profile Soroban WASM Gas
  uses: OrderStream/wasm-gas-profiler@main
  with:
    wasm_path: "target/wasm32-unknown-unknown/release/contract.wasm"
    out_json: "gas_profile.json"
```
