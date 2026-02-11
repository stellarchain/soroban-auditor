#![allow(dead_code)]

use crate::sdk::detector::SdkFunctionDetector;
use std::collections::HashMap;

/// SDK usage analyzer for contracts
pub struct SdkUsageAnalyzer {
    detector: SdkFunctionDetector,
}

impl SdkUsageAnalyzer {
    pub fn new() -> Self {
        Self {
            detector: SdkFunctionDetector::default(),
        }
    }

    /// Analyze SDK function usage and generate a report
    pub fn analyze(&self, sdk_calls: &HashMap<String, usize>) -> SdkUsageReport {
        let mut report = SdkUsageReport::default();

        for (fn_name, &count) in sdk_calls {
            if let Some(info) = self.detector.get_by_name(fn_name) {
                report.total_calls += count;
                report.unique_functions += 1;

                // Categorize by module
                *report
                    .calls_by_module
                    .entry(info.module_name.clone())
                    .or_insert(0) += count;

                // Categorize by high-level category
                if let Some(category) = self.detector.categorize_function(fn_name) {
                    let category_name = format!("{:?}", category);
                    *report.calls_by_category.entry(category_name).or_insert(0) += count;
                }

                // Track specific important functions
                match fn_name.as_str() {
                    "require_auth" | "require_auth_for_args" => {
                        report.auth_calls += count;
                    }
                    "get_contract_data" | "has_contract_data" => {
                        report.storage_reads += count;
                    }
                    "put_contract_data" | "del_contract_data" => {
                        report.storage_writes += count;
                    }
                    "contract_event" => {
                        report.event_emissions += count;
                    }
                    _ if fn_name.contains("vec_") => {
                        report.vec_operations += count;
                    }
                    _ if fn_name.contains("map_") => {
                        report.map_operations += count;
                    }
                    _ => {}
                }

                report.functions.push(FunctionUsage {
                    name: fn_name.clone(),
                    module: info.module_name.clone(),
                    count,
                    docs: info.docs.clone(),
                });
            }
        }

        // Sort functions by usage count
        report.functions.sort_by(|a, b| b.count.cmp(&a.count));

        report
    }

    /// Generate a human-readable report
    pub fn format_report(&self, report: &SdkUsageReport) -> String {
        let mut output = String::new();

        output.push_str("=== Soroban SDK Usage Report ===\n\n");

        output.push_str(&format!("Total SDK calls: {}\n", report.total_calls));
        output.push_str(&format!(
            "Unique SDK functions: {}\n",
            report.unique_functions
        ));
        output.push_str("\n");

        if !report.calls_by_module.is_empty() {
            output.push_str("Calls by Module:\n");
            let mut modules: Vec<_> = report.calls_by_module.iter().collect();
            modules.sort_by(|a, b| b.1.cmp(a.1));
            for (module, count) in modules {
                output.push_str(&format!("  {}: {} calls\n", module, count));
            }
            output.push_str("\n");
        }

        if !report.calls_by_category.is_empty() {
            output.push_str("Calls by Category:\n");
            let mut categories: Vec<_> = report.calls_by_category.iter().collect();
            categories.sort_by(|a, b| b.1.cmp(a.1));
            for (category, count) in categories {
                output.push_str(&format!("  {}: {} calls\n", category, count));
            }
            output.push_str("\n");
        }

        output.push_str("Key Metrics:\n");
        output.push_str(&format!("  Authentication checks: {}\n", report.auth_calls));
        output.push_str(&format!("  Storage reads: {}\n", report.storage_reads));
        output.push_str(&format!("  Storage writes: {}\n", report.storage_writes));
        output.push_str(&format!("  Event emissions: {}\n", report.event_emissions));
        output.push_str(&format!("  Vec operations: {}\n", report.vec_operations));
        output.push_str(&format!("  Map operations: {}\n", report.map_operations));
        output.push_str("\n");

        if !report.functions.is_empty() {
            output.push_str("Top 20 Most Used SDK Functions:\n");
            for func in report.functions.iter().take(20) {
                output.push_str(&format!(
                    "  {:40} ({:15}) - {} calls\n",
                    func.name, func.module, func.count
                ));
                if !func.docs.is_empty() && func.count > 5 {
                    output.push_str(&format!("    └─ {}\n", func.docs));
                }
            }
        }

        output
    }

    /// Generate a compact summary for embedding in generated code
    pub fn format_compact_summary(&self, report: &SdkUsageReport) -> String {
        let mut lines = Vec::new();

        lines.push(format!(
            "// SDK Usage: {} calls to {} unique functions",
            report.total_calls, report.unique_functions
        ));

        if report.auth_calls > 0 {
            lines.push(format!("//   - {} auth checks", report.auth_calls));
        }
        if report.storage_reads > 0 || report.storage_writes > 0 {
            lines.push(format!(
                "//   - {} storage reads, {} writes",
                report.storage_reads, report.storage_writes
            ));
        }
        if report.event_emissions > 0 {
            lines.push(format!("//   - {} events emitted", report.event_emissions));
        }

        lines.join("\n")
    }
}

impl Default for SdkUsageAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SdkUsageReport {
    pub total_calls: usize,
    pub unique_functions: usize,
    pub calls_by_module: HashMap<String, usize>,
    pub calls_by_category: HashMap<String, usize>,
    pub auth_calls: usize,
    pub storage_reads: usize,
    pub storage_writes: usize,
    pub event_emissions: usize,
    pub vec_operations: usize,
    pub map_operations: usize,
    pub functions: Vec<FunctionUsage>,
}

#[derive(Debug, Clone)]
pub struct FunctionUsage {
    pub name: String,
    pub module: String,
    pub count: usize,
    pub docs: String,
}
