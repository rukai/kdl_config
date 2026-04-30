use kdl::{KdlDocument, KdlNode};
use kdl_config::error::ParseDiagnostic;
use kdl_config::{KdlConfig, KdlConfigFinalize, Parsed};
use kdl_config_derive::{KdlConfig, KdlConfigFinalize};
use miette::NamedSource;

#[derive(Default, Debug, PartialEq, KdlConfig, KdlConfigFinalize)]
#[kdl_config_finalize_into = "ColorFinal"]
enum Color {
    #[default]
    RedApple,
    BlueSky,
    GreenGrass,
}

#[derive(Default, Debug, PartialEq)]
enum ColorFinal {
    #[default]
    RedApple,
    BlueSky,
    GreenGrass,
}

/// Used directly with a single child node, since enums expect a node with one argument (not a document fragment).
fn parse_node_directly<T: KdlConfig>(source: &str) -> (Parsed<T>, Vec<ParseDiagnostic>) {
    let doc: KdlDocument = source.parse().expect("test KDL is valid");
    let node: &KdlNode = doc.nodes().first().expect("at least one node");
    let named = NamedSource::new("test.kdl", source.to_owned());
    let mut diag = vec![];
    let parsed = T::parse_as_node(named, node, &mut diag);
    (parsed, diag)
}

#[test]
fn known_variant_parses() {
    let (parsed, diagnostics) = parse_node_directly::<Color>("color \"blue-sky\"");
    assert_eq!(
        diagnostics
            .iter()
            .map(crate::diag_content)
            .collect::<Vec<_>>(),
        vec![]
    );
    assert_eq!(
        parsed,
        Parsed {
            value: Color::BlueSky,
            valid: true,
            ..Default::default()
        }
    );
    assert_eq!(parsed.value.finalize(), ColorFinal::BlueSky);
}

#[test]
fn unknown_variant_produces_diagnostic() {
    let (parsed, diagnostics) = parse_node_directly::<Color>("color \"purple-thing\"");
    assert_eq!(
        diagnostics
            .iter()
            .map(crate::diag_content)
            .collect::<Vec<_>>(),
        vec![crate::DiagContent {
            message: Some("Unknown value purple-thing".to_owned()),
            label: None,
            help: Some(
                "Consider replacing it with one of [\"red-apple\", \"blue-sky\", \"green-grass\"]"
                    .to_owned(),
            ),
            severity: miette::Severity::Error,
        }]
    );
    assert_eq!(
        parsed,
        Parsed {
            value: Color::RedApple,
            valid: false,
            ..Default::default()
        }
    );
    assert_eq!(parsed.value.finalize(), ColorFinal::RedApple);
}

#[test]
fn wrong_argument_type_produces_diagnostic() {
    let (parsed, diagnostics) = parse_node_directly::<Color>("color 5");
    assert_eq!(
        diagnostics
            .iter()
            .map(crate::diag_content)
            .collect::<Vec<_>>(),
        vec![crate::DiagContent {
            message: Some("Expected type string but was TODO".to_owned()),
            label: None,
            help: None,
            severity: miette::Severity::Error,
        }]
    );
    assert_eq!(
        parsed,
        Parsed {
            value: Color::RedApple,
            valid: false,
            ..Default::default()
        }
    );
    assert_eq!(parsed.value.finalize(), ColorFinal::RedApple);
}
