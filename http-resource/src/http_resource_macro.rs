use proc_macro2::TokenStream as TokenStream2;
use crate::quote::ToTokens;

type StaticStr = &'static str;

pub(crate) fn macro_impl(input: TokenStream2) -> Result<TokenStream2, String> {
    let input = syn::parse2::<syn::Item>(input).map_err(|err| format!("Syntax error: {}", err) )?;

    // 1. Parse struct definition
    let (struct_type, struct_fields) = unwrap_struct(input)?;
    // 2. Parse struct fields
    let named_struct_fields = unwrap_fields(struct_fields)?;

    // 3. Make sure there's an `error` field
    let (_error_field, error_type) = find_field_with_name(&named_struct_fields, "error")?;
    // 4. It has to be Option<T>
    unwrap_option(&error_type).map_err(|err| format!("`error` field must be Option<String>, got: {}", err) )?;

    // 5. Make sure there's an `ok` field
    let (_ok_field, ok_type) = find_field_with_name(&named_struct_fields, "ok")?;
    if ok_type.to_token_stream().to_string() != "bool" {
        return Err("`ok` field must have type `bool`".to_owned());
    }

    // 6. Find `result: <>` struct field
    let (result_field, result_type) = find_field_with_attr(&named_struct_fields, "result")?;

    // 7. Unwrap `Option<Result>` into `Result`
    let result_type = unwrap_option(&result_type).map_err(|err| format!("`result` field must be Option<String>, got: {} ({})", result_type.to_token_stream().to_string(), err))?;

    // 8. Build final code
    let derived = quote!{
        impl HttpResponse<#result_type> for #struct_type {
            fn to_result(&self) -> crate::SlackResult<#result_type> {
                if self.ok {
                    if let Some(result) = &self.#result_field {
                        return Ok(result.clone());
                    } else {
                        return Err(crate::SlackError::from(format!("'ok' is true, but '{}' is null", stringify!(#result_field))));
                    }
                }
                if let Some(err) = &self.error {
                    return Err(crate::SlackError::from(err));
                }
                Err(crate::SlackError::from("Broken response format (no 'error' field)"))
            }
        }
    };

    // println!("{}", derived.to_token_stream().to_string());

    Ok(derived.into())
}

fn unwrap_struct(item: syn::Item) -> Result<(syn::Ident, syn::Fields), StaticStr> {
    match &item {
        syn::Item::Struct(syn::ItemStruct { ident, fields, .. }) => {
            Ok((ident.clone(), fields.clone()))
        },
        _ => Err("macro can only be applied to struct")
    }
}

fn unwrap_fields(fields: syn::Fields) -> Result<Vec<syn::Field>, StaticStr> {
    match fields {
        syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            Ok(named.iter().map(|item|item.clone()).collect())
        },
        _ => Err("struct has no named fields")
    }
}

fn find_field_with_name(fields: &Vec<syn::Field>, name: &str) -> Result<( syn::Ident, syn::Type ), String> {
    for field in fields {
        if let Some(ident) = &field.ident {
            if ident.to_string() == name {
                return Ok(( field.ident.clone().unwrap(), field.ty.clone() ))
            }
        }
    }
    Err(format!("macro must be used on a struct that has `{}` field", name))
}

fn find_field_with_attr(fields: &Vec<syn::Field>, attribute: &str) -> Result<( syn::Ident, syn::Type ), String> {
    for field in fields {
        for attr in &field.attrs {
            if attr.path.is_ident(attribute) {
                return Ok(( field.ident.clone().unwrap(), field.ty.clone() ))
            }
        }
    }
    Err(format!("macro must have a field with #[{}] attribute", attribute))
}

fn unwrap_option(tt: &syn::Type) -> Result<syn::Type, StaticStr> {
    // `tt` maybe is generic, maybe absolute, maybe not, we don't know at this point
    let path = match tt {
        syn::Type::Path(syn::TypePath { path, .. }) => path,
        _ => return Err("not a type")
    };

    if path.segments.is_empty() {
        return Err("multiple path segments");
    }
    // `tt` is a local type
    let segment = &path.segments[0];

    if segment.ident.to_string() != "Option" {
        return Err("type, but not an Option");
    }
    // `tt` is Option

    let args = match &segment.arguments {
        syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }) => args,
        _ => return Err("Option, but not generic")
    };
    // `tt` is `Option` and generic

    if args.len() != 1 {
        return Err("Option, generic, but with multiple arguments")
    }
    // `tt` is `Option<T>`

    // return T
    match &args[0] {
        syn::GenericArgument::Type(tt) => Ok(tt.clone()),
        _ => Err("Option, generic, with 1 argument, but it's not a type")
    }
}

#[cfg(test)]
mod tests {
    use super::macro_impl;
    use proc_macro2::TokenStream as TokenStream2;

    macro_rules! assert_derives {
        ($input: expr, $expected: expr) => {
            let input = $input.parse::<TokenStream2>().unwrap();
            let expected = $expected.parse::<TokenStream2>().unwrap();

            let output = macro_impl(input).unwrap();

            assert_eq!(
                output.to_string(),
                expected.to_string()
            )
        }
    }

    macro_rules! assert_errors {
        ($input: expr, $err: expr) => {
            let input = $input.parse::<TokenStream2>().unwrap();
            let output = macro_impl(input);

            if let Err(err) = output {
                assert_eq!(err, String::from($err));
            } else {
                panic!("expected to get Err, got Ok");
            }
        };
    }

    #[test]
    fn it_rejects_non_structs() {
        assert_errors!(
            r#"
                enum E {
                    A,
                    B
                }
            "#,
            "macro can only be applied to struct"
        );
    }

    #[test]
    fn it_rejects_struct_without_error_field() {
        assert_errors!(
            r#"
                struct E {
                }
            "#,
            "macro must be used on a struct that has `error` field"
        );
    }

    #[test]
    fn it_rejects_struct_with_error_field_that_is_not_an_option() {
        assert_errors!(
            r#"
                struct E {
                    error: i32
                }
            "#,
            "`error` field must be Option<String>, got: type, but not an Option"
        );
    }

    #[test]
    fn it_rejects_struct_with_error_field_that_is_option_but_not_generic() {
        assert_errors!(
            r#"
                struct E {
                    error: Option
                }
            "#,
            "`error` field must be Option<String>, got: Option, but not generic"
        );
    }

    #[test]
    fn it_rejects_struct_with_error_field_that_is_option_but_with_multiple_arguments() {
        assert_errors!(
            r#"
                struct E {
                    error: Option<A, B>
                }
            "#,
            "`error` field must be Option<String>, got: Option, generic, but with multiple arguments"
        );
    }

    #[test]
    fn it_rejects_struct_without_ok_field() {
        assert_errors!(
            r#"
                struct E {
                    error: Option<i32>
                }
            "#,
            "macro must be used on a struct that has `ok` field"
        );
    }

    #[test]
    fn it_rejects_struct_without_ok_field_that_is_not_bool() {
        assert_errors!(
            r#"
                struct E {
                    error: Option<i32>,
                    ok: i32
                }
            "#,
            "`ok` field must have type `bool`"
        );
    }

    #[test]
    fn it_rejects_struct_without_result_attribute() {
        assert_errors!(
            r#"
                struct E {
                    error: Option<i32>,
                    ok: bool,
                }
            "#,
            "macro must have a field with #[result] attribute"
        );
    }

    #[test]
    fn it_rejects_struct_when_result_field_is_not_option() {
        assert_errors!(
            r#"
                struct E {
                    error: Option<i32>,
                    ok: bool,
                    #[result]
                    data_field: i32
                }
            "#,
            "`result` field must be Option<String>, got: i32 (type, but not an Option)"
        );
    }

    #[test]
    fn it_rejects_struct_when_result_field_that_is_option_but_not_generic() {
        assert_errors!(
            r#"
                struct E {
                    error: Option<i32>,
                    ok: bool,
                    #[result]
                    data_field: Option
                }
            "#,
            "`result` field must be Option<String>, got: Option (Option, but not generic)"
        );
    }


    #[test]
    fn it_rejects_struct_when_result_field_that_is_option_but_with_multiple_arguments() {
        assert_errors!(
            r#"
                struct E {
                    error: Option<i32>,
                    ok: bool,
                    #[result]
                    data_field: Option<A, B>
                }
            "#,
            "`result` field must be Option<String>, got: Option < A , B > (Option, generic, but with multiple arguments)"
        );
    }


    #[test]
    fn it_appends_http_response_implementation_if_struct_definition_is_valid() {
        assert_derives!(
            r#"
            struct Foo {
                ok: bool,
                error: Option<String>,
                #[result]
                data_is_here: Option<Vec<i32>>
            }
            "#,

            r#"
            impl HttpResponse<Vec<i32> > for Foo {
                fn to_result(&self) -> crate::SlackResult<Vec<i32> > {
                    if self.ok {
                        if let Some(result) = &self.data_is_here {
                            return Ok(result.clone());
                        } else {
                            return Err(crate::SlackError::from(format!("'ok' is true, but '{}' is null", stringify!( data_is_here ))));
                        }
                    }

                    if let Some(err) = &self.error {
                        return Err(crate::SlackError::from(err));
                    }
                    Err(crate::SlackError::from("Broken response format (no 'error' field)"))
                }
            }
            "#
        );
    }


}
