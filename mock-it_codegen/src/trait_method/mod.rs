use syn::{FnArg, ItemTrait, PatType, ReturnType, Signature, TraitItem, TraitItemMethod, Type};

pub struct TraitMethodType {
    pub args: Vec<PatType>,
    pub return_type: Option<Type>,
    pub signature: Signature,
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
    let args: Vec<PatType> = method
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(inner) => Some(inner.clone()),
            _ => None,
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
