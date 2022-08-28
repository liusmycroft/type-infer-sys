use std::collections::HashMap;

use type_infer_sys::core::{exp::{Exp, Type, Literal}, env::Env, infer_action::do_type_infer, scheme::Scheme};

fn main() {    
    // 预设环境
    let mut map:HashMap<String, Scheme> = HashMap::new();
    // map.insert(String::from("x"), Scheme::new(["x".to_string()].to_vec(), Type::TVar("x".to_string())));
    // map.insert(String::from("y"), Scheme::new(["y".to_string()].to_vec(), Type::TVar("y".to_string())));
    let default_env = Env::new(map);
    
    // 测试函数
    test_two(&default_env);
}

// f: y -> y, f(true)
fn test_one(default_env:& Env) {
    let res = Exp::EVar("y".to_string());
    let e0_1: Exp = Exp::EAbs("y".to_string(), &res);

    let bool_p = Exp::ELit(Literal::LBool(true));
    let e0: Exp = Exp::EApp(&e0_1, &bool_p);
    let t = do_type_infer(default_env, &e0);
    println!("{}", t.to_string());
}

// f: x -> x(x(1))
fn test_two(default_env:& Env) {
    let x_1 = Exp::EVar("x".to_string());
    let p_0 = Exp::ELit(Literal::LInt(1));
    let app_0 = Exp::EApp(&x_1, &p_0);

    let x_2 = Exp::EVar("x".to_string());
    let app_0 = Exp::EApp(&x_2, &app_0);

    let abs_1 = Exp::EAbs("x".to_string(), &app_0);

    let t = do_type_infer(default_env, &abs_1);
    println!("{}", t.to_string());
}

// f: x,y -> x(y(1))(y(1))
fn test_three(default_env:& Env) {
    let y_1 = Exp::EVar("y".to_string());
    let p_0 = Exp::ELit(Literal::LInt(1));
    let app_0 = Exp::EApp(&y_1, &p_0);

    let x_1 = Exp::EVar("x".to_string());
    let app_1 = Exp::EApp(&x_1, &app_0);

    let y_2 = Exp::EVar("y".to_string());
    let p_1 = Exp::ELit(Literal::LInt(1));
    let app_2 = Exp::EApp(&y_2, &p_1);

    let app_3 = Exp::EApp(&app_1, &app_2);

    let abs_1 = Exp::EAbs("y".to_string(), &app_3);
    let abs_2 = Exp::EAbs("x".to_string(), &abs_1);

    let t = do_type_infer(default_env, &abs_2);
    println!("{}", t.to_string());
}

// let id = x -> x in id id
fn test_four(default_env:& Env) {
    let e1_return_exp = Exp::EVar("x".to_string());
    let e1 = Exp::EAbs("x".to_string(), &e1_return_exp);

    let f_1 = Exp::EVar("id".to_string());
    let f_2 =  Exp::EVar("id".to_string());
    let e2 = Exp::EApp(&f_1, &f_2);

    let let_exp = Exp::ELet("id".to_string(), &e1, &e2);
    
    let t = do_type_infer(default_env, &let_exp);
    println!("{}", t.to_string());
}

// let id = let y = x -> y in id
fn test_five(default_env:& Env) {
    let e1_inner = Exp::EVar("x".to_string());
    let e2_inner = Exp::EVar("y".to_string());
    let let_exp_inner = Exp::ELet("y".to_string(), &e1_inner, &e2_inner);

    let e1 = Exp::EAbs("x".to_string(), &let_exp_inner);

    let e2 = Exp::EVar("id".to_string());

    let let_exp = Exp::ELet("id".to_string(), &e1, &e2);
    
    let t = do_type_infer(default_env, &let_exp);
    println!("{}", t.to_string());
}