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
    let trait_method_types = get_trait_method_types(&item_trait);
    let mock_fns = mock_fns(&trait_method_types);

    // Create the mock identifier
    let trait_ident = &item_trait.ident;
    let mock_ident = Ident::new(&format!("{}Mock", trait_ident), trait_ident.span());

    // Generate the mock
    let fields = create_fields(&mock_fns);
    let field_init = create_field_init(&mock_fns);
    let trait_impls = create_trait_impls(&trait_method_types);
    let clone_impl = create_clone_impl(&trait_method_types);

    let mock_fn_stream = mock_fns.into_iter().map(|mock_fn| mock_fn.token_stream);

    let output = quote! {

        #(#mock_fn_stream)*

        #item_trait

        pub struct #mock_ident {
            #(#fields),*
        }

        impl #mock_ident {
            pub fn new() -> Self {
                #mock_ident {
                    #(#field_init),*
                }
            }

        }

        impl std::clone::Clone for #mock_ident {
            fn clone(&self) -> Self {
                #mock_ident {
                    #(#clone_impl),*
                }
            }
        }

        impl #trait_ident for #mock_ident {
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
            let mock_name = &mock_fn.struct_name;
            let field_name = &mock_fn.fn_name;

            quote! {
                pub #field_name: #mock_name
            }
        })
        .collect()
}

/// Create the field initializers for the `new` method
fn create_field_init(mock_fns: &Vec<MockFn>) -> Vec<TokenStream> {
    mock_fns
        .iter()
        .map(|mock_fn| {
            let mock_name = &mock_fn.struct_name;
            let field_name = &mock_fn.fn_name;

            quote! {
                #field_name: #mock_name::new()
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
fn create_trait_impls(
    trait_method_types: &Vec<TraitMethodType>,
) -> impl Iterator<Item = TokenStream> + '_ {
    trait_method_types.iter().map(|method_type| {
        let ident = &method_type.signature.ident;

        let arg_names: Vec<TokenStream> = method_type
            .args
            .iter()
            .map(|arg| {
                let ty = &arg.name;
                quote! {
                    mock_it::Matcher::Val(#ty)
                }
            })
            .collect();
        let signature = &method_type.signature;

        quote! {
            #signature {
                self.#ident.called(#(#arg_names),*)
            }
        }
    })
}
