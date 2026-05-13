use clap::ValueEnum;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Text,
    Json,
    Sarif,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Confidence {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CodeSpan {
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub confidence: Confidence,
    pub description: String,
    pub remediation: String,
    pub file: String,
    pub function: String,
    pub line: usize,
    pub detector: String,
    pub evidence: Vec<String>,
    pub trace: Vec<String>,
    pub code_span: CodeSpan,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScannedFile {
    pub file: String,
    pub functions: usize,
    pub findings: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnalysisSummary {
    pub files: usize,
    pub functions: usize,
    pub findings: usize,
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub informational: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnalysisReport {
    pub analyzer: String,
    pub target: String,
    pub summary: AnalysisSummary,
    pub scanned_files: Vec<ScannedFile>,
    pub findings: Vec<Finding>,
}

impl AnalysisReport {
    pub fn new(
        target: String,
        scanned_files: Vec<ScannedFile>,
        findings: Vec<Finding>,
        total_functions: usize,
    ) -> Self {
        let summary = AnalysisSummary {
            files: scanned_files.len(),
            functions: total_functions,
            findings: findings.len(),
            critical: findings
                .iter()
                .filter(|finding| finding.severity == Severity::Critical)
                .count(),
            high: findings
                .iter()
                .filter(|finding| finding.severity == Severity::High)
                .count(),
            medium: findings
                .iter()
                .filter(|finding| finding.severity == Severity::Medium)
                .count(),
            low: findings
                .iter()
                .filter(|finding| finding.severity == Severity::Low)
                .count(),
            informational: findings
                .iter()
                .filter(|finding| finding.severity == Severity::Informational)
                .count(),
        };

        Self {
            analyzer: crate::ANALYZER_NAME.to_string(),
            target,
            summary,
            scanned_files,
            findings,
        }
    }

    pub fn sort_findings(&mut self) {
        self.findings.sort_by(|left, right| {
            severity_rank(left.severity)
                .cmp(&severity_rank(right.severity))
                .then_with(|| left.file.cmp(&right.file))
                .then_with(|| left.line.cmp(&right.line))
                .then_with(|| left.id.cmp(&right.id))
        });
    }

    pub fn render(&self, format: OutputFormat) -> anyhow::Result<String> {
        match format {
            OutputFormat::Text => Ok(render_text(self)),
            OutputFormat::Json => Ok(serde_json::to_string_pretty(self)?),
            OutputFormat::Sarif => Ok(serde_json::to_string_pretty(&render_sarif(self))?),
        }
    }
}

impl Finding {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        severity: Severity,
        confidence: Confidence,
        description: impl Into<String>,
        remediation: impl Into<String>,
        file: impl Into<String>,
        function: impl Into<String>,
        line: usize,
        detector: impl Into<String>,
        evidence: Vec<String>,
        trace: Vec<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            severity,
            confidence,
            description: description.into(),
            remediation: remediation.into(),
            file: file.into(),
            function: function.into(),
            line,
            detector: detector.into(),
            evidence,
            trace,
            code_span: CodeSpan {
                start_line: line,
                end_line: line,
            },
        }
    }
}

fn render_text(report: &AnalysisReport) -> String {
    let mut lines = vec![
        format!("{} :: {}", report.analyzer, report.target),
        format!(
            "files={} functions={} findings={} critical={} high={} medium={} low={} info={}",
            report.summary.files,
            report.summary.functions,
            report.summary.findings,
            report.summary.critical,
            report.summary.high,
            report.summary.medium,
            report.summary.low,
            report.summary.informational
        ),
    ];

    for finding in &report.findings {
        lines.push(String::new());
        lines.push(format!("[{}] {} ({})", severity_label(finding.severity), finding.title, finding.id));
        lines.push(format!(
            "file: {}:{} :: {}",
            finding.file, finding.line, finding.function
        ));
        lines.push(format!("confidence: {}", confidence_label(finding.confidence)));
        lines.push(format!("detector: {}", finding.detector));
        lines.push(format!("description: {}", finding.description));
        lines.push(format!("remediation: {}", finding.remediation));

        if !finding.evidence.is_empty() {
            lines.push("evidence:".to_string());
            for item in &finding.evidence {
                lines.push(format!("- {item}"));
            }
        }

        if !finding.trace.is_empty() {
            lines.push(format!("trace: {}", finding.trace.join(" -> ")));
        }
    }

    if report.findings.is_empty() {
        lines.push(String::new());
        lines.push("No findings detected.".to_string());
    }

    lines.join("\n")
}

fn render_sarif(report: &AnalysisReport) -> Value {
    let rules: Vec<Value> = report
        .findings
        .iter()
        .map(|finding| {
            json!({
                "id": finding.id,
                "name": finding.title,
                "shortDescription": { "text": finding.title },
                "fullDescription": { "text": finding.description },
                "help": { "text": finding.remediation },
                "properties": {
                    "security-severity": severity_label(finding.severity).to_ascii_lowercase(),
                    "confidence": confidence_label(finding.confidence).to_ascii_lowercase(),
                }
            })
        })
        .collect();

    let results: Vec<Value> = report
        .findings
        .iter()
        .map(|finding| {
            json!({
                "ruleId": finding.id,
                "level": sarif_level(finding.severity),
                "message": { "text": finding.title },
                "locations": [{
                    "physicalLocation": {
                        "artifactLocation": { "uri": finding.file },
                        "region": {
                            "startLine": finding.code_span.start_line,
                            "endLine": finding.code_span.end_line,
                        }
                    }
                }],
                "properties": {
                    "confidence": confidence_label(finding.confidence).to_ascii_lowercase(),
                    "function": finding.function,
                    "evidence": finding.evidence,
                    "trace": finding.trace,
                }
            })
        })
        .collect();

    json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": report.analyzer,
                    "informationUri": "https://github.com/sentinel-forge/sentinel-forge",
                    "rules": rules,
                }
            },
            "results": results,
        }]
    })
}

fn severity_rank(severity: Severity) -> usize {
    match severity {
        Severity::Critical => 0,
        Severity::High => 1,
        Severity::Medium => 2,
        Severity::Low => 3,
        Severity::Informational => 4,
    }
}

fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Critical => "CRITICAL",
        Severity::High => "HIGH",
        Severity::Medium => "MEDIUM",
        Severity::Low => "LOW",
        Severity::Informational => "INFO",
    }
}

fn confidence_label(confidence: Confidence) -> &'static str {
    match confidence {
        Confidence::High => "high",
        Confidence::Medium => "medium",
        Confidence::Low => "low",
    }
}

fn sarif_level(severity: Severity) -> &'static str {
    match severity {
        Severity::Critical | Severity::High => "error",
        Severity::Medium => "warning",
        Severity::Low | Severity::Informational => "note",
    }
}
