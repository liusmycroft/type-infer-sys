use std::collections::HashMap;

use super::{scheme::Scheme, inferable::CanInfer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Env {
    repo: HashMap<String, Scheme>
}

impl Env {
    pub fn new(repo: HashMap<String, Scheme>) -> Env {
        return Env { repo };
    } 

    pub fn empty() -> Env {
        return Env { repo: HashMap::new()};
    } 

    pub fn merge(&self, other_env: HashMap<String, Scheme>) -> Env {
        let mut x = HashMap::new();
        x.extend(self.repo.clone());
        x.extend(other_env);
        return Env { repo: x };
    }

    pub fn remove(&self, var: &str) -> Env {
        let mut m = self.repo.clone();
        m.remove(var);
        return Env{repo: m};
    }

    pub fn find(&self, var: &str) -> Scheme {
        return match self.repo.get(var) {
            Some(x) => x.clone(),
            None => panic!("cannot find var type, var: {}", var),
        };
    }
}

impl CanInfer<Env> for Env {
    fn find_free_var(&self) -> std::collections::HashSet<String> {
        let schemes:Vec<Scheme> = self.repo.values().map(|v| (*v).clone()).collect();
        return schemes.find_free_var();
    }

    fn apply(&self, sub:& super::substitution::Substitution) -> Env {
        return Env {repo: self.repo.iter().map(|(k, v)| {
            return ((*k).clone(), (*v).apply(sub));
        }).collect()};
    }
}