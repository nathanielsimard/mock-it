use proc_macro2::Ident;
use quote::quote;
use std::collections::HashSet;
use syn::{parse2, Generics, Type, WhereClause, WherePredicate};

pub struct MockItGenerics {
    generics: Generics,
}

impl MockItGenerics {
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

    pub fn add_predicate(&mut self, predicate: WherePredicate) {
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

    pub fn find_type_ident(&self, ty: &Type) -> Option<Ident> {
        for generic_ty in self.types().into_iter() {
            let idents = type2idents(ty.clone());
            if idents.contains(&generic_ty) {
                return Some(generic_ty);
            }
        }

        None
    }
}

fn type2idents(ty: Type) -> HashSet<Ident> {
    match ty {
        Type::Path(pty) => {
            let mut items = HashSet::new();
            if let Some(val) = pty.path.get_ident().map(|ty| ty.clone()) {
                items.insert(val);
            }
            return items;
        }
        Type::Ptr(ptr) => type2idents(*ptr.elem),
        Type::Reference(re) => type2idents(*re.elem),
        Type::Slice(slice) => type2idents(*slice.elem),
        _ => HashSet::new(),
    }
}

impl Into<Generics> for MockItGenerics {
    fn into(self) -> Generics {
        self.generics
    }
}
