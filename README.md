# wasm-gas-profiler

Instruction-level WASM opcode analyzer and CPU fuel profiler for Soroban smart contracts on Stellar.

## Capabilities
- **Instruction-Level Gas Modeling**: Calculates granular CPU fuel costs for every WASM opcode matching Soroban host cost parameters.
- **Storage Host Call Hotspotting**: Detects expensive `storage_get`, `storage_put`, and `extend_ttl` operations in contract execution paths.
- **Loop & Unbounded Iteration Risk Analysis**: Pinpoints nested loops containing storage mutations that could trigger out-of-gas evictions or denial-of-service vulnerabilities.
- **Flamegraph and JSON Export**: Generates flamegraph stack samples and machine-readable JSON metrics for CI/CD gas regressions.

## CLI Usage
```bash
# Profile contract WASM binary
wasm-gas-profiler profile --wasm target/wasm32-unknown-unknown/release/contract.wasm --out-json gas_profile.json --out-flamegraph flame.txt
```
