use crate::{
    KdlConfig, KdlConfigFinalize, Parsed, error::ParseDiagnostic, kdl_value_to_str,
    parse_helpers::get_single_argument_value,
};
use kdl::KdlNode;
use miette::NamedSource;

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
            Some(kdl::KdlValue::Integer(value)) => {
                let value = *value;
                if value >= 0 && value <= u32::MAX as i128 {
                    Parsed {
                        value: value as u32,
                        full_span: node.span(),
                        name_span: node.span(),
                        valid: true,
                    }
                } else {
                    diagnostics.push(
                        ParseDiagnostic::new(input, node.span())
                            .message("Expected type u32 but was out of range"),
                    );
                    Parsed {
                        value: 0,
                        full_span: node.span(),
                        name_span: node.span(),
                        valid: false,
                    }
                }
            }
            Some(value) => {
                diagnostics.push(ParseDiagnostic::new(input, node.span()).message(format!(
                    "Expected type Integer but was {}",
                    kdl_value_to_str(value)
                )));
                Parsed {
                    value: 0,
                    full_span: node.span(),
                    name_span: node.span(),
                    valid: false,
                }
            }
            None => Parsed {
                value: 0,
                full_span: node.span(),
                name_span: node.span(),
                valid: false,
            },
        }
    }
}

impl KdlConfigFinalize for u32 {
    type FinalizeType = u32;
    fn finalize(&self) -> Self::FinalizeType {
        *self
    }
}
