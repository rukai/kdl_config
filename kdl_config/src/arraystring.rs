use crate::{
    KdlConfig, KdlConfigFinalize, Parsed, error::ParseDiagnostic, kdl_value_to_str,
    parse_helpers::get_single_argument_value,
};
use arrayvec::ArrayString;
use kdl::{KdlEntry, KdlNode};
use miette::{NamedSource, SourceSpan};

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
            Some(value) => parse_arraystring_value(value, input, node.span(), diagnostics),
            None => Parsed {
                value: ArrayString::new(),
                full_span: node.span(),
                name_span: node.span(),
                valid: false,
            },
        }
    }

    fn parse_as_argument(
        input: NamedSource<String>,
        entry: &KdlEntry,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> Parsed<Self> {
        if entry.name().is_some() {
            diagnostics.push(
                ParseDiagnostic::new(input, entry.span())
                    .message("Named properties are not allowed here, only positional arguments"),
            );
            return Parsed {
                value: ArrayString::new(),
                full_span: entry.span(),
                name_span: entry.span(),
                valid: false,
            };
        }
        parse_arraystring_value(entry.value(), input, entry.span(), diagnostics)
    }
}

fn parse_arraystring_value<const CAP: usize>(
    value: &kdl::KdlValue,
    input: NamedSource<String>,
    span: SourceSpan,
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> Parsed<ArrayString<CAP>> {
    match value {
        kdl::KdlValue::String(value) => match ArrayString::from(value.as_str()) {
            Ok(s) => Parsed {
                value: s,
                full_span: span,
                name_span: span,
                valid: true,
            },
            Err(_) => {
                let len = value.len();
                diagnostics.push(
                    ParseDiagnostic::new(input, span).message(format!(
                        "Expected string with less than or equal to {CAP} characters but contained {len} characters. Try reducing the number of characters."
                    )),
                );
                Parsed {
                    value: ArrayString::new(),
                    full_span: span,
                    name_span: span,
                    valid: false,
                }
            }
        },
        value => {
            diagnostics.push(ParseDiagnostic::new(input, span).message(format!(
                "Expected type String but was {}",
                kdl_value_to_str(value)
            )));
            Parsed {
                value: ArrayString::new(),
                full_span: span,
                name_span: span,
                valid: false,
            }
        }
    }
}

impl<const CAP: usize> KdlConfigFinalize for ArrayString<CAP> {
    type FinalizeType = ArrayString<CAP>;
    fn finalize(&self) -> Self::FinalizeType {
        *self
    }
}
