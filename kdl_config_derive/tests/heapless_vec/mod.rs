use kdl::{KdlDocument, KdlNode};
use kdl_config::error::ParseDiagnostic;
use kdl_config::{KdlConfig, KdlConfigFinalize, Parsed};
use kdl_config_derive::{KdlConfig, KdlConfigFinalize};
use miette::NamedSource;

#[derive(Default, Debug, PartialEq, KdlConfig, KdlConfigFinalize)]
#[kdl_config_finalize_into = "ContainerFinal"]
struct Container {
    list: Parsed<heapless::Vec<Parsed<u32>, 10>>,
}

#[derive(Default, Debug, PartialEq)]
struct ContainerFinal {
    list: heapless::Vec<u32, 10>,
}

fn parse_doc<T: KdlConfig>(source: &str) -> (Parsed<T>, Vec<ParseDiagnostic>) {
    let doc: KdlDocument = source.parse().expect("test KDL is valid");
    let named = NamedSource::new("test.kdl", source.to_owned());
    let (parsed, err) = kdl_config::parse::<T>(named, doc);
    (parsed, err.diagnostics)
}

#[test]
fn happy_path() {
    let src = "\
list {
    - 1
    - 2
    - 3
}
";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
    assert_eq!(
        diagnostics
            .iter()
            .map(crate::diag_content)
            .collect::<Vec<_>>(),
        vec![]
    );
    let expected_list = {
        let mut v: heapless::Vec<Parsed<u32>, 10> = heapless::Vec::new();
        v.push(Parsed {
            value: 1,
            valid: true,
            ..Default::default()
        })
        .unwrap();
        v.push(Parsed {
            value: 2,
            valid: true,
            ..Default::default()
        })
        .unwrap();
        v.push(Parsed {
            value: 3,
            valid: true,
            ..Default::default()
        })
        .unwrap();
        v
    };
    assert_eq!(
        parsed,
        Parsed {
            value: Container {
                list: Parsed {
                    value: expected_list,
                    valid: true,
                    ..Default::default()
                },
            },
            valid: true,
            ..Default::default()
        }
    );
    let expected_final_list = {
        let mut v: heapless::Vec<u32, 10> = heapless::Vec::new();
        v.push(1).unwrap();
        v.push(2).unwrap();
        v.push(3).unwrap();
        v
    };
    assert_eq!(
        parsed.value.finalize(),
        ContainerFinal {
            list: expected_final_list,
        }
    );
}

#[test]
fn exceeds_capacity_produces_diagnostic() {
    let src = "\
list {
    - 1
    - 2
    - 3
    - 4
    - 5
    - 6
    - 7
    - 8
    - 9
    - 10
    - 11
}
";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
    assert_eq!(
        diagnostics
            .iter()
            .map(crate::diag_content)
            .collect::<Vec<_>>(),
        vec![crate::DiagContent {
            message: Some(
                "List exceeds maximum capacity of 10 items. Remove excess items.".to_owned()
            ),
            label: None,
            help: None,
            severity: miette::Severity::Error,
        }]
    );
    let expected_list = {
        let mut v: heapless::Vec<Parsed<u32>, 10> = heapless::Vec::new();
        for i in 1..=10 {
            v.push(Parsed {
                value: i,
                valid: true,
                ..Default::default()
            })
            .unwrap();
        }
        v
    };
    assert_eq!(
        parsed,
        Parsed {
            value: Container {
                list: Parsed {
                    value: expected_list,
                    valid: true,
                    ..Default::default()
                },
            },
            valid: true,
            ..Default::default()
        }
    );
}

#[test]
fn invalid_item_name_produces_diagnostic() {
    let src = "\
list {
    - 1
    foo 2
    - 3
}
";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
    assert_eq!(
        diagnostics
            .iter()
            .map(crate::diag_content)
            .collect::<Vec<_>>(),
        vec![crate::DiagContent {
            message: Some("List items must start with a \"-\"".to_owned()),
            label: None,
            help: Some(
                "Consider replacing the \"foo\" at the start of this section with a \"-\""
                    .to_owned()
            ),
            severity: miette::Severity::Error,
        }]
    );
    let expected_list = {
        let mut v: heapless::Vec<Parsed<u32>, 10> = heapless::Vec::new();
        v.push(Parsed {
            value: 1,
            valid: true,
            ..Default::default()
        })
        .unwrap();
        v.push(Parsed {
            value: 0,
            valid: false,
            ..Default::default()
        })
        .unwrap();
        v.push(Parsed {
            value: 3,
            valid: true,
            ..Default::default()
        })
        .unwrap();
        v
    };
    assert_eq!(
        parsed,
        Parsed {
            value: Container {
                list: Parsed {
                    value: expected_list,
                    valid: true,
                    ..Default::default()
                },
            },
            valid: true,
            ..Default::default()
        }
    );
}

#[test]
fn arguments_instead_of_children_produces_diagnostic() {
    let src = "\
list 1 2 3
";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
    assert_eq!(
        diagnostics
            .iter()
            .map(crate::diag_content)
            .collect::<Vec<_>>(),
        vec![crate::DiagContent {
            message: Some(
                "List node has arguments but expected child nodes prefixed with \"-\"".to_owned()
            ),
            label: None,
            help: None,
            severity: miette::Severity::Error,
        }]
    );
    assert_eq!(
        parsed,
        Parsed {
            value: Container {
                list: Parsed {
                    value: heapless::Vec::new(),
                    valid: false,
                    ..Default::default()
                },
            },
            valid: true,
            ..Default::default()
        }
    );
}

#[test]
fn item_wrong_type_produces_diagnostic() {
    let src = "\
list {
    - 1
    - \"not-an-integer\"
    - 3
}
";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
    assert_eq!(
        diagnostics
            .iter()
            .map(crate::diag_content)
            .collect::<Vec<_>>(),
        vec![crate::DiagContent {
            message: Some("Expected type Integer but was String".to_owned()),
            label: None,
            help: None,
            severity: miette::Severity::Error,
        }]
    );
    let expected_list = {
        let mut v: heapless::Vec<Parsed<u32>, 10> = heapless::Vec::new();
        v.push(Parsed {
            value: 1,
            valid: true,
            ..Default::default()
        })
        .unwrap();
        v.push(Parsed {
            value: 0,
            valid: false,
            ..Default::default()
        })
        .unwrap();
        v.push(Parsed {
            value: 3,
            valid: true,
            ..Default::default()
        })
        .unwrap();
        v
    };
    assert_eq!(
        parsed,
        Parsed {
            value: Container {
                list: Parsed {
                    value: expected_list,
                    valid: true,
                    ..Default::default()
                },
            },
            valid: true,
            ..Default::default()
        }
    );
}
