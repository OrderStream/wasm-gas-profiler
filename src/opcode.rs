use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WasmOpcode {
    Unreachable,
    Nop,
    Block,
    Loop,
    If,
    Else,
    End,
    Br,
    BrIf,
    BrTable,
    Return,
    Call,
    CallIndirect,
    Drop,
    Select,
    LocalGet,
    LocalSet,
    LocalTee,
    GlobalGet,
    GlobalSet,
    I32Load,
    I64Load,
    I32Store,
    I64Store,
    MemorySize,
    MemoryGrow,
    I32Const,
    I64Const,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    HostStorageGet,
    HostStoragePut,
    HostStorageDel,
    HostTtlExtend,
    Unknown(u8),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionInfo {
    pub opcode: WasmOpcode,
    pub offset: usize,
    pub gas_cost: u64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmCostModel {
    pub default_op_cost: u64,
    pub control_flow_cost: u64,
    pub loop_header_cost: u64,
    pub memory_load_store_cost: u64,
    pub memory_grow_cost: u64,
    pub call_cost: u64,
    pub storage_get_cost: u64,
    pub storage_put_cost: u64,
    pub storage_del_cost: u64,
    pub ttl_extend_cost: u64,
}

impl Default for WasmCostModel {
    fn default() -> Self {
        Self {
            default_op_cost: 1,
            control_flow_cost: 3,
            loop_header_cost: 10,
            memory_load_store_cost: 5,
            memory_grow_cost: 1000,
            call_cost: 15,
            storage_get_cost: 800,
            storage_put_cost: 1500,
            storage_del_cost: 600,
            ttl_extend_cost: 400,
        }
    }
}

impl WasmCostModel {
    pub fn cost_for(&self, op: WasmOpcode) -> u64 {
        match op {
            WasmOpcode::Block | WasmOpcode::If | WasmOpcode::Br | WasmOpcode::BrIf | WasmOpcode::BrTable => {
                self.control_flow_cost
            }
            WasmOpcode::Loop => self.loop_header_cost,
            WasmOpcode::Call | WasmOpcode::CallIndirect => self.call_cost,
            WasmOpcode::I32Load | WasmOpcode::I64Load | WasmOpcode::I32Store | WasmOpcode::I64Store => {
                self.memory_load_store_cost
            }
            WasmOpcode::MemoryGrow => self.memory_grow_cost,
            WasmOpcode::HostStorageGet => self.storage_get_cost,
            WasmOpcode::HostStoragePut => self.storage_put_cost,
            WasmOpcode::HostStorageDel => self.storage_del_cost,
            WasmOpcode::HostTtlExtend => self.ttl_extend_cost,
            _ => self.default_op_cost,
        }
    }
}
