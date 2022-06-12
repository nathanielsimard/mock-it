use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    FnArg, Ident, ItemTrait, Pat, PatType, ReturnType, Signature, TraitItem, TraitItemMethod, Type,
};

pub struct TraitMethodType {
    pub args: Vec<Argument>,
    pub return_type: Option<Type>,
    pub signature: Signature,
}

pub struct Argument {
    pub is_reference: bool,
    pub name: Ident,
    pub definition: TokenStream,
    pub original_type: Type,
}

pub fn get_trait_method_types(item_trait: &ItemTrait) -> Vec<TraitMethodType> {
    get_trait_methods(item_trait)
        .map(get_method_types)
        .collect()
}

fn get_trait_methods(item_trait: &ItemTrait) -> impl Iterator<Item = &TraitItemMethod> {
    item_trait.items.iter().filter_map(|item| {
        if let TraitItem::Method(item_method) = item {
            Some(item_method)
        } else {
            None
        }
    })
}

fn get_method_types(method: &TraitItemMethod) -> TraitMethodType {
    let args: Vec<Argument> = method
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(inner) => Some(inner.clone()),
            _ => None,
        })
        .map(|arg| {
            let original_type = arg.ty.clone();
            if let Type::Reference(reference) = *original_type.clone() {
                let ty = reference.elem.clone();
                let definition = quote! {
                    mock_it::Matcher<std::sync::Arc<#ty>>
                };
                let name = get_pat_type_name(&arg);
                return Argument {
                    is_reference: true,
                    definition,
                    name,
                    original_type: *original_type.clone(),
                };
            } else {
                let definition = quote! {
                    mock_it::Matcher<#original_type>
                };
                let name = get_pat_type_name(&arg);
                return Argument {
                    is_reference: false,
                    definition,
                    name,
                    original_type: *original_type.clone(),
                };
            }
        })
        .collect();
    let return_type = match method.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ref return_type) => Some(*return_type.clone()),
    };
    let signature = method.sig.clone();

    TraitMethodType {
        signature,
        args,
        return_type,
    }
}

fn get_pat_type_name(pat_type: &PatType) -> Ident {
    match &*pat_type.pat {
        Pat::Ident(inner) => inner.ident.clone(),
        _ => panic!("unknown argument pattern"),
    }
}
