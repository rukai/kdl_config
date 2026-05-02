use kdl::{KdlDocument, KdlNode};
use kdl_config::error::ParseDiagnostic;
use kdl_config::{KdlConfig, KdlConfigFinalize, KdlValue, Parsed};
use kdl_config_derive::{KdlConfig, KdlConfigFinalize};
use miette::NamedSource;

#[derive(Default, Debug, PartialEq, KdlConfig, KdlConfigFinalize)]
#[kdl_config_finalize_into = "ContainerFinal"]
struct Container {
    field: Parsed<KdlValue>,
}

#[derive(Default, Debug, PartialEq)]
struct ContainerFinal {
    field: KdlValue,
}

fn parse_doc<T: KdlConfig>(source: &str) -> (Parsed<T>, Vec<ParseDiagnostic>) {
    let doc: KdlDocument = source.parse().expect("test KDL is valid");
    let named = NamedSource::new("test.kdl", source.to_owned());
    let (parsed, err) = kdl_config::parse::<T>(named, doc);
    (parsed, err.diagnostics)
}

#[test]
fn string_value() {
    let src = "field \"hello\"\n";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
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
            value: Container {
                field: Parsed {
                    value: KdlValue::String("hello".to_owned()),
                    valid: true,
                    ..Default::default()
                },
            },
            valid: true,
            ..Default::default()
        }
    );
    assert_eq!(
        parsed.value.finalize(),
        ContainerFinal {
            field: KdlValue::String("hello".to_owned()),
        }
    );
}

#[test]
fn integer_value() {
    let src = "field 42\n";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
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
            value: Container {
                field: Parsed {
                    value: KdlValue::Integer(42),
                    valid: true,
                    ..Default::default()
                },
            },
            valid: true,
            ..Default::default()
        }
    );
    assert_eq!(
        parsed.value.finalize(),
        ContainerFinal {
            field: KdlValue::Integer(42),
        }
    );
}

#[test]
fn float_value() {
    let src = "field 3.9\n";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
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
            value: Container {
                field: Parsed {
                    value: KdlValue::Float(3.9),
                    valid: true,
                    ..Default::default()
                },
            },
            valid: true,
            ..Default::default()
        }
    );
    assert_eq!(
        parsed.value.finalize(),
        ContainerFinal {
            field: KdlValue::Float(3.9),
        }
    );
}

#[test]
fn bool_true_value() {
    let src = "field #true\n";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
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
            value: Container {
                field: Parsed {
                    value: KdlValue::Bool(true),
                    valid: true,
                    ..Default::default()
                },
            },
            valid: true,
            ..Default::default()
        }
    );
    assert_eq!(
        parsed.value.finalize(),
        ContainerFinal {
            field: KdlValue::Bool(true),
        }
    );
}

#[test]
fn bool_false_value() {
    let src = "field #false\n";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
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
            value: Container {
                field: Parsed {
                    value: KdlValue::Bool(false),
                    valid: true,
                    ..Default::default()
                },
            },
            valid: true,
            ..Default::default()
        }
    );
    assert_eq!(
        parsed.value.finalize(),
        ContainerFinal {
            field: KdlValue::Bool(false),
        }
    );
}

#[test]
fn null_value() {
    let src = "field #null\n";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
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
            value: Container {
                field: Parsed {
                    value: KdlValue::Null,
                    valid: true,
                    ..Default::default()
                },
            },
            valid: true,
            ..Default::default()
        }
    );
    assert_eq!(
        parsed.value.finalize(),
        ContainerFinal {
            field: KdlValue::Null,
        }
    );
}
