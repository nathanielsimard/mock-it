use proc_macro2::Ident;
use quote::quote;
use syn::{parse2, Generics, WhereClause, WherePredicate};

pub struct TraitGenerics {
    generics: Generics,
}

impl TraitGenerics {
    pub fn new(generics: Generics) -> Self {
        Self { generics }
    }

    pub fn types(&self) -> Vec<Ident> {
        self.generics
            .type_params()
            .into_iter()
            .map(|tp| tp.ident.clone())
            .collect()
    }

    pub fn add_predicates(&mut self, predicate: WherePredicate) {
        let where_clause: WhereClause = match &self.generics.where_clause {
            Some(val) => parse2(quote! {
                #val
                    #predicate,
            })
            .unwrap(),
            None => parse2(quote! {
                where
                    #predicate,
            })
            .unwrap(),
        };
        self.generics.where_clause = Some(where_clause);
    }
}

impl Into<Generics> for TraitGenerics {
    fn into(self) -> Generics {
        self.generics
    }
}

pub fn add_generics(generics: &Generics) -> Generics {
    let mut trait_generics = TraitGenerics::new(generics.clone());
    trait_generics
        .types()
        .into_iter()
        .map(|ty| parse2(quote! { #ty: Clone + PartialEq }).unwrap())
        .for_each(|predicate| trait_generics.add_predicates(predicate));

    trait_generics.into()
}
