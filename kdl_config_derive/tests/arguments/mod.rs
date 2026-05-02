use kdl::{KdlDocument, KdlNode};
use kdl_config::error::ParseDiagnostic;
use kdl_config::{KdlConfig, KdlConfigFinalize, KdlValue, Parsed};
use kdl_config_derive::{KdlConfig, KdlConfigFinalize};
use miette::NamedSource;

#[derive(Default, Debug, PartialEq, KdlConfig, KdlConfigFinalize)]
#[kdl_config_finalize_into = "ContainerFinal"]
struct Container {
    #[arguments]
    heapless_vec_of_values: Parsed<heapless::Vec<Parsed<KdlValue>, 10>>,
    #[arguments]
    heapless_vec_of_heapless_strings: Parsed<heapless::Vec<Parsed<heapless::String<50>>, 10>>,
    #[arguments]
    heapless_vec_of_ints: Parsed<heapless::Vec<Parsed<u32>, 10>>,
    #[arguments]
    arrayvec_of_values: Parsed<arrayvec::ArrayVec<Parsed<KdlValue>, 10>>,
    #[arguments]
    arrayvec_of_arraystring: Parsed<arrayvec::ArrayVec<Parsed<arrayvec::ArrayString<50>>, 10>>,
}

#[derive(Default, Debug, PartialEq)]
struct ContainerFinal {
    heapless_vec_of_values: heapless::Vec<KdlValue, 10>,
    heapless_vec_of_heapless_strings: heapless::Vec<heapless::String<50>, 10>,
    heapless_vec_of_ints: heapless::Vec<u32, 10>,
    arrayvec_of_values: arrayvec::ArrayVec<KdlValue, 10>,
    arrayvec_of_arraystring: arrayvec::ArrayVec<arrayvec::ArrayString<50>, 10>,
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
heapless-vec-of-values on-press : button-left + button-right -> set-profile 0
heapless-vec-of-heapless-strings hello world
heapless-vec-of-ints 6 7
arrayvec-of-values 42 #true
arrayvec-of-arraystring foo bar
";
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
                heapless_vec_of_values: Parsed {
                    value: [
                        KdlValue::String("on-press".to_owned()),
                        KdlValue::String(":".to_owned()),
                        KdlValue::String("button-left".to_owned()),
                        KdlValue::String("+".to_owned()),
                        KdlValue::String("button-right".to_owned()),
                        KdlValue::String("->".to_owned()),
                        KdlValue::String("set-profile".to_owned()),
                        KdlValue::Integer(0),
                    ]
                    .into_iter()
                    .map(|val| Parsed {
                        value: val,
                        valid: true,
                        ..Default::default()
                    })
                    .collect(),
                    valid: true,
                    ..Default::default()
                },
                heapless_vec_of_heapless_strings: Parsed {
                    value: [
                        heapless::String::try_from("hello").unwrap(),
                        heapless::String::try_from("world").unwrap(),
                    ]
                    .into_iter()
                    .map(|val| Parsed {
                        value: val,
                        valid: true,
                        ..Default::default()
                    })
                    .collect(),
                    valid: true,
                    ..Default::default()
                },
                heapless_vec_of_ints: Parsed {
                    value: [6, 7]
                        .into_iter()
                        .map(|val| Parsed {
                            value: val,
                            valid: true,
                            ..Default::default()
                        })
                        .collect(),
                    valid: true,
                    ..Default::default()
                },
                arrayvec_of_values: Parsed {
                    value: [KdlValue::Integer(42), KdlValue::Bool(true)]
                        .into_iter()
                        .map(|val| Parsed {
                            value: val,
                            valid: true,
                            ..Default::default()
                        })
                        .collect(),
                    valid: true,
                    ..Default::default()
                },
                arrayvec_of_arraystring: Parsed {
                    value: [
                        arrayvec::ArrayString::<50>::from("foo").unwrap(),
                        arrayvec::ArrayString::<50>::from("bar").unwrap(),
                    ]
                    .into_iter()
                    .map(|val| Parsed {
                        value: val,
                        valid: true,
                        ..Default::default()
                    })
                    .collect(),
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
            heapless_vec_of_values: [
                KdlValue::String("on-press".to_owned()),
                KdlValue::String(":".to_owned()),
                KdlValue::String("button-left".to_owned()),
                KdlValue::String("+".to_owned()),
                KdlValue::String("button-right".to_owned()),
                KdlValue::String("->".to_owned()),
                KdlValue::String("set-profile".to_owned()),
                KdlValue::Integer(0),
            ]
            .into_iter()
            .collect(),
            heapless_vec_of_heapless_strings: [
                heapless::String::try_from("hello").unwrap(),
                heapless::String::try_from("world").unwrap(),
            ]
            .into_iter()
            .collect(),
            heapless_vec_of_ints: [6, 7].into_iter().collect(),
            arrayvec_of_values: [KdlValue::Integer(42), KdlValue::Bool(true)]
                .into_iter()
                .collect(),
            arrayvec_of_arraystring: [
                arrayvec::ArrayString::<50>::from("foo").unwrap(),
                arrayvec::ArrayString::<50>::from("bar").unwrap(),
            ]
            .into_iter()
            .collect(),
        }
    );
}
