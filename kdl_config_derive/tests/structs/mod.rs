use kdl::{KdlDocument, KdlNode};
use kdl_config::error::ParseDiagnostic;
use kdl_config::{KdlConfig, KdlConfigFinalize, Parsed};
use kdl_config_derive::{KdlConfig, KdlConfigFinalize};
use miette::NamedSource;

#[derive(Default, Debug, PartialEq, KdlConfig, KdlConfigFinalize)]
#[kdl_config_finalize_into = "InnerInnerFinal"]
struct InnerInner {
    value: Parsed<u32>,
}

#[derive(Default, Debug, PartialEq)]
struct InnerInnerFinal {
    value: u32,
}

#[derive(Default, Debug, PartialEq, KdlConfig, KdlConfigFinalize)]
#[kdl_config_finalize_into = "InnerFinal"]
struct Inner {
    my_number: Parsed<u32>,
    other_number: Parsed<u32>,
    inner_inner: Parsed<InnerInner>,
}

#[derive(Default, Debug, PartialEq)]
struct InnerFinal {
    my_number: u32,
    other_number: u32,
    inner_inner: InnerInnerFinal,
}

#[derive(Default, Debug, PartialEq, KdlConfig, KdlConfigFinalize)]
#[kdl_config_finalize_into = "OuterFinal"]
struct Outer {
    inner: Parsed<Inner>,
    count: Parsed<u32>,
}

#[derive(Default, Debug, PartialEq)]
struct OuterFinal {
    inner: InnerFinal,
    count: u32,
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
inner {
    my-number 7
    other-number 13
    inner-inner {
        value 3
    }
}
count 99
";
    let (parsed, diagnostics) = parse_doc::<Outer>(src);
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
            value: Outer {
                inner: Parsed {
                    value: Inner {
                        my_number: Parsed {
                            value: 7,
                            valid: true,
                            ..Default::default()
                        },
                        other_number: Parsed {
                            value: 13,
                            valid: true,
                            ..Default::default()
                        },
                        inner_inner: Parsed {
                            value: InnerInner {
                                value: Parsed {
                                    value: 3,
                                    valid: true,
                                    ..Default::default()
                                },
                            },
                            valid: true,
                            ..Default::default()
                        },
                    },
                    valid: true,
                    ..Default::default()
                },
                count: Parsed {
                    value: 99,
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
        OuterFinal {
            inner: InnerFinal {
                my_number: 7,
                other_number: 13,
                inner_inner: InnerInnerFinal { value: 3 },
            },
            count: 99,
        }
    );
}

#[test]
fn missing_field_produces_diagnostic_and_is_invalid() {
    let src = "\
inner {
    my-number 7
    inner-inner {
        value 3
    }
}
count 1
";
    let (parsed, diagnostics) = parse_doc::<Outer>(src);
    assert_eq!(
        diagnostics
            .iter()
            .map(crate::diag_content)
            .collect::<Vec<_>>(),
        vec![crate::DiagContent {
            message: Some("Child other-number is missing from this node".to_owned()),
            label: None,
            help: None,
            severity: miette::Severity::Error,
        }]
    );
    assert_eq!(
        parsed,
        Parsed {
            value: Outer {
                inner: Parsed {
                    value: Inner::default(),
                    valid: false,
                    ..Default::default()
                },
                count: Parsed {
                    value: 1,
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
        OuterFinal {
            inner: InnerFinal {
                my_number: 0,
                other_number: 0,
                inner_inner: InnerInnerFinal { value: 0 },
            },
            count: 1,
        }
    );
}

#[test]
fn unknown_field_produces_diagnostic() {
    let src = "\
inner {
    my-number 7
    other-number 13
    inner-inner {
        value 3
    }
    bogus 1
}
count 1
";
    let (parsed, diagnostics) = parse_doc::<Outer>(src);
    assert_eq!(
        diagnostics
            .iter()
            .map(crate::diag_content)
            .collect::<Vec<_>>(),
        vec![crate::DiagContent {
            message: Some("Unknown node name".to_owned()),
            label: None,
            help: Some(
                "This node already has all the children it needs. Consider removing this section."
                    .to_owned(),
            ),
            severity: miette::Severity::Error,
        }]
    );
    assert_eq!(
        parsed,
        Parsed {
            value: Outer {
                inner: Parsed {
                    value: Inner {
                        my_number: Parsed {
                            value: 7,
                            valid: true,
                            ..Default::default()
                        },
                        other_number: Parsed {
                            value: 13,
                            valid: true,
                            ..Default::default()
                        },
                        inner_inner: Parsed {
                            value: InnerInner {
                                value: Parsed {
                                    value: 3,
                                    valid: true,
                                    ..Default::default()
                                },
                            },
                            valid: true,
                            ..Default::default()
                        },
                    },
                    valid: true,
                    ..Default::default()
                },
                count: Parsed {
                    value: 1,
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
        OuterFinal {
            inner: InnerFinal {
                my_number: 7,
                other_number: 13,
                inner_inner: InnerInnerFinal { value: 3 },
            },
            count: 1,
        }
    );
}

#[test]
fn field_wrong_type_produces_diagnostic() {
    let src = "\
inner {
    my-number \"not-an-integer\"
    other-number 13
    inner-inner {
        value 3
    }
}
count 1
";
    let (parsed, diagnostics) = parse_doc::<Outer>(src);
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
    assert_eq!(
        parsed,
        Parsed {
            value: Outer {
                inner: Parsed {
                    value: Inner {
                        my_number: Parsed {
                            value: 0,
                            valid: false,
                            ..Default::default()
                        },
                        other_number: Parsed {
                            value: 13,
                            valid: true,
                            ..Default::default()
                        },
                        inner_inner: Parsed {
                            value: InnerInner {
                                value: Parsed {
                                    value: 3,
                                    valid: true,
                                    ..Default::default()
                                },
                            },
                            valid: true,
                            ..Default::default()
                        },
                    },
                    valid: true,
                    ..Default::default()
                },
                count: Parsed {
                    value: 1,
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
        OuterFinal {
            inner: InnerFinal {
                my_number: 0,
                other_number: 13,
                inner_inner: InnerInnerFinal { value: 3 },
            },
            count: 1,
        }
    );
}
