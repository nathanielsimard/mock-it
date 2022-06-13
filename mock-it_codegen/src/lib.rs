extern crate proc_macro;

mod mock_fn;
mod trait_method;

use mock_fn::{mock_fns, MockFn};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Item};
use trait_method::{get_trait_method_types, TraitMethodType};

/// Generate a mock struct from a trait. The mock struct will be named after the
/// trait, with "Mock" appended.
#[proc_macro_attribute]
pub fn mock_it(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Parse the tokens
    let input: Item = parse_macro_input!(item as Item);

    // Make sure it's a trait
    let item_trait = match input {
        Item::Trait(item_trait) => item_trait,
        _ => panic!("Only traits can be mocked with the mock_it macro"),
    };

    let generics = item_trait.generics.clone();
    let generics_where = item_trait.generics.where_clause.clone();

    let trait_method_types = get_trait_method_types(&item_trait);
    let mock_fns = mock_fns(trait_method_types.clone());
    let helper_functions: Vec<TokenStream> = mock_fns
        .iter()
        .map(|mock_fn| mock_fn.helper_functions())
        .collect();

    // Create the mock identifier
    let trait_ident = &item_trait.ident;
    let mock_ident = Ident::new(&format!("{}Mock", trait_ident), trait_ident.span());

    // Generate the mock
    let fields = create_fields(&mock_fns);
    let field_init = create_field_init(&mock_fns);
    let trait_impls = create_trait_impls(&mock_fns);
    let clone_impl = create_clone_impl(&trait_method_types);

    let output = quote! {
        #item_trait

        pub struct #mock_ident #generics #generics_where {
            #(#fields),*
        }

        impl #generics #mock_ident #generics #generics_where {
            pub fn new() -> Self {
                #mock_ident {
                    #(#field_init),*
                }
            }

            #(#helper_functions)*

        }

        impl #generics std::clone::Clone for #mock_ident #generics #generics_where {
            fn clone(&self) -> Self {
                #mock_ident {
                    #(#clone_impl),*
                }
            }
        }

        impl #generics #trait_ident #generics for #mock_ident #generics #generics_where {
            #(#trait_impls)*
        }
    };

    output.into()
}

/// Create the struct fields
fn create_fields(mock_fns: &Vec<MockFn>) -> Vec<TokenStream> {
    mock_fns
        .iter()
        .map(|mock_fn| {
            let name = mock_fn.name();
            let return_input_types = mock_fn.return_input_types();
            let return_output_type = mock_fn.return_output_type();

            quote! {
                #name: mock_it::Mock<#return_input_types, #return_output_type>
            }
        })
        .collect()
}

/// Create the field initializers for the `new` method
fn create_field_init(mock_fns: &Vec<MockFn>) -> Vec<TokenStream> {
    mock_fns
        .iter()
        .map(|mock_fn| {
            let name = mock_fn.name();

            quote! {
                #name: mock_it::Mock::new()
            }
        })
        .collect()
}

/// Create the clone implementation
fn create_clone_impl(
    trait_method_types: &Vec<TraitMethodType>,
) -> impl Iterator<Item = TokenStream> + '_ {
    trait_method_types.iter().map(|method_type| {
        let ident = &method_type.signature.ident;
        quote! {
            #ident: self.#ident.clone()
        }
    })
}

/// Create the trait method implementations
fn create_trait_impls(mock_fns: &Vec<MockFn>) -> impl Iterator<Item = TokenStream> + '_ {
    mock_fns.iter().map(|mock_fn| {
        let called_fn_name = mock_fn.called_fn_name();
        let arg_names = mock_fn.args().into_iter().map(|arg| {
            let name = &arg.name;
            quote! {
                mock_it::Matcher::Val(#name)
            }
        });
        let signature = mock_fn.signature();

        quote! {
            #signature {
                self.#called_fn_name(#(#arg_names),*)
            }
        }
    })
}
