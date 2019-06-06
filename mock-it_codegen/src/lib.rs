extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, ArgCaptured, FnArg, Ident, Item, ItemTrait, Pat, ReturnType, TraitItem,
    TraitItemMethod, Type,
};

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

    // Create the mock identifier
    let trait_ident = &item_trait.ident;
    let mock_ident = Ident::new(&format!("{}Mock", trait_ident), trait_ident.span());

    // Generate the mock
    let fields = create_fields(&item_trait);
    let field_init = create_field_init(&item_trait);
    let trait_impls = create_trait_impls(&item_trait);
    let clone_impls = create_clone_impls(&item_trait);

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
                    #(#clone_impls),*
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
fn create_fields(item_trait: &ItemTrait) -> impl Iterator<Item = TokenStream> + '_ {
    get_mocks(item_trait).map(|(ident, mock)| {
        quote! {
            pub #ident: #mock
        }
    })
}

/// Create the field initializers for the `new` method
fn create_field_init(item_trait: &ItemTrait) -> impl Iterator<Item = TokenStream> + '_ {
    get_trait_methods(item_trait).map(|method| {
        let ident = method.sig.ident.clone();

        quote! {
            #ident: mock_it::Mock::new()
        }
    })
}

/// Create the clone implementation
fn create_clone_impls(item_trait: &ItemTrait) -> impl Iterator<Item = TokenStream> + '_ {
    get_trait_method_types(item_trait).map(|(ident, _, _)| {
        quote! {
            #ident: self.#ident.clone()
        }
    })
}


/// Create the trait method implementations
fn create_trait_impls(item_trait: &ItemTrait) -> impl Iterator<Item = TokenStream> + '_ {
    get_trait_methods(item_trait).map(|method| {
        let (ident, args, _) = get_method_types(method);
        let arg_names: Vec<Ident> = args
            .into_iter()
            .map(|arg| match arg.pat {
                Pat::Ident(ref inner) => inner.ident.clone(),
                _ => panic!("unknown argument pattern"),
            })
            .collect();
        let signature = &method.sig;

        quote! {
            #signature {
                self.#ident.called((#(#arg_names),*))
            }
        }
    })
}

/// Get the mock types for each method on the trait
fn get_mocks(item_trait: &ItemTrait) -> impl Iterator<Item = (Ident, TokenStream)> + '_ {
    get_trait_method_types(item_trait)
        .map(|(ident, args, return_type)| (ident, get_mock(args, return_type)))
}

/// Get the mock type for the arguments and return type combination
fn get_mock(args: Vec<&ArgCaptured>, return_type: Option<&Box<Type>>) -> TokenStream {
    let arg_types: Vec<&Type> = args.into_iter().map(|arg| &arg.ty).collect();
    let return_tokens = match return_type {
        Some(return_type) => quote! { #return_type },
        None => quote! { () },
    };

    quote! {
        mock_it::Mock<(#(#arg_types),*), #return_tokens>
    }
}

/// Get the identifier, arguments, and return type for each method in the trait
fn get_trait_method_types(
    item_trait: &ItemTrait,
) -> impl Iterator<Item = (Ident, Vec<&ArgCaptured>, Option<&Box<Type>>)> {
    get_trait_methods(item_trait).map(get_method_types)
}

/// Get the method's identifier, arguments, and return type
fn get_method_types(method: &TraitItemMethod) -> (Ident, Vec<&ArgCaptured>, Option<&Box<Type>>) {
    let ident = method.sig.ident.clone();
    let args: Vec<&ArgCaptured> = method
        .sig
        .decl
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Captured(inner) => Some(inner),
            _ => None,
        })
        .collect();
    let return_type = match method.sig.decl.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ref return_type) => Some(return_type),
    };

    (ident, args, return_type)
}

/// Get the methods for the given trait
fn get_trait_methods(item_trait: &ItemTrait) -> impl Iterator<Item = &TraitItemMethod> {
    item_trait.items.iter().filter_map(|item| {
        if let TraitItem::Method(item_method) = item {
            Some(item_method)
        } else {
            None
        }
    })
}
