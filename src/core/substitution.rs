use std::collections::HashMap;

use super::{exp::Type, inferable::CanInfer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Substitution {
    repo: HashMap<String, Type>
}

impl Substitution {

    pub fn new(repo: HashMap<String, Type>) -> Substitution {
        return Substitution { repo };
    }

    pub fn empty() -> Substitution {
        return Substitution { repo: HashMap::new() };
    }

    pub fn try_find_var(& self, name: &String) -> Type {
        let res = match self.repo.get(name) {
            Some(t) => (*t).clone(),
            None => Type::TVar((*name).clone()),
        };
        return res;
    }

    pub fn eliminate(&self, vars: &Vec<String>) -> Substitution {
        let mut r = self.repo.clone();
        vars.iter().for_each(|v| {r.remove(v);});
        return Substitution { repo: r };
    }

    pub fn compose_substitution(&self, s2: &Substitution) -> Substitution {
        let mut substitude_repo:HashMap<String, Type> = s2.repo.iter()
            .map(|(k, v)| {return (k.clone(), v.apply(self));})
            .collect();
        substitude_repo.extend(self.repo.clone());
        return Substitution{ repo: substitude_repo};
    }
}

