use crate::{KdlConfig, KdlConfigFinalize, Parsed, error::ParseDiagnostic};
use arrayvec::ArrayVec;
use kdl::KdlNode;
use miette::NamedSource;

impl<T: KdlConfig + Default, const CAP: usize> KdlConfig for ArrayVec<Parsed<T>, CAP> {
    fn parse_as_node(
        input: NamedSource<String>,
        node: &KdlNode,
        diagnostics: &mut Vec<ParseDiagnostic>,
    ) -> Parsed<Self>
    where
        Self: Sized,
    {
        let mut array = ArrayVec::new();
        if let Some(children) = node.children() {
            for node in children.nodes() {
                let name = node.name().value();
                if node.name().value() == "-" {
                    array.push(KdlConfig::parse_as_node(input.clone(), node, diagnostics))
                } else {
                    array.push(Parsed {
                        value: Default::default(),
                        full_span: node.span(),
                        name_span: node.span(),
                        valid: false,
                    });
                    diagnostics.push(
                        ParseDiagnostic::new(input.clone(), node.span())
                            .message("List items must start with a \"-\"")
                            .help(format!("Consider replacing the {name:?} at the start of this section with a \"-\"")),
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
        let mut array = ArrayVec::new();
        for entry in node.entries() {
            let parsed = T::parse_as_argument(input.clone(), entry, diagnostics);
            if array.try_push(parsed).is_err() {
                diagnostics.push(ParseDiagnostic::new(input.clone(), entry.span()).message(
                    format!("List exceeds maximum capacity of {CAP} items. Remove excess items."),
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

impl<T: KdlConfigFinalize + Default, const CAP: usize> KdlConfigFinalize
    for ArrayVec<Parsed<T>, CAP>
{
    type FinalizeType = ArrayVec<T::FinalizeType, CAP>;
    fn finalize(&self) -> Self::FinalizeType {
        let mut array = ArrayVec::new();
        for value in self {
            array.push(value.value.finalize());
        }
        array
    }
}
