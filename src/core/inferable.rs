use std::collections::HashSet;

use super::substitution::Substitution;

pub trait CanInfer<T> {
    fn find_free_var(&self) -> HashSet<String>;
    fn apply(&self, sub:& Substitution) -> T;
}

impl <T: CanInfer<T>> CanInfer<Vec<T>> for Vec<T> {
    fn find_free_var(&self) -> HashSet<String> {
        return self.iter().map(|t| t.find_free_var())
            .flat_map(|set| set)
            .collect();
    }

    fn apply(&self, sub:& Substitution) -> Vec<T> {
        return self.iter().map(|t| t.apply(sub))
            .collect();
    }
}