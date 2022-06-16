use crate::trait_method::{Argument, TraitMethodType};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Signature};

pub struct MockFn {
    method: TraitMethodType,
}

pub fn mock_fns(trait_method_types: Vec<TraitMethodType>) -> Vec<MockFn> {
    trait_method_types.into_iter().map(MockFn::new).collect()
}

impl MockFn {
    pub fn new(method: TraitMethodType) -> Self {
        Self { method }
    }

    pub fn is_async(&self) -> bool {
        if let Some(_) = self.method.signature.asyncness {
            return true;
        }

        false
    }

    pub fn name(&self) -> Ident {
        self.method.signature.ident.clone()
    }

    pub fn signature(&self) -> Signature {
        self.method.signature.clone()
    }

    pub fn return_input_types(&self) -> TokenStream {
        let output: Vec<TokenStream> = self
            .method
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

    pub fn helper_functions(&self) -> TokenStream {
        let when_fn = self.when_fn();
        let called_fn = self.called_fn();
        let was_called_with_fn = self.was_called_with_fn();

        let output = quote! {
            #when_fn
            #called_fn
            #was_called_with_fn
        };

        output.into()
    }

    pub fn return_output_type(&self) -> TokenStream {
        match &self.method.return_type {
            Some(ty) => quote! { #ty },
            None => quote! { () },
        }
    }

    pub fn when_fn_name(&self) -> Ident {
        let name = self.name();
        Ident::new(&format!("when_{}", name.to_string()), name.span())
    }

    pub fn called_fn_name(&self) -> Ident {
        let name = self.name();
        Ident::new(&format!("calling_{}", name.to_string()), name.span())
    }

    pub fn was_called_with_fn_name(&self) -> Ident {
        let name = self.name();
        Ident::new(&format!("expect_{}", name.to_string()), name.span())
    }

    pub fn args(&self) -> Vec<Argument> {
        self.method.args.clone()
    }

    fn when_fn(&self) -> TokenStream {
        let name = self.name();
        let fn_name = self.when_fn_name();
        let args = self.args_input_types();
        let args_input_mapping = self.args_input_mapping();
        let return_input_types = self.return_input_types();
        let return_input_names = self.return_input_names();
        let output_type = self.return_output_type();

        let quote = quote! {
            pub fn #fn_name(&self, #(#args),*) -> mock_it::When<#return_input_types, #output_type> {
                #(#args_input_mapping);*
                self.#name.when((#(#return_input_names),*))
            }
        };
        quote.into()
    }

    fn called_fn(&self) -> TokenStream {
        let name = self.name();
        let fn_name = self.called_fn_name();
        let args = self.args_input_types();
        let args_input_mapping = self.args_input_mapping();
        let return_input_names = self.return_input_names();
        let output_type = self.return_output_type();

        let quote = quote! {
            pub fn #fn_name(&self, #(#args),*) -> #output_type {
                #(#args_input_mapping);*
                self.#name.called((#(#return_input_names),*))
            }
        };
        quote.into()
    }

    fn was_called_with_fn(&self) -> TokenStream {
        let name = self.name();
        let fn_name = self.was_called_with_fn_name();
        let args = self.args_input_types();
        let args_input_mapping = self.args_input_mapping();
        let return_input_types = self.return_input_types();
        let return_input_names = self.return_input_names();

        let quote = quote! {
            pub fn #fn_name(&self, #(#args),*) -> mock_it::Validator<#return_input_types> {
                #(#args_input_mapping);*
                self.#name.was_called_with((#(#return_input_names),*))
            }
        };
        quote.into()
    }

    fn args_input_types(&self) -> Vec<TokenStream> {
        self.method
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

    fn args_input_mapping(&self) -> Vec<TokenStream> {
        self.method
            .args
            .iter()
            .map(|arg| {
                let name = &arg.name;
                if !arg.is_reference {
                    return quote! {};
                }
                quote! {
                    let #name  = match #name {
                        mock_it::Matcher::Val(val) => mock_it::Matcher::Val(std::sync::Arc::from(val.clone())),
                        mock_it::Matcher::Any => mock_it::Matcher::Any,
                    };
                }
            })
            .collect()
    }

    fn return_input_names(&self) -> Vec<TokenStream> {
        self.method
            .args
            .iter()
            .map(|arg| {
                let name = &arg.name;
                quote! { #name }
            })
            .collect()
    }
}
