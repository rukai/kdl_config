use crate::{
    KdlConfig, KdlConfigFinalize, Parsed, error::ParseDiagnostic, kdl_value_to_str,
    parse_helpers::get_single_argument_value,
};
use kdl::{KdlEntry, KdlNode};
use miette::{NamedSource, SourceSpan};

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
            diagnostics.push(
                ParseDiagnostic::new(input.clone(), node.span()).message(
                    "List node has arguments but expected child nodes prefixed with \"-\"",
                ),
            );
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
                        diagnostics.push(ParseDiagnostic::new(input.clone(), node.span()).message(
                            format!(
                                "List exceeds maximum capacity of {N} items. Remove excess items."
                            ),
                        ));
                    }
                } else {
                    let _ = array.push(Parsed::invalid(node.span()));
                    diagnostics.push(
                        ParseDiagnostic::new(input.clone(), node.span())
                            .message("List items must start with a \"-\"")
                            .help(format!(
                                "Consider replacing the {name:?} at the start of this section with a \"-\""
                            )),
                    );
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

    fn parse_as_arguments(
        input: NamedSource<String>,
        node: &KdlNode,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> Parsed<Self> {
        let mut array = heapless::Vec::new();
        for entry in node.entries() {
            let parsed = T::parse_as_argument(input.clone(), entry, diagnostics);
            if array.push(parsed).is_err() {
                diagnostics.push(ParseDiagnostic::new(input.clone(), entry.span()).message(
                    format!("List exceeds maximum capacity of {N} items. Remove excess items."),
                ));
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
            Some(value) => parse_heapless_string_value(value, input, node.span(), diagnostics),
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
                ParseDiagnostic::new(input, entry.span())
                    .message("Named properties are not allowed here, only positional arguments"),
            );
            return Parsed::invalid(entry.span());
        }
        parse_heapless_string_value(entry.value(), input, entry.span(), diagnostics)
    }
}

fn parse_heapless_string_value<const N: usize>(
    value: &kdl::KdlValue,
    input: NamedSource<String>,
    span: SourceSpan,
    diagnostics: &mut Vec<ParseDiagnostic>,
) -> Parsed<heapless::String<N>> {
    match value {
        kdl::KdlValue::String(value) => match heapless::String::try_from(value.as_str()) {
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
                        "Expected string with less than or equal to {N} characters but contained {len} characters. Try reducing the number of characters."
                    )),
                );
                Parsed::invalid(span)
            }
        },
        value => {
            diagnostics.push(ParseDiagnostic::new(input, span).message(format!(
                "Expected type String but was {}",
                kdl_value_to_str(value)
            )));
            Parsed::invalid(span)
        }
    }
}

impl<const N: usize> KdlConfigFinalize for heapless::String<N> {
    type FinalizeType = heapless::String<N>;
    fn finalize(&self) -> Self::FinalizeType {
        self.clone()
    }
}
