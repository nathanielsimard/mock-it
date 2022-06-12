use crate::trait_method::{Argument, TraitMethodType};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type};

pub struct MockFn {
    pub fn_name: Ident,
    pub struct_name: Ident,
    pub token_stream: TokenStream,
}

pub fn mock_fns(trait_method_types: &Vec<TraitMethodType>) -> Vec<MockFn> {
    trait_method_types.iter().map(mock_fn).collect()
}

fn mock_fn(method: &TraitMethodType) -> MockFn {
    let fn_name = &method.signature.ident;
    let struct_name = Ident::new(
        &format!("_MockFn{}", fn_name.to_string().as_str().replace("_", "M")), // Can't have underscore inside struct names.
        fn_name.span(),
    );

    let struct_def = struct_def(method, &struct_name);
    let new_fn = new_fn();
    let given_fn = given_fn(method);
    let called_fn = called_fn(method);
    let was_called_with = was_called_with(method);

    let struct_impl = quote! {
        impl #struct_name {
            #new_fn
            #given_fn
            #called_fn
            #was_called_with
        }
    };

    let output = quote! {
        #struct_def
        #struct_impl
    };

    MockFn {
        fn_name: fn_name.clone(),
        struct_name,
        token_stream: output.into(),
    }
}

fn new_fn() -> TokenStream {
    let output = quote! {
        pub fn new() -> Self {
            Self {
                mock: mock_it::Mock::new(),
            }
        }
    };
    output.into()
}

fn struct_def(method: &TraitMethodType, fn_struct_ident: &Ident) -> TokenStream {
    let fn_struct_field = fn_struct_field(&method.args, &method.return_type);
    let struct_def = quote! {
        #[derive(Clone)]
        pub struct #fn_struct_ident {
            mock: #fn_struct_field,
        }
    };

    struct_def.into()
}

fn given_fn(method: &TraitMethodType) -> TokenStream {
    let args = args_input_types(method);
    let args_input_mapping = args_input_mapping(method);
    let return_input_types = return_input_types(method);
    let return_input_names = return_input_names(method);
    let output_type = return_output_type(method);

    let quote = quote! {
        pub fn given(&self, #(#args),*) -> mock_it::Given<#return_input_types, #output_type> {
            #(#args_input_mapping);*
            self.mock.given((#(#return_input_names),*))
        }
    };
    quote.into()
}

fn called_fn(method: &TraitMethodType) -> TokenStream {
    let args = args_input_types(method);
    let args_input_mapping = args_input_mapping(method);
    let return_input_names = return_input_names(method);
    let output_type = return_output_type(method);

    let quote = quote! {
        pub fn called(&self, #(#args),*) -> #output_type {
            #(#args_input_mapping);*
            self.mock.called((#(#return_input_names),*))
        }
    };
    quote.into()
}

fn was_called_with(method: &TraitMethodType) -> TokenStream {
    let args = args_input_types(method);
    let args_input_mapping = args_input_mapping(method);
    let return_input_types = return_input_types(method);
    let return_input_names = return_input_names(method);

    let quote = quote! {
        pub fn was_called_with(&self, #(#args),*) -> mock_it::Validator<#return_input_types> {
            #(#args_input_mapping);*
            self.mock.was_called_with((#(#return_input_names),*))
        }
    };
    quote.into()
}

fn args_input_types(method: &TraitMethodType) -> Vec<TokenStream> {
    method
        .args
        .iter()
        .map(|arg| {
            let ty = &arg.original_type;
            let name = &arg.name;
            quote! {
                #name: mock_it::Matcher<#ty>
            }
        })
        .collect()
}

fn args_input_mapping(method: &TraitMethodType) -> Vec<TokenStream> {
    method
        .args
        .iter()
        .map(|arg| {
            let name = &arg.name;
            if !arg.is_reference {
                return quote! {};
            }
            quote! {
                let #name  = match #name {
                    mock_it::Matcher::Val(val) => mock_it::Matcher::Val(Arc::from(val.clone())),
                    mock_it::Matcher::Any => mock_it::Matcher::Any,
                };
            }
        })
        .collect()
}

fn return_input_names(method: &TraitMethodType) -> Vec<TokenStream> {
    method
        .args
        .iter()
        .map(|arg| {
            let name = &arg.name;
            quote! { #name }
        })
        .collect()
}
fn return_input_types(method: &TraitMethodType) -> TokenStream {
    let output: Vec<TokenStream> = method
        .args
        .iter()
        .map(|arg| {
            if !arg.is_reference {
                let ty = &arg.original_type;
                return quote! {
                    mock_it::Matcher<#ty>
                };
            }

            let ty = &arg.definition;
            quote! {
                #ty
            }
        })
        .collect();

    if output.len() == 0 {
        return quote! { () };
    }
    let output = quote! {
        (#(#output),*)
    };
    output.into()
}

fn return_output_type(method: &TraitMethodType) -> TokenStream {
    match &method.return_type {
        Some(ty) => quote! { #ty },
        None => quote! { () },
    }
}

fn fn_struct_field(args: &Vec<Argument>, return_type: &Option<Type>) -> TokenStream {
    let arg_types: Vec<TokenStream> = args.iter().map(|arg| arg.definition.clone()).collect();
    let return_tokens = match return_type {
        Some(return_type) => quote! { #return_type },
        None => quote! { () },
    };

    quote! {
        mock_it::Mock<(#(#arg_types),*), #return_tokens>
    }
}
