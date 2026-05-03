use crate::{
    KdlConfig, KdlConfigFinalize, Parsed, error::ParseDiagnostic, kdl_value_to_str,
    parse_helpers::get_single_argument_value,
};
use kdl::{KdlEntry, KdlNode, KdlValue};
use miette::{NamedSource, SourceSpan};

impl KdlConfig for u32 {
    fn parse_as_node(
        input: NamedSource<String>,
        node: &KdlNode,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> Parsed<Self>
    where
        Self: Sized,
    {
        match get_single_argument_value(input.clone(), node, diagnostics) {
            Some(value) => parse_int_value(value, input, node.span(), diagnostics),
            None => Parsed::invalid(node.span()),
        }
    }

    fn parse_as_argument(
        input: NamedSource<String>,
        entry: &KdlEntry,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> Parsed<Self> {
        if entry.name().is_some() {
            diagnostics.push(
                crate::error::ParseDiagnostic::new(input, entry.span())
                    .message("Named properties are not allowed here, only positional arguments"),
            );
            return Parsed::invalid(entry.span());
        }
        parse_int_value(entry.value(), input, entry.span(), diagnostics)
    }
}

fn parse_int_value(
    value: &KdlValue,
    input: NamedSource<String>,
    span: SourceSpan,
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> Parsed<u32> {
    match value {
        kdl::KdlValue::Integer(value) => {
            let value = *value;
            if value >= 0 && value <= u32::MAX as i128 {
                Parsed {
                    value: value as u32,
                    full_span: span,
                    name_span: span,
                    valid: true,
                }
            } else {
                diagnostics.push(
                    ParseDiagnostic::new(input, span)
                        .message("Expected type u32 but was out of range"),
                );
                Parsed::invalid(span)
            }
        }
        value => {
            diagnostics.push(ParseDiagnostic::new(input, span).message(format!(
                "Expected type Integer but was {}",
                kdl_value_to_str(value)
            )));
            Parsed::invalid(span)
        }
    }
}

impl KdlConfigFinalize for u32 {
    type FinalizeType = u32;
    fn finalize(&self) -> Self::FinalizeType {
        *self
    }
}
