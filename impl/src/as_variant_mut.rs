
use crate::utils::{AttrParams, DeriveType, State};
use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Fields, Result};

pub fn expand(input: &DeriveInput, trait_name: &'static str) -> Result<TokenStream> {
    let state = State::with_attr_params(
        input,
        trait_name,
        quote! {},
        "as_variant_mut".into(),
        AttrParams {
            enum_: vec!["ignore"],
            variant: vec!["ignore"],
            struct_: vec!["ignore"],
            field: vec!["ignore"],
        },
    )?;
    assert!(
        state.derive_type == DeriveType::Enum,
        "AsVariantMut can only be derived for enums",
    );

    let enum_name = &input.ident;
    let (imp_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let mut funcs = vec![];
    for variant_state in state.enabled_variant_data().variant_states {
        let variant = variant_state.variant.unwrap();
        let fn_name = format_ident!(
            "as_{}_mut",
            variant.ident.to_string().to_case(Case::Snake),
            span = variant.ident.span(),
        );
        let variant_ident = &variant.ident;

        let (data_pattern, return_pattern, return_type) = match &variant.fields {
            Fields::Named(_) => return Err(syn::parse::Error::new_spanned(&variant.ident, "AsVariantMut cannot be derived on enums with variants that have named fields")),
            Fields::Unnamed(fields) => {
                let field_idents = (0..fields.unnamed.len()).map(|x| format_ident!("_{x}"));

                let data_pattern = quote! { (#(#field_idents),*) };

                // handle singular field case separately to get return type without wrapping parentheses
                if fields.unnamed.len() == 1 {
                    let syn::Field { ty, .. } = fields.unnamed.first().unwrap();
                    (data_pattern.clone(), data_pattern, quote! { &mut #ty })
                } else {
                    let field_tys = fields.unnamed.iter().map(|field| &field.ty);
                    (data_pattern.clone(), data_pattern, quote! { (#(&mut #field_tys),*) })
                }
            },
            Fields::Unit => (quote! {}, quote! { () }, quote! { () }),
        };
        let variant_name = stringify!(variant_ident);
        let func = quote! {
            #[doc = "Returns `Some(#return_type)` if this value is of type `"]
            #[doc = #variant_name]
            #[doc = "`. Returns `None` otherwise"]
            pub fn #fn_name(&mut self) -> Option<#return_type> {
                match self {
                    #enum_name ::#variant_ident #data_pattern => Some(#return_pattern),
                    _ => None
                }
            }
        };
        funcs.push(func);
    }

    let imp = quote! {
        #[automatically_derived]
        impl #imp_generics #enum_name #type_generics #where_clause {
            #(#funcs)*
        }
    };

    Ok(imp)
}

