use super::{exp::Type, inferable::CanInfer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scheme {
    free_type_vars: Vec<String>,
    t: Type
}

impl CanInfer<Scheme> for Scheme {
    fn find_free_var(&self) -> std::collections::HashSet<String> {
        return self.t.find_free_var()
            .difference(&self.free_type_vars.iter()
                .map(|v| (*v).clone())
                .collect())
            .map(|v| (*v).clone())
            .collect();
    }

    fn apply(&self, sub:& super::substitution::Substitution) -> Scheme {
        return Scheme{
            free_type_vars: self.free_type_vars.clone(),
            t: self.t.apply(&sub.eliminate(&self.free_type_vars))
        };
    }
}

impl Scheme {
    pub fn new(vars: Vec<String>, t: Type) -> Scheme {
        return Scheme { free_type_vars: vars, t };
    }

    pub fn get_free_type_vars(&self) -> Vec<String> {
        return self.free_type_vars.clone();
    }

    pub fn get_type(&self) -> Type {
        return self.t.clone();
    }
}