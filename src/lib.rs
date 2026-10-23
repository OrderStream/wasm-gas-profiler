//! Instruction-level WASM opcode analyzer and CPU fuel profiler for Soroban contracts.

pub mod analyzer;
pub mod opcode;
pub mod reporter;

pub use analyzer::{FunctionProfile, GasHotspot, LoopRisk, WasmGasAnalyzer, WasmGasProfile};
pub use opcode::{InstructionInfo, WasmCostModel, WasmOpcode};
pub use reporter::GasReporter;
