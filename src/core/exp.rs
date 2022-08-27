use std::{collections::{HashSet, HashMap}, fmt::Display};

use super::{inferable::CanInfer, substitution::Substitution};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exp<'a> {
    EVar(String),
    EApp(&'a Exp<'a>, &'a Exp<'a>),
    EAbs(String, &'a Exp<'a>),
    ELet(String, &'a Exp<'a>, &'a Exp<'a>),
    ELit(Literal)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Literal {
    LInt(i32),
    LBool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    TVar(String),
    TInt,
    TBool,
    TFun(Box<Type>, Box<Type>)
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::TVar(n) => {
                write!(f, "{}", n)
            },
            Type::TInt => {
                write!(f, "Int")
            },
            Type::TBool => {
                write!(f, "Bool")
            },
            Type::TFun(e1, e2) => {
                let a:&Type = e1;
                let b:&Type = e2;
                write!(f, "{} -> {}", a.to_string(), b.to_string())
            },
        }
    }
}

impl Type {
    pub fn bind_var(&self, var: &String) -> Substitution {
        if *self == Type::TVar((*var).clone()) {
            return Substitution::empty();
        } else {
            let mut m = HashMap::new();
            m.insert((*var).clone(), (*self).clone());
            return Substitution::new(m);
        }
    }
}

impl CanInfer<Type> for Type {
    fn find_free_var(&self) -> HashSet<String> {
        match self {
            Type::TVar(n) => {
                let mut a:HashSet<String> = HashSet::new();
                a.insert((*n).clone());
                a
            }
            Type::TInt => {
                HashSet::new()
            },
            Type::TBool => {
                HashSet::new()
            },
            Type::TFun(t1, t2) => {
                let a = (*t1).find_free_var();
                let b = (*t2).find_free_var();
                return a.union(&b)
                    .map(|i| (*i).clone())
                    .collect::<HashSet<String>>();
            },
        }
    }

    fn apply(&self, sub:& super::substitution::Substitution) -> Type {
        match self {
            Type::TVar(n) => {
                let fc = sub.try_find_var(&n).clone();
                return fc;
            },
            Type::TFun(t1, t2) => {
                let a = (*t1).apply(sub);
                let b = (*t2).apply(sub);
                let x = Type::TFun(Box::new(a), Box::new(b));
                return x;
            },
            literal => {
                return (*literal).clone();
            }
        }
    }
}
