use crate::{
    KdlConfig, KdlConfigFinalize, Parsed, error::ParseDiagnostic, kdl_value_to_str,
    parse_helpers::get_single_argument_value,
};
use kdl::KdlNode;
use miette::NamedSource;

impl<T: KdlConfig + Default, const N: usize> KdlConfig for heapless::Vec<Parsed<T>, N> {
    fn parse_as_node(
        input: NamedSource<String>,
        node: &KdlNode,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> Parsed<Self>
    where
        Self: Sized,
    {
        let mut array = heapless::Vec::new();
        if !node.entries().is_empty() {
            diagnostics.push(ParseDiagnostic {
                input: input.clone(),
                span: node.span(),
                message: Some(
                    "List node has arguments but expected child nodes prefixed with \"-\""
                        .to_owned(),
                ),
                label: None,
                help: None,
                severity: miette::Severity::Error,
            });
            return Parsed {
                value: array,
                full_span: node.span(),
                name_span: node.span(),
                valid: false,
            };
        }
        if let Some(children) = node.children() {
            for node in children.nodes() {
                let name = node.name().value();
                if name == "-" {
                    let parsed = KdlConfig::parse_as_node(input.clone(), node, diagnostics);
                    if array.push(parsed).is_err() {
                        diagnostics.push(ParseDiagnostic {
                            input: input.clone(),
                            span: node.span(),
                            message: Some(format!(
                                "List exceeds maximum capacity of {N} items. Remove excess items."
                            )),
                            label: None,
                            help: None,
                            severity: miette::Severity::Error,
                        });
                    }
                } else {
                    let _ = array.push(Parsed {
                        value: Default::default(),
                        full_span: node.span(),
                        name_span: node.span(),
                        valid: false,
                    });
                    diagnostics.push(ParseDiagnostic {
                        input: input.clone(),
                        span: node.span(),
                        message: Some("List items must start with a \"-\"".to_owned()),
                        label: None,
                        help: Some(format!(
                            "Consider replacing the {name:?} at the start of this section with a \"-\""
                        )),
                        severity: miette::Severity::Error,
                    });
                }
            }
        }
        Parsed {
            value: array,
            full_span: node.span(),
            name_span: node.span(),
            valid: true,
        }
    }
}

impl<T: KdlConfigFinalize + Default, const N: usize> KdlConfigFinalize
    for heapless::Vec<Parsed<T>, N>
{
    type FinalizeType = heapless::Vec<T::FinalizeType, N>;
    fn finalize(&self) -> Self::FinalizeType {
        let mut array = heapless::Vec::new();
        for value in self {
            array.push(value.value.finalize()).ok();
        }
        array
    }
}

impl<const N: usize> KdlConfig for heapless::String<N> {
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
                match heapless::String::try_from(value.as_str()) {
                    Ok(s) => Parsed {
                        value: s,
                        full_span: node.span(),
                        name_span: node.span(),
                        valid: true,
                    },
                    Err(_) => {
                        let len = value.len();
                        diagnostics.push(ParseDiagnostic {
                            input,
                            span: node.span(),
                            message: Some(format!("Expected string with less than or equal to {N} characters but contained {len} characters. Try reducing the number of characters.")),
                            label: None,
                            help: None,
                            severity: miette::Severity::Error,
                        });
                        Parsed {
                            value: heapless::String::new(),
                            full_span: node.span(),
                            name_span: node.span(),
                            valid: false,
                        }
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
                    value: heapless::String::new(),
                    full_span: node.span(),
                    name_span: node.span(),
                    valid: false,
                }
            }
            None => Parsed {
                value: heapless::String::new(),
                full_span: node.span(),
                name_span: node.span(),
                valid: false,
            },
        }
    }
}

impl<const N: usize> KdlConfigFinalize for heapless::String<N> {
    type FinalizeType = heapless::String<N>;
    fn finalize(&self) -> Self::FinalizeType {
        self.clone()
    }
}
