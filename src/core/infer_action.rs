use std::collections::HashMap;

use super::{env::Env, exp::{Type, Exp, Literal}, scheme::Scheme, inferable::CanInfer, substitution::Substitution};

pub fn do_type_infer(default_env:& Env, exp:& Exp) -> Type {
    let (s, t) = infer(&default_env, &exp);
    println!("infer finish, substitutions: {:?} type: {:?}", s, t);
    return t.apply(&s);
}

pub fn generalize(env: Env, t: Type) -> Scheme {
    let free_type_vars_in_env = env.find_free_var();
    let free_type_vars_in_type = t.find_free_var();
    let free_vars: Vec<String> = free_type_vars_in_type.difference(&free_type_vars_in_env)
        .map(|x| (*x).clone())
        .collect();
    return Scheme::new(free_vars, t);
}

pub fn instantiated(scheme: Scheme) -> Type {
    let a:Vec<String> = scheme.get_free_type_vars();
    let b:Vec<Type> = scheme.get_free_type_vars().iter()
        .map(|_| create_new_type_var(&String::from("a")))
        .collect();
    let substitute = Substitution::new(a.into_iter().zip(b.into_iter()).collect());

    println!("instantiated, sub: {:?}, type: {}", substitute, scheme.get_type());
    return scheme.get_type().apply(&substitute);
}

pub fn unification(left:& Type, right:& Type) -> Substitution {
    match (left, right) {
        (Type::TFun(t1_left, t2_left), Type::TFun(t1_right, t2_right)) => {
            let u1 = unification(t1_left, t2_left);
            let u2 = unification(t1_right, t2_right);
            return u1.compose_substitution(&u2);
        },
        (Type::TVar(n), t) => {
            return t.bind_var(n);
        },
        (t,Type::TVar(n)) => {
            return t.bind_var(n);
        },
        (Type::TInt, Type::TInt) => {
            return Substitution::empty();
        },
        (Type::TBool, Type::TBool) => {
            return Substitution::empty();
        },
        (t1, t2) => {
            panic!("cannot unify this equation, t1: {:?} t2: {:?}", *t1, *t2);
        }
    }
}

pub fn infer(env:& Env, exp:& Exp) -> (Substitution, Type) {
    match exp {
        Exp::EVar(x) => {
            return (Substitution::empty(), instantiated(env.find(&x))); 
        },
        Exp::EApp(func, param) => {                                     
            let return_type = create_new_type_var(&String::from("a"));
            let (s1, type_of_func) = infer(&env, &func);
            let (s2, type_of_param) = infer(&env.apply(&s1), &param);

            println!("App infer info: return_type is {}, func_type is {}, param_type is{}", return_type, type_of_func, type_of_param);
            
            // x -> x x x 1
            let s3 = unification(&type_of_func.apply(&s2), &Type::TFun(Box::new(type_of_param), Box::new(return_type.clone()))); 

            println!("App subt: s1 is {:?} s2 is {:?} s3 is {:?}", s1, s2, s3);
            return (s3.compose_substitution(&s2).compose_substitution(&s1), return_type.apply(&s3)); 
        },
        Exp::EAbs(parm_name, func_body) => {
            let param_type = create_new_type_var(&String::from("a"));

            let mut var_type_item = HashMap::new();
            var_type_item.insert((*parm_name).clone(), Scheme::new([].to_vec(), param_type.clone()));

            let env_exclude_var = env.remove(parm_name);
            let func_env = env_exclude_var.merge(var_type_item);
            
            let (s1, type_of_func) = infer(&func_env, &func_body);

            println!("Abs subt: {:?}", s1);
            return (s1.clone(), Type::TFun(Box::new(param_type.apply(&s1)), Box::new(type_of_func)));
        },
        Exp::ELet(x, e1, e2) => {
            let (s1, e1_type) = infer(env, e1);
            
            println!("Let: e1_type is {}", e1_type.to_string());
            let generic_type_for_e1 = generalize(env.apply(&s1), e1_type);
            let mut var_type_item = HashMap::new();
            var_type_item.insert((*x).clone(), generic_type_for_e1);

            let env_exclude_var = env.remove(x);
            let e2_env = env_exclude_var.merge(var_type_item);

            let (s2, e2_type) = infer(&e2_env, &e2);

            println!("Let subt: s1 is {:?} s2 is {:?}", s1, s2);
            return (s1.compose_substitution(&s2), e2_type);
        },
        Exp::ELit(lit) => {
            return infer_literal(&lit);
        },
    }
}

pub fn infer_literal(lit:& Literal) -> (Substitution, Type) {
    match lit {
        Literal::LInt(_) => (Substitution::empty(), Type::TInt),
        Literal::LBool(_) => (Substitution::empty(), Type::TBool),
    }
}

static mut COUNTER: usize = 0;
pub fn create_new_type_var(prefix: &String) -> Type {
    unsafe {
        COUNTER += 1;
        return Type::TVar((*prefix).clone() + &(COUNTER.to_string()));
    }
}