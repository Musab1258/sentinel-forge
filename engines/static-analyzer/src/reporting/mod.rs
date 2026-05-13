use clap::ValueEnum;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Text,
    Json,
    Sarif,
    Html,
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
            OutputFormat::Html => Ok(render_html(self)),
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

fn render_html(report: &AnalysisReport) -> String {
    let summary = &report.summary;
    let summary_cards = [
        ("Files scanned", summary.files.to_string(), "summary-neutral"),
        ("Functions indexed", summary.functions.to_string(), "summary-neutral"),
        ("Findings", summary.findings.to_string(), "summary-neutral"),
        ("Critical", summary.critical.to_string(), "summary-critical"),
        ("High", summary.high.to_string(), "summary-high"),
        ("Medium", summary.medium.to_string(), "summary-medium"),
        ("Low", summary.low.to_string(), "summary-low"),
        (
            "Informational",
            summary.informational.to_string(),
            "summary-info",
        ),
    ];

    let file_rows = report
        .scanned_files
        .iter()
        .map(|file| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                escape_html(&file.file),
                file.functions,
                file.findings
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let finding_cards = if report.findings.is_empty() {
        "<article class=\"finding-card empty\"><h2>No findings detected</h2><p>The analyzer did not identify issues in this scan.</p></article>".to_string()
    } else {
        report
            .findings
            .iter()
            .map(|finding| {
                let evidence = if finding.evidence.is_empty() {
                    String::new()
                } else {
                    format!(
                        "<div><h3>Evidence</h3><ul>{}</ul></div>",
                        finding
                            .evidence
                            .iter()
                            .map(|item| format!("<li>{}</li>", escape_html(item)))
                            .collect::<Vec<_>>()
                            .join("")
                    )
                };

                let trace = if finding.trace.is_empty() {
                    String::new()
                } else {
                    format!(
                        "<div><h3>Trace</h3><ol>{}</ol></div>",
                        finding
                            .trace
                            .iter()
                            .map(|step| format!("<li>{}</li>", escape_html(step)))
                            .collect::<Vec<_>>()
                            .join("")
                    )
                };

                format!(
                    "<article class=\"finding-card severity-{}\">\
                        <div class=\"finding-header\">\
                            <div>\
                                <p class=\"finding-meta\">{} · {} · {}</p>\
                                <h2>{}</h2>\
                            </div>\
                            <span class=\"severity-pill severity-{}\">{}</span>\
                        </div>\
                        <p class=\"finding-description\">{}</p>\
                        <dl class=\"finding-grid\">\
                            <div><dt>File</dt><dd>{}</dd></div>\
                            <div><dt>Function</dt><dd>{}</dd></div>\
                            <div><dt>Line</dt><dd>{}</dd></div>\
                            <div><dt>Detector</dt><dd>{}</dd></div>\
                            <div><dt>Confidence</dt><dd>{}</dd></div>\
                            <div><dt>Remediation</dt><dd>{}</dd></div>\
                        </dl>\
                        <div class=\"finding-details\">{}{}\
                        </div>\
                    </article>",
                    severity_slug(finding.severity),
                    escape_html(&finding.id),
                    escape_html(confidence_label(finding.confidence)),
                    escape_html(&finding.function),
                    escape_html(&finding.title),
                    severity_slug(finding.severity),
                    escape_html(severity_label(finding.severity)),
                    escape_html(&finding.description),
                    escape_html(&finding.file),
                    escape_html(&finding.function),
                    finding.line,
                    escape_html(&finding.detector),
                    escape_html(confidence_label(finding.confidence)),
                    escape_html(&finding.remediation),
                    evidence,
                    trace
                )
            })
            .collect::<Vec<_>>()
            .join("")
    };

    let summary_markup = summary_cards
        .iter()
        .map(|(label, value, class_name)| {
            format!(
                "<article class=\"summary-card {}\"><span>{}</span><strong>{}</strong></article>",
                class_name,
                escape_html(label),
                escape_html(value)
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        "<!DOCTYPE html>\
        <html lang=\"en\">\
        <head>\
            <meta charset=\"utf-8\" />\
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\
            <title>Sentinel Forge report :: {}</title>\
            <style>{}</style>\
        </head>\
        <body>\
            <main class=\"page-shell\">\
                <header class=\"hero-panel\">\
                    <p class=\"eyebrow\">Sentinel Forge static analyzer</p>\
                    <h1>Security report for {}</h1>\
                    <p class=\"hero-copy\">HTML reporting is the phase 4 bridge between the Rust engine and future dashboard workflows. This export keeps findings readable, shareable, and evidence-rich.</p>\
                </header>\
                <section class=\"summary-grid\">{}</section>\
                <section class=\"panel\">\
                    <div class=\"panel-heading\">\
                        <div>\
                            <p class=\"eyebrow\">Scan inventory</p>\
                            <h2>Files scanned</h2>\
                        </div>\
                    </div>\
                    <table>\
                        <thead>\
                            <tr><th>File</th><th>Functions</th><th>Findings</th></tr>\
                        </thead>\
                        <tbody>{}</tbody>\
                    </table>\
                </section>\
                <section class=\"panel\">\
                    <div class=\"panel-heading\">\
                        <div>\
                            <p class=\"eyebrow\">Findings</p>\
                            <h2>Evidence and remediation</h2>\
                        </div>\
                    </div>\
                    <div class=\"finding-list\">{}</div>\
                </section>\
            </main>\
        </body>\
        </html>",
        escape_html(&report.target),
        html_styles(),
        escape_html(&report.target),
        summary_markup,
        file_rows,
        finding_cards
    )
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

fn severity_slug(severity: Severity) -> &'static str {
    match severity {
        Severity::Critical => "critical",
        Severity::High => "high",
        Severity::Medium => "medium",
        Severity::Low => "low",
        Severity::Informational => "info",
    }
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn html_styles() -> &'static str {
    r#"
    :root {
        color-scheme: dark;
        --bg: #050816;
        --surface: #0b1120;
        --surface-strong: #111c31;
        --border: rgba(148, 163, 184, 0.18);
        --text: #e2e8f0;
        --muted: #94a3b8;
        --critical: #ef4444;
        --high: #f97316;
        --medium: #eab308;
        --low: #22c55e;
        --info: #64748b;
        --accent: #3b82f6;
    }

    * { box-sizing: border-box; }

    body {
        margin: 0;
        min-height: 100vh;
        background:
            radial-gradient(circle at top left, rgba(59, 130, 246, 0.16), transparent 28%),
            linear-gradient(180deg, #050816 0%, #08111f 54%, #050816 100%);
        color: var(--text);
        font-family: Inter, "Segoe UI", sans-serif;
    }

    .page-shell {
        margin: 0 auto;
        max-width: 1100px;
        padding: 40px 24px 72px;
    }

    .hero-panel,
    .panel,
    .summary-card,
    .finding-card {
        border: 1px solid var(--border);
        border-radius: 28px;
        background: linear-gradient(180deg, rgba(15, 23, 42, 0.88), rgba(8, 15, 29, 0.96));
        box-shadow: 0 18px 50px rgba(2, 8, 23, 0.32);
    }

    .hero-panel,
    .panel {
        padding: 28px;
    }

    .hero-panel h1,
    .panel h2,
    .finding-card h2 {
        margin: 10px 0 0;
        font-family: "Space Grotesk", Inter, sans-serif;
        line-height: 1.1;
    }

    .hero-panel h1 {
        font-size: clamp(2rem, 4vw, 3.4rem);
    }

    .hero-copy,
    .finding-description,
    .finding-grid dd,
    td,
    th {
        color: var(--muted);
        line-height: 1.7;
    }

    .eyebrow {
        color: #7dd3fc;
        font-size: 0.72rem;
        letter-spacing: 0.22em;
        margin: 0;
        text-transform: uppercase;
    }

    .summary-grid {
        display: grid;
        gap: 14px;
        grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
        margin: 20px 0 24px;
    }

    .summary-card {
        padding: 18px 20px;
    }

    .summary-card span {
        color: var(--muted);
        display: block;
        font-size: 0.82rem;
        letter-spacing: 0.08em;
        text-transform: uppercase;
    }

    .summary-card strong {
        display: block;
        font-family: "Space Grotesk", Inter, sans-serif;
        font-size: 1.9rem;
        margin-top: 8px;
    }

    .summary-critical strong { color: var(--critical); }
    .summary-high strong { color: var(--high); }
    .summary-medium strong { color: var(--medium); }
    .summary-low strong { color: var(--low); }
    .summary-info strong { color: #cbd5e1; }

    .panel + .panel {
        margin-top: 24px;
    }

    .panel-heading {
        align-items: end;
        display: flex;
        justify-content: space-between;
        gap: 16px;
        margin-bottom: 18px;
    }

    table {
        border-collapse: collapse;
        width: 100%;
    }

    th,
    td {
        border-top: 1px solid rgba(148, 163, 184, 0.12);
        padding: 14px 12px;
        text-align: left;
    }

    th {
        color: #cbd5e1;
        font-size: 0.78rem;
        letter-spacing: 0.12em;
        text-transform: uppercase;
    }

    .finding-list {
        display: grid;
        gap: 16px;
    }

    .finding-card {
        padding: 22px;
    }

    .finding-card.empty {
        text-align: center;
    }

    .finding-header {
        align-items: start;
        display: flex;
        gap: 16px;
        justify-content: space-between;
    }

    .finding-meta {
        color: var(--muted);
        font-size: 0.82rem;
        letter-spacing: 0.08em;
        margin: 0;
        text-transform: uppercase;
    }

    .severity-pill {
        border-radius: 999px;
        display: inline-flex;
        font-size: 0.78rem;
        font-weight: 700;
        letter-spacing: 0.14em;
        padding: 8px 12px;
        text-transform: uppercase;
    }

    .severity-critical { border-left: 4px solid var(--critical); }
    .severity-high { border-left: 4px solid var(--high); }
    .severity-medium { border-left: 4px solid var(--medium); }
    .severity-low { border-left: 4px solid var(--low); }
    .severity-info { border-left: 4px solid var(--info); }

    .severity-pill.severity-critical { background: rgba(239, 68, 68, 0.16); color: #fecaca; }
    .severity-pill.severity-high { background: rgba(249, 115, 22, 0.16); color: #fed7aa; }
    .severity-pill.severity-medium { background: rgba(234, 179, 8, 0.16); color: #fde68a; }
    .severity-pill.severity-low { background: rgba(34, 197, 94, 0.16); color: #bbf7d0; }
    .severity-pill.severity-info { background: rgba(100, 116, 139, 0.16); color: #cbd5e1; }

    .finding-grid {
        display: grid;
        gap: 14px;
        grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
        margin: 20px 0;
    }

    .finding-grid dt {
        color: #cbd5e1;
        font-size: 0.78rem;
        letter-spacing: 0.08em;
        text-transform: uppercase;
    }

    .finding-grid dd {
        margin: 8px 0 0;
    }

    .finding-details {
        display: grid;
        gap: 16px;
        grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    }

    .finding-details h3 {
        font-size: 0.82rem;
        letter-spacing: 0.08em;
        margin: 0 0 10px;
        text-transform: uppercase;
    }

    .finding-details ul,
    .finding-details ol {
        color: var(--muted);
        line-height: 1.7;
        margin: 0;
        padding-left: 18px;
    }

    @media (max-width: 720px) {
        .page-shell {
            padding: 24px 16px 56px;
        }

        .finding-header {
            flex-direction: column;
        }
    }
    "#
}

#[cfg(test)]
mod tests {
    use super::{
        AnalysisReport, Confidence, Finding, OutputFormat, ScannedFile, Severity,
    };

    fn sample_report() -> AnalysisReport {
        AnalysisReport::new(
            "fixtures".to_string(),
            vec![ScannedFile {
                file: "examples/vulnerable-contracts/missing_authorization.rs".to_string(),
                functions: 1,
                findings: 1,
            }],
            vec![Finding::new(
                "SF-001",
                "Missing authorization check",
                Severity::Critical,
                Confidence::High,
                "Public function mutates storage without authorization.",
                "Require authorization before privileged state writes.",
                "examples/vulnerable-contracts/missing_authorization.rs",
                "transfer_admin",
                4,
                "missing-authorization",
                vec!["line 4: storage::set_admin".to_string()],
                vec!["Function transfer_admin".to_string(), "WriteStorage".to_string()],
            )],
            1,
        )
    }

    #[test]
    fn renders_html_report() {
        let report = sample_report();
        let html = report
            .render(OutputFormat::Html)
            .expect("html render should succeed");

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Security report for fixtures"));
        assert!(html.contains("Missing authorization check"));
        assert!(html.contains("line 4: storage::set_admin"));
    }
}
