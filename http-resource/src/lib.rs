extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use crate::quote::ToTokens;

type StaticStr = &'static str;

#[proc_macro_derive(HttpResource, attributes(result))]
pub fn resource_macro(input: TokenStream) -> TokenStream {
    match macro_impl(input) {
        Ok(output) => output,
        Err(err) => {
            let macro_error = format!("#[HttpResource] {}", err);
            let quoted_error = quote! {
                compile_error!(#macro_error);
            };
            quoted_error.into()
        }
    }
}

fn macro_impl(input: TokenStream) -> Result<TokenStream, String> {
    let input = syn::parse::<syn::Item>(input).map_err(|err| format!("Syntax error: {}", err) )?;

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
