use std::collections::HashMap;

use type_infer_sys::core::{exp::{Exp, Type, Literal}, env::Env, infer_action::do_type_infer, scheme::Scheme};

fn main() {    
    let mut map:HashMap<String, Scheme> = HashMap::new();
    // map.insert(String::from("x"), Scheme::new([].to_vec(), Type::TInt));
    let default_env = Env::new(map);
    
    // test_app(&default_env);
    test_let_id_x_x_id_2(&default_env);
    // test_let_id_x_let_y_x_x_id(&default_env);
}

fn test_app(default_env:& Env) {
    let res = Exp::EVar("x".to_string());
    let e0: Exp = Exp::EAbs("x".to_string(), &res);
    let t = do_type_infer(default_env, &e0);
    println!("{:?}", t);
}

fn test_let_id_x_x_id(default_env:& Env) {
    let e1_return_exp = Exp::EVar("x".to_string());
    let e1 = Exp::EAbs("x".to_string(), &e1_return_exp);

    let f_1 = Exp::EVar("id".to_string());
    let f_2 =  Exp::EVar("id".to_string());
    let e2 = Exp::EApp(&f_1, &f_2);

    let let_exp = Exp::ELet("id".to_string(), &e1, &e2);
    
    let t = do_type_infer(default_env, &let_exp);
    println!("{}", t.to_string());
}

fn test_let_id_x_x_id_2(default_env:& Env) {
    let e1_return_exp = Exp::EVar("x".to_string());
    let e1 = Exp::EAbs("x".to_string(), &e1_return_exp);

    let f_1 = Exp::EVar("id".to_string());
    let f_2 =  Exp::EVar("id".to_string());
    let e2 = Exp::EApp(&f_1, &f_2);

    let two = Exp::ELit(Literal::LInt(2));
    let e2_invoke = Exp::EApp(&e2, &two);

    let let_exp = Exp::ELet("id".to_string(), &e1, &e2_invoke);
    
    let t = do_type_infer(default_env, &let_exp);
    println!("{}", t.to_string());
}

fn test_let_id_x_let_y_x_x_id(default_env:& Env) {

    let e1_inner = Exp::EVar("x".to_string());
    let e2_inner = Exp::EVar("y".to_string());
    let let_exp_inner = Exp::ELet("y".to_string(), &e1_inner, &e2_inner);

    let e1 = Exp::EAbs("x".to_string(), &let_exp_inner);

    let e2 = Exp::EVar("id".to_string());

    let let_exp = Exp::ELet("id".to_string(), &e1, &e2);
    
    let t = do_type_infer(default_env, &let_exp);
    println!("{}", t.to_string());
}

fn test_abs(default_env:& Env) {
    // let e1_inner = Exp::EVar("x".to_string());
    // let e2_inner = Exp::EVar("y".to_string());
    // let let_exp_inner = Exp::ELet("y".to_string(), &e1_inner, &e2_inner);

    // let e1 = Exp::EAbs("x".to_string(), &let_exp_inner);

    // let e2 = Exp::EVar("id".to_string());

    // let let_exp = Exp::ELet("id".to_string(), &e1, &e2);

    // let abs = Exp::EApp("x".to_string(), )
    
    // let t = do_type_infer(default_env, &let_exp);
    // println!("{}", t.to_string());
}