use crate::{
    KdlConfig, KdlConfigFinalize, Parsed, error::ParseDiagnostic, kdl_value_to_str,
    parse_helpers::get_single_argument_value,
};
use arrayvec::ArrayString;
use kdl::KdlNode;
use miette::NamedSource;

impl<const CAP: usize> KdlConfig for ArrayString<CAP> {
    fn parse_as_node(
        input: NamedSource<String>,
        node: &KdlNode,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> Parsed<Self>
    where
        Self: Sized,
    {
        match get_single_argument_value(input.clone(), node, diagnostics) {
            Some(kdl::KdlValue::String(value)) => {
                if let Ok(value) = ArrayString::from(value) {
                    Parsed {
                        value,
                        full_span: node.span(),
                        name_span: node.span(),
                        valid: true,
                    }
                } else {
                    let len = value.len();
                    diagnostics.push(ParseDiagnostic {
                        input,
                        span: node.span(),
                        message: Some(format!("Expected string with less than or equal to {CAP} characters but contained {len} characters. Try reducing the number of characters.")),
                        label: None,
                        help: None,
                        severity: miette::Severity::Error,
                    });
                    Parsed {
                        value: ArrayString::new(),
                        full_span: node.span(),
                        name_span: node.span(),
                        valid: false,
                    }
                }
            }
            Some(value) => {
                diagnostics.push(ParseDiagnostic {
                    input,
                    span: node.span(),
                    message: Some(format!(
                        "Expected type String but was {}",
                        kdl_value_to_str(value)
                    )),
                    label: None,
                    help: None,
                    severity: miette::Severity::Error,
                });
                Parsed {
                    value: ArrayString::new(),
                    full_span: node.span(),
                    name_span: node.span(),
                    valid: false,
                }
            }
            None => Parsed {
                value: ArrayString::new(),
                full_span: node.span(),
                name_span: node.span(),
                valid: false,
            },
        }
    }
}

impl<const CAP: usize> KdlConfigFinalize for ArrayString<CAP> {
    type FinalizeType = ArrayString<CAP>;
    fn finalize(&self) -> Self::FinalizeType {
        *self
    }
}
