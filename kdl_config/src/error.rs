use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Debug, Diagnostic, Clone, Eq, PartialEq, Error)]
#[error("Failed to parse configuration")]
pub struct ParseError {
    /// Original input that this failure came from.
    #[source_code]
    pub input: NamedSource<String>,

    /// Sub-diagnostics for this failure.
    #[related]
    pub diagnostics: Vec<ParseDiagnostic>,
}

/// An individual diagnostic message for a KDL parsing issue.
///
/// While generally signifying errors, they can also be treated as warnings.
#[derive(Debug, Diagnostic, Clone, Eq, PartialEq, Error)]
#[error("{}", message.clone().unwrap_or_else(|| "Unexpected error".into()))]
pub struct ParseDiagnostic {
    /// Shared source for the diagnostic.
    #[source_code]
    pub input: NamedSource<String>,

    /// Offset in chars of the error.
    #[label("{}", label.clone().unwrap_or_else(|| "here".into()))]
    pub span: SourceSpan,

    /// Message for the error itself.
    pub message: Option<String>,

    /// Label text for this span. Defaults to `"here"`.
    pub label: Option<String>,

    /// Suggestion for fixing the parser error.
    #[help]
    pub help: Option<String>,

    /// Severity level for the Diagnostic.
    #[diagnostic(severity)]
    pub severity: miette::Severity,
}

impl ParseDiagnostic {
    pub fn new(input: NamedSource<String>, span: SourceSpan) -> Self {
        Self {
            input,
            span,
            message: None,
            label: None,
            help: None,
            severity: miette::Severity::Error,
        }
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    pub fn severity(mut self, severity: miette::Severity) -> Self {
        self.severity = severity;
        self
    }
}
