use crate::generics::MockItGenerics;
use crate::mock_fn::MockFn;
use quote::quote;
use syn::{parse2, Generics};

pub struct MockItTraitGenerics {
    generics: MockItGenerics,
}

impl MockItTraitGenerics {
    pub fn new(generics: &Generics) -> Self {
        Self {
            generics: MockItGenerics::new(generics.clone()),
        }
    }

    pub fn configure_predicates(&mut self, mock_fns: &Vec<MockFn>) {
        self.configure_input_predicates(mock_fns);
        self.configure_output_predicates(mock_fns);
    }

    fn configure_input_predicates(&mut self, mock_fns: &Vec<MockFn>) {
        let mut input_types = Vec::new();

        for mock_fn in mock_fns {
            for ty in mock_fn.input_original_types().into_iter() {
                if let Some(generic_ty) = self.generics.find_type_ident(&ty) {
                    input_types.push(generic_ty);
                }
            }
        }

        input_types
            .into_iter()
            .map(|ty| parse2(quote! { #ty: Clone + PartialEq }).unwrap())
            .for_each(|predicate| self.generics.add_predicate(predicate));
    }

    fn configure_output_predicates(&mut self, mock_fns: &Vec<MockFn>) {
        let mut output_types = Vec::new();

        for mock_fn in mock_fns {
            if let Some(ty) = mock_fn.output_original_type() {
                if let Some(generic_ty) = self.generics.find_type_ident(&ty) {
                    output_types.push(generic_ty);
                }
            }
        }

        output_types
            .into_iter()
            .map(|ty| parse2(quote! { #ty: Clone }).unwrap())
            .for_each(|predicate| self.generics.add_predicate(predicate));
    }
}

impl Into<Generics> for MockItTraitGenerics {
    fn into(self) -> Generics {
        self.generics.into()
    }
}
