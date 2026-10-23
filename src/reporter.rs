use crate::analyzer::WasmGasProfile;

pub struct GasReporter;

impl GasReporter {
    pub fn format_text_report(profile: &WasmGasProfile) -> String {
        let mut out = String::new();

        out.push_str("========================================================\n");
        out.push_str("    SOROBAN WASM INSTRUCTION GAS & FUEL PROFILER\n");
        out.push_str("========================================================\n\n");

        out.push_str(&format!("📦 Total Bytecode Size: {} bytes\n", profile.total_module_bytes));
        out.push_str(&format!("⚡ Total Estimated Fuel: {} gas units\n", profile.total_estimated_gas));
        out.push_str(&format!("💾 Total Storage Host Calls: {}\n", profile.total_storage_ops));
        out.push_str(&format!("🔧 Functions Analyzed: {}\n\n", profile.total_functions));

        out.push_str("---------------- FUNCTION PROFILES ---------------------\n");
        for f in &profile.functions {
            out.push_str(&format!(
                "  • fn {:<24} | Gas: {:>8} | Instructions: {:>5} | Loops: {:>2} | Storage Ops: {:>2}\n",
                f.name, f.estimated_cpu_gas, f.total_instructions, f.loops_count, f.storage_ops_count
            ));
        }

        if !profile.loop_risks.is_empty() {
            out.push_str("\n🚨 ---------------- LOOP RISK ADVISORIES ------------------\n");
            for r in &profile.loop_risks {
                out.push_str(&format!(
                    "  [!] {} in fn `{}` at offset +{} (depth: {}, storage inside: {})\n",
                    r.risk_severity, r.function_name, r.loop_offset, r.nesting_depth, r.has_storage_calls_inside
                ));
            }
        }

        if !profile.hotspots.is_empty() {
            out.push_str("\n🔥 ----------------- GAS HOTSPOTS ----------------------\n");
            for h in &profile.hotspots {
                out.push_str(&format!(
                    "  🔥 Offset +{:<4} in `{}`: {:?} -> {} gas ({})\n",
                    h.offset, h.function_name, h.opcode, h.estimated_gas, h.reason
                ));
            }
        }

        out.push_str("\n========================================================\n");
        out
    }

    pub fn to_flamegraph_lines(profile: &WasmGasProfile) -> String {
        let mut out = String::new();
        for f in &profile.functions {
            out.push_str(&format!("contract;{};compute {}\n", f.name, f.estimated_cpu_gas));
            if f.storage_ops_count > 0 {
                out.push_str(&format!("contract;{};storage_ops {}\n", f.name, f.storage_ops_count * 1000));
            }
        }
        out
    }
}
