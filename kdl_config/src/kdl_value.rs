use crate::{
    KdlConfig, KdlConfigFinalize, Parsed, error::ParseDiagnostic,
    parse_helpers::get_single_argument_value,
};
use kdl::{KdlEntry, KdlNode};
use miette::NamedSource;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum KdlValue {
    String(String),
    Integer(i128),
    Float(f64),
    Bool(bool),
    #[default]
    Null,
}

impl From<&kdl::KdlValue> for KdlValue {
    fn from(value: &kdl::KdlValue) -> Self {
        match value {
            kdl::KdlValue::String(s) => KdlValue::String(s.to_string()),
            kdl::KdlValue::Integer(i) => KdlValue::Integer(*i),
            kdl::KdlValue::Float(f) => KdlValue::Float(*f),
            kdl::KdlValue::Bool(b) => KdlValue::Bool(*b),
            kdl::KdlValue::Null => KdlValue::Null,
        }
    }
}

impl KdlConfig for KdlValue {
    fn parse_as_node(
        input: NamedSource<String>,
        node: &KdlNode,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> Parsed<Self>
    where
        Self: Sized,
    {
        match get_single_argument_value(input.clone(), node, diagnostics) {
            Some(value) => Parsed {
                value: KdlValue::from(value),
                full_span: node.span(),
                name_span: node.span(),
                valid: true,
            },
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
        Parsed {
            value: KdlValue::from(entry.value()),
            full_span: entry.span(),
            name_span: entry.span(),
            valid: true,
        }
    }
}

impl KdlConfigFinalize for KdlValue {
    type FinalizeType = KdlValue;
    fn finalize(&self) -> KdlValue {
        self.clone()
    }
}
