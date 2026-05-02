use crate::{
    KdlConfig, KdlConfigFinalize, Parsed, error::ParseDiagnostic,
    parse_helpers::get_single_argument_value,
};
use kdl::KdlNode;
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
            None => Parsed {
                value: KdlValue::Null,
                full_span: node.span(),
                name_span: node.span(),
                valid: false,
            },
        }
    }
}

impl KdlConfigFinalize for KdlValue {
    type FinalizeType = KdlValue;
    fn finalize(&self) -> KdlValue {
        self.clone()
    }
}
