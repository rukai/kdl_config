mod arguments;
mod enums;
mod heapless_vec;
mod kdl_value;
mod structs;

use kdl_config::error::ParseDiagnostic;

#[derive(Debug, PartialEq)]
pub struct DiagContent {
    pub message: Option<String>,
    pub label: Option<String>,
    pub help: Option<String>,
    pub severity: miette::Severity,
}

pub fn diag_content(d: &ParseDiagnostic) -> DiagContent {
    DiagContent {
        message: d.message.clone(),
        label: d.label.clone(),
        help: d.help.clone(),
        severity: d.severity,
    }
}
