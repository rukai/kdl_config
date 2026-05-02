use kdl::{KdlDocument, KdlNode};
use kdl_config::error::ParseDiagnostic;
use kdl_config::{KdlConfig, KdlConfigFinalize, KdlValue, Parsed};
use kdl_config_derive::{KdlConfig, KdlConfigFinalize};
use miette::NamedSource;

#[derive(Default, Debug, PartialEq, KdlConfig, KdlConfigFinalize)]
#[kdl_config_finalize_into = "ContainerFinal"]
struct Container {
    #[arguments]
    list_of_arguments: Parsed<heapless::Vec<Parsed<KdlValue>, 10>>,
}

#[derive(Default, Debug, PartialEq)]
struct ContainerFinal {
    list_of_arguments: heapless::Vec<KdlValue, 10>,
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
list-of-arguments on-press : button-left + button-right -> set-profile 0
";
    let (parsed, diagnostics) = parse_doc::<Container>(src);
    assert_eq!(
        diagnostics
            .iter()
            .map(crate::diag_content)
            .collect::<Vec<_>>(),
        vec![]
    );

    let mut expected_list: heapless::Vec<Parsed<KdlValue>, 10> = heapless::Vec::new();
    for val in [
        KdlValue::String("on-press".to_owned()),
        KdlValue::String(":".to_owned()),
        KdlValue::String("button-left".to_owned()),
        KdlValue::String("+".to_owned()),
        KdlValue::String("button-right".to_owned()),
        KdlValue::String("->".to_owned()),
        KdlValue::String("set-profile".to_owned()),
        KdlValue::Integer(0),
    ] {
        expected_list
            .push(Parsed {
                value: val,
                valid: true,
                ..Default::default()
            })
            .unwrap();
    }
    assert_eq!(
        parsed,
        Parsed {
            value: Container {
                list_of_arguments: Parsed {
                    value: expected_list,
                    valid: true,
                    ..Default::default()
                }
            },
            valid: true,
            ..Default::default()
        }
    );
    assert_eq!(
        parsed.value.finalize(),
        ContainerFinal {
            list_of_arguments: {
                let mut v = heapless::Vec::new();
                for val in [
                    KdlValue::String("on-press".to_owned()),
                    KdlValue::String(":".to_owned()),
                    KdlValue::String("button-left".to_owned()),
                    KdlValue::String("+".to_owned()),
                    KdlValue::String("button-right".to_owned()),
                    KdlValue::String("->".to_owned()),
                    KdlValue::String("set-profile".to_owned()),
                    KdlValue::Integer(0),
                ] {
                    v.push(val).unwrap();
                }
                v
            }
        }
    );
}
