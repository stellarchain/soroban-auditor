use crate::sdk::SdkFunctionDetector;
use crate::wasm_ir::Function;
use parity_wasm::elements::{CodeSection, Instruction, Module};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    fn as_str(self) -> &'static str {
        match self {
            Severity::Critical => "CRITICAL",
            Severity::High => "HIGH",
            Severity::Medium => "MEDIUM",
            Severity::Low => "LOW",
            Severity::Info => "INFO",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecurityFinding {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub confidence: &'static str,
    pub description: String,
    pub evidence: Vec<String>,
    pub recommendation: String,
}

#[derive(Debug, Clone, Default)]
pub struct SecurityMetrics {
    pub total_functions: usize,
    pub public_functions: usize,
    pub auth_calls: usize,
    pub storage_reads: usize,
    pub storage_writes: usize,
    pub event_emissions: usize,
    pub cross_contract_calls: usize,
    pub call_indirect_count: usize,
    pub unreachable_count: usize,
    pub loop_count: usize,
    pub prng_calls: usize,
}

#[derive(Debug, Clone)]
pub struct SecurityReport {
    pub metrics: SecurityMetrics,
    pub findings: Vec<SecurityFinding>,
}

#[derive(Debug, Clone, Default)]
struct FunctionSecuritySignal {
    name: String,
    is_public: bool,
    has_auth: bool,
    storage_writes: usize,
    cross_contract_calls: usize,
    call_indirect: usize,
    traps: usize,
    loops: usize,
}

pub struct SecurityAnalyzer {
    detector: SdkFunctionDetector,
}

impl SecurityAnalyzer {
    pub fn new() -> Self {
        Self {
            detector: SdkFunctionDetector::default(),
        }
    }

    pub fn analyze_module(
        &self,
        _module: &Module,
        functions: &[Function],
        import_count: usize,
        code: &CodeSection,
    ) -> SecurityReport {
        let mut metrics = SecurityMetrics::default();
        metrics.total_functions = code.bodies().len();

        let mut function_signals = Vec::new();
        for (i, body) in code.bodies().iter().enumerate() {
            let fn_index = import_count + i;
            let function = functions.get(fn_index);
            let mut signal = FunctionSecuritySignal {
                name: function
                    .map(|f| f.name.clone())
                    .unwrap_or_else(|| format!("func{}", fn_index)),
                is_public: function.map(|f| f.make_public).unwrap_or(false),
                ..FunctionSecuritySignal::default()
            };
            if signal.is_public {
                metrics.public_functions += 1;
            }

            for instr in body.code().elements() {
                match instr {
                    Instruction::Call(idx) => {
                        let idx = *idx as usize;
                        if idx >= import_count || idx >= functions.len() {
                            continue;
                        }
                        let call_name = &functions[idx].name;

                        if is_auth_function(call_name) {
                            metrics.auth_calls += 1;
                            signal.has_auth = true;
                        }
                        if is_storage_write_function(call_name) {
                            metrics.storage_writes += 1;
                            signal.storage_writes += 1;
                        }
                        if is_storage_read_function(call_name) {
                            metrics.storage_reads += 1;
                        }
                        if is_event_function(call_name) {
                            metrics.event_emissions += 1;
                        }
                        if is_prng_function(call_name) {
                            metrics.prng_calls += 1;
                        }
                        if is_cross_contract_call(call_name, &self.detector) {
                            metrics.cross_contract_calls += 1;
                            signal.cross_contract_calls += 1;
                        }
                    }
                    Instruction::CallIndirect(_, _) => {
                        metrics.call_indirect_count += 1;
                        signal.call_indirect += 1;
                    }
                    Instruction::Unreachable => {
                        metrics.unreachable_count += 1;
                        signal.traps += 1;
                    }
                    Instruction::Loop(_) => {
                        metrics.loop_count += 1;
                        signal.loops += 1;
                    }
                    _ => {}
                }
            }

            function_signals.push(signal);
        }

        let findings = self.analyze_signals(&metrics, &function_signals);
        SecurityReport { metrics, findings }
    }

    fn analyze_signals(
        &self,
        metrics: &SecurityMetrics,
        function_signals: &[FunctionSecuritySignal],
    ) -> Vec<SecurityFinding> {
        let mut findings = Vec::new();

        if metrics.storage_writes > 0 && metrics.auth_calls == 0 {
            findings.push(SecurityFinding {
                id: "missing-auth-on-state-mutation".to_string(),
                title: "State mutation detected without authorization checks".to_string(),
                severity: Severity::Critical,
                confidence: "medium",
                description: "Contract writes to storage but no `require_auth` style calls were found."
                    .to_string(),
                evidence: vec![format!("storage writes: {}", metrics.storage_writes)],
                recommendation: "Guard every state-changing public entrypoint with explicit auth checks."
                    .to_string(),
            });
        }

        if metrics.cross_contract_calls > 0 && metrics.auth_calls == 0 {
            findings.push(SecurityFinding {
                id: "cross-contract-without-auth".to_string(),
                title: "Cross-contract calls without visible auth guard".to_string(),
                severity: Severity::High,
                confidence: "low",
                description: "Contract performs external calls but no auth check pattern was detected."
                    .to_string(),
                evidence: vec![format!(
                    "cross-contract calls: {}",
                    metrics.cross_contract_calls
                )],
                recommendation:
                    "Validate caller permissions before outbound calls and enforce least-privilege auth."
                        .to_string(),
            });
        }

        if metrics.call_indirect_count > 0 {
            findings.push(SecurityFinding {
                id: "dynamic-dispatch-surface".to_string(),
                title: "Dynamic dispatch (`call_indirect`) expands attack surface".to_string(),
                severity: Severity::Medium,
                confidence: "high",
                description: "Indirect calls can enable unexpected execution paths if indices are not constrained."
                    .to_string(),
                evidence: vec![format!("call_indirect count: {}", metrics.call_indirect_count)],
                recommendation:
                    "Ensure call table indexes are fully controlled and validated before dispatch."
                        .to_string(),
            });
        }

        if metrics.unreachable_count >= 5 {
            findings.push(SecurityFinding {
                id: "high-trap-density".to_string(),
                title: "High trap density detected".to_string(),
                severity: Severity::Medium,
                confidence: "medium",
                description:
                    "Frequent trap sites may enable denial-of-service behavior under malformed inputs."
                        .to_string(),
                evidence: vec![format!("unreachable count: {}", metrics.unreachable_count)],
                recommendation:
                    "Replace trap-heavy paths with explicit error handling and bounded validation."
                        .to_string(),
            });
        } else if metrics.unreachable_count > 0 {
            findings.push(SecurityFinding {
                id: "trap-sites-present".to_string(),
                title: "Trap instructions present".to_string(),
                severity: Severity::Low,
                confidence: "high",
                description:
                    "At least one trap site (`unreachable`) exists; verify all are intentional and safe."
                        .to_string(),
                evidence: vec![format!("unreachable count: {}", metrics.unreachable_count)],
                recommendation:
                    "Audit trap branches and convert recoverable failures to explicit return errors."
                        .to_string(),
            });
        }

        if metrics.storage_writes > 0 && metrics.event_emissions == 0 {
            findings.push(SecurityFinding {
                id: "no-events-on-state-change".to_string(),
                title: "State changes without event emission".to_string(),
                severity: Severity::Low,
                confidence: "medium",
                description:
                    "Storage writes were found but no event emission was detected; this reduces auditability."
                        .to_string(),
                evidence: vec![
                    format!("storage writes: {}", metrics.storage_writes),
                    "event emissions: 0".to_string(),
                ],
                recommendation:
                    "Emit events for critical state transitions to improve monitoring and incident response."
                        .to_string(),
            });
        }

        if metrics.prng_calls > 0 {
            findings.push(SecurityFinding {
                id: "prng-usage-review".to_string(),
                title: "PRNG usage requires explicit threat-model review".to_string(),
                severity: Severity::Low,
                confidence: "medium",
                description:
                    "Pseudo-random generation calls are present. Ensure they are not used where adversarial predictability matters."
                        .to_string(),
                evidence: vec![format!("prng calls: {}", metrics.prng_calls)],
                recommendation:
                    "Document entropy assumptions and avoid PRNG-dependent security decisions when stronger guarantees are needed."
                        .to_string(),
            });
        }

        if metrics.loop_count > 0 {
            findings.push(SecurityFinding {
                id: "loop-budget-review".to_string(),
                title: "Loop usage should be budget-bounded".to_string(),
                severity: Severity::Info,
                confidence: "high",
                description:
                    "Loops were detected. Confirm loop bounds are input-safe to avoid budget exhaustion."
                        .to_string(),
                evidence: vec![format!("loop count: {}", metrics.loop_count)],
                recommendation:
                    "Enforce strict upper bounds for loop iterations based on trusted limits."
                        .to_string(),
            });
        }

        for signal in function_signals.iter().filter(|s| s.is_public) {
            if signal.storage_writes > 0 && !signal.has_auth {
                findings.push(SecurityFinding {
                    id: format!("public-mutation-no-auth-{}", signal.name),
                    title: format!(
                        "Public function `{}` mutates state without visible auth",
                        signal.name
                    ),
                    severity: Severity::High,
                    confidence: "medium",
                    description:
                        "A public entrypoint writes to storage, but no auth check pattern was detected in that function."
                            .to_string(),
                    evidence: vec![format!("storage writes in function: {}", signal.storage_writes)],
                    recommendation:
                        "Add explicit caller authorization checks in this public mutation path."
                            .to_string(),
                });
            }

            if signal.cross_contract_calls > 0 && !signal.has_auth {
                findings.push(SecurityFinding {
                    id: format!("public-cross-call-no-auth-{}", signal.name),
                    title: format!(
                        "Public function `{}` makes external calls without visible auth",
                        signal.name
                    ),
                    severity: Severity::High,
                    confidence: "low",
                    description:
                        "A public entrypoint performs cross-contract calls and no auth pattern was detected."
                            .to_string(),
                    evidence: vec![format!(
                        "cross-contract calls in function: {}",
                        signal.cross_contract_calls
                    )],
                    recommendation:
                        "Require authorization before performing privileged outbound calls."
                            .to_string(),
                });
            }
        }

        findings.sort_by_key(|f| f.severity);
        findings
    }

    pub fn format_report(&self, report: &SecurityReport, detailed: bool) -> String {
        let mut out = String::new();
        out.push_str("=== WASM Security Report ===\n\n");
        out.push_str(&format!("Functions analyzed: {}\n", report.metrics.total_functions));
        out.push_str(&format!(
            "Public functions: {}\n",
            report.metrics.public_functions
        ));
        out.push_str(&format!("Findings: {}\n\n", report.findings.len()));

        out.push_str("Signals:\n");
        out.push_str(&format!("  auth calls: {}\n", report.metrics.auth_calls));
        out.push_str(&format!(
            "  storage writes: {}\n",
            report.metrics.storage_writes
        ));
        out.push_str(&format!(
            "  storage reads: {}\n",
            report.metrics.storage_reads
        ));
        out.push_str(&format!(
            "  cross-contract calls: {}\n",
            report.metrics.cross_contract_calls
        ));
        out.push_str(&format!("  events: {}\n", report.metrics.event_emissions));
        out.push_str(&format!(
            "  call_indirect: {}\n",
            report.metrics.call_indirect_count
        ));
        out.push_str(&format!("  traps: {}\n", report.metrics.unreachable_count));
        out.push_str(&format!("  loops: {}\n", report.metrics.loop_count));
        out.push_str(&format!("  prng calls: {}\n\n", report.metrics.prng_calls));

        if report.findings.is_empty() {
            out.push_str("No security findings were detected by the heuristic checks.\n");
            return out;
        }

        let mut by_severity: BTreeMap<Severity, usize> = BTreeMap::new();
        for finding in &report.findings {
            *by_severity.entry(finding.severity).or_insert(0) += 1;
        }
        out.push_str("Summary by severity:\n");
        for severity in [
            Severity::Critical,
            Severity::High,
            Severity::Medium,
            Severity::Low,
            Severity::Info,
        ] {
            if let Some(count) = by_severity.get(&severity) {
                out.push_str(&format!("  {}: {}\n", severity.as_str(), count));
            }
        }
        out.push('\n');

        out.push_str("Findings:\n");
        for (idx, finding) in report.findings.iter().enumerate() {
            out.push_str(&format!(
                "  {}. [{}] {} ({}, confidence={})\n",
                idx + 1,
                finding.severity.as_str(),
                finding.title,
                finding.id,
                finding.confidence
            ));
            if detailed {
                out.push_str(&format!("     {}\n", finding.description));
                if !finding.evidence.is_empty() {
                    out.push_str("     evidence:\n");
                    for evidence in &finding.evidence {
                        out.push_str(&format!("       - {}\n", evidence));
                    }
                }
                out.push_str(&format!("     recommendation: {}\n", finding.recommendation));
            }
        }

        out
    }
}

impl Default for SecurityAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

fn is_auth_function(name: &str) -> bool {
    name == "require_auth" || name == "require_auth_for_args"
}

fn is_storage_write_function(name: &str) -> bool {
    matches!(
        name,
        "put_contract_data"
            | "del_contract_data"
            | "update_current_contract_wasm"
            | "update_asset_contract_wasm"
    )
}

fn is_storage_read_function(name: &str) -> bool {
    matches!(
        name,
        "get_contract_data"
            | "has_contract_data"
            | "get_asset_contract_data"
            | "has_asset_contract_data"
    )
}

fn is_event_function(name: &str) -> bool {
    name == "contract_event"
}

fn is_prng_function(name: &str) -> bool {
    name.starts_with("prng_")
}

fn is_cross_contract_call(name: &str, detector: &SdkFunctionDetector) -> bool {
    if let Some(info) = detector.get_by_name(name) {
        if info.module_name == "call" {
            return name == "call" || name == "try_call";
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_missing_auth_for_storage_writes() {
        let analyzer = SecurityAnalyzer::default();
        let metrics = SecurityMetrics {
            storage_writes: 3,
            ..SecurityMetrics::default()
        };
        let findings = analyzer.analyze_signals(&metrics, &[]);
        assert!(findings
            .iter()
            .any(|f| f.id == "missing-auth-on-state-mutation"));
    }

    #[test]
    fn flags_call_indirect_surface() {
        let analyzer = SecurityAnalyzer::default();
        let metrics = SecurityMetrics {
            call_indirect_count: 1,
            ..SecurityMetrics::default()
        };
        let findings = analyzer.analyze_signals(&metrics, &[]);
        assert!(findings.iter().any(|f| f.id == "dynamic-dispatch-surface"));
    }

    #[test]
    fn reports_no_findings_on_empty_metrics() {
        let analyzer = SecurityAnalyzer::default();
        let report = SecurityReport {
            metrics: SecurityMetrics::default(),
            findings: Vec::new(),
        };
        let output = analyzer.format_report(&report, false);
        assert!(output.contains("No security findings"));
    }
}
