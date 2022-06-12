extern crate proc_macro;

mod trait_method;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Item, Type};
use trait_method::{get_trait_method_types, Argument, TraitMethodType};

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

    // Create the mock identifier
    let trait_ident = &item_trait.ident;
    let mock_ident = Ident::new(&format!("{}Mock", trait_ident), trait_ident.span());

    // Generate the mock
    let fields = create_fields(&trait_method_types);
    let field_init = create_field_init(&trait_method_types);
    let trait_impls = create_trait_impls(&trait_method_types);
    let clone_impl = create_clone_impl(&trait_method_types);

    // Combine and tokenize the final result
    let output = quote! {
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

    // panic!("{}", output);
    output.into()
}

/// Create the struct fields
fn create_fields(
    trait_method_types: &Vec<TraitMethodType>,
) -> impl Iterator<Item = TokenStream> + '_ {
    get_mocks(trait_method_types).map(|(ident, mock)| {
        quote! {
            pub #ident: #mock
        }
    })
}

/// Create the field initializers for the `new` method
fn create_field_init(
    trait_method_types: &Vec<TraitMethodType>,
) -> impl Iterator<Item = TokenStream> + '_ {
    trait_method_types.iter().map(|method| {
        let ident = method.signature.ident.clone();

        quote! {
            #ident: mock_it::Mock::new()
        }
    })
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
                if arg.is_reference {
                    return quote! {
                        std::sync::Arc::from(#ty.clone())
                    };
                }
                quote! {
                    #ty
                }
            })
            .collect();
        let signature = &method_type.signature;

        quote! {
            #signature {
                self.#ident.called((#(#arg_names),*))
            }
        }
    })
}

/// Get the mock types for each method on the trait
fn get_mocks(
    trait_method_types: &Vec<TraitMethodType>,
) -> impl Iterator<Item = (Ident, TokenStream)> + '_ {
    trait_method_types.iter().map(|method_type| {
        (
            method_type.signature.ident.clone(),
            get_mock(&method_type.args, &method_type.return_type),
        )
    })
}

/// Get the mock type for the arguments and return type combination
fn get_mock(args: &Vec<Argument>, return_type: &Option<Type>) -> TokenStream {
    let arg_types: Vec<TokenStream> = args.iter().map(|arg| arg.definition.clone()).collect();
    let return_tokens = match return_type {
        Some(return_type) => quote! { #return_type },
        None => quote! { () },
    };

    quote! {
        mock_it::Mock<(#(#arg_types),*), #return_tokens>
    }
}
