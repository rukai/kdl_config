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
arguments on-press : button-left + button-right -> set-profile 0
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
                list_of_arguments: Parsed {
                    value: heapless::Vec::new(), // TODO: holds "on-press", ":", "button-left" etc.
                    valid: true,
                    ..Default::default()
                }
            },
            valid: true,
            ..Default::default()
        }
    );
    // assert_eq!(
    //     parsed.value.finalize(),
    //     todo!()
    // );
}
