use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{DataEnum, DeriveInput, FieldsNamed, FieldsUnnamed, Ident};

pub fn generate(input: DeriveInput) -> Result<TokenStream2, syn::Error> {
    let ident = input.ident;
    match input.data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => {
                let rust_field_names: Vec<&syn::Ident> =
                    named.iter().map(|x| x.ident.as_ref().unwrap()).collect();
                let kdl_field_names = rust_field_names
                    .iter()
                    .map(|x| stringcase::kebab_case(&x.to_string()));
                Ok(quote! {
                    impl KdlConfig for #ident {
                        // arguments are prefixed with a random guid to ensure they dont collide with user field names.
                        // Its silly but I'm not aware of a better solution
                        fn parse_as_node(c068528d5bea4f73bf39204d30e57322_input: NamedSource<String>, c068528d5bea4f73bf39204d30e57322_node: &KdlNode, c068528d5bea4f73bf39204d30e57322_diag: &mut Vec<kdl_config::error::ParseDiagnostic>) -> Parsed<#ident> {
                            if let [
                                #(Some(#rust_field_names),)*
                            ] = kdl_config::parse_helpers::get_children(
                                c068528d5bea4f73bf39204d30e57322_input.clone(),
                                c068528d5bea4f73bf39204d30e57322_node,
                                [ #(#kdl_field_names,)* ],
                                c068528d5bea4f73bf39204d30e57322_diag,
                            ) {
                                return Parsed {
                                    value: #ident {
                                        #(#rust_field_names: KdlConfig::parse_as_node(c068528d5bea4f73bf39204d30e57322_input.clone(), #rust_field_names, c068528d5bea4f73bf39204d30e57322_diag),)*
                                    },
                                    full_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                    name_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                    valid: true,
                                }
                            }
                            else {
                                return Parsed {
                                    value: Default::default(),
                                    full_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                    name_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                    valid: false,
                                }
                            }
                        }
                    }
                })
            }
            syn::Fields::Unnamed(FieldsUnnamed { .. }) => Err(syn::Error::new(
                s.struct_token.span,
                "`KdlConfig` cannot be derived for unnamed structs",
            )),
            syn::Fields::Unit => Err(syn::Error::new(
                s.struct_token.span,
                "`KdlConfig` cannot be derived for unit structs",
            )),
        },
        syn::Data::Enum(DataEnum { variants, .. }) => {
            let variant_idents: Vec<&Ident> = variants.iter().map(|v| &v.ident).collect();
            let kdl_names: Vec<String> = variants
                .iter()
                .map(|v| {
                    // TODO: just rewrite this ourselves
                    stringcase::kebab_case(&v.ident.to_string())
                })
                .collect();
            Ok(quote! {
                impl KdlConfig for #ident {
                    fn parse_as_node(c068528d5bea4f73bf39204d30e57322_input: NamedSource<String>, c068528d5bea4f73bf39204d30e57322_node: &KdlNode, c068528d5bea4f73bf39204d30e57322_diagnostics: &mut Vec<kdl_config::error::ParseDiagnostic>) -> Parsed<#ident> {
                        use kdl::KdlValue;
                        use kdl_config::parse_helpers::get_single_argument_value;
                        let kdl_names = [#(#kdl_names,)*];
                        match get_single_argument_value(c068528d5bea4f73bf39204d30e57322_input.clone(), c068528d5bea4f73bf39204d30e57322_node, c068528d5bea4f73bf39204d30e57322_diagnostics) {
                            Some(KdlValue::String(string)) => match string.as_str() {
                                #(
                                    #kdl_names => Parsed {
                                        value: #ident::#variant_idents,
                                        full_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                        name_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                        valid: false,
                                    },
                                )*
                                name => {
                                    c068528d5bea4f73bf39204d30e57322_diagnostics.push(kdl_config::error::ParseDiagnostic {
                                        input: c068528d5bea4f73bf39204d30e57322_input.clone(),
                                        span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                        message: Some(format!(
                                            "Unknown value {name}"
                                        )),
                                        label: None,
                                        help: Some(format!("Consider replacing it with one of {kdl_names:?}")),
                                        severity: miette::Severity::Error,
                                    });
                                    Parsed {
                                        value: Default::default(),
                                        full_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                        name_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                        valid: false,
                                    }
                                }
                            },
                            Some(value) => {
                                c068528d5bea4f73bf39204d30e57322_diagnostics.push(kdl_config::error::ParseDiagnostic {
                                    input: c068528d5bea4f73bf39204d30e57322_input.clone(),
                                    span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                    message: Some(format!(
                                        "Expected type string but was {}", "TODO"
                                    )),
                                    label: None,
                                    help: None,
                                    severity: miette::Severity::Error,
                                });
                                Parsed {
                                    value: Default::default(),
                                    full_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                    name_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                    valid: false,
                                }
                            }
                            None => Parsed {
                                value: Default::default(),
                                full_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                name_span: c068528d5bea4f73bf39204d30e57322_node.span(),
                                valid: false,
                            }
                        }
                    }
                }
            })
        }
        syn::Data::Union(data) => Err(syn::Error::new(
            data.union_token.span,
            "`KdlConfig` cannot be derived for unions",
        )),
    }
}
