use crate::ast::{BlockStatement, Expression, HashLiteral, Node, Program, Statement};
use crate::object::builtins::builtins_fn;
use crate::object::environment::{Env, Environment};
use crate::object::hash::{HashKey, HashPair};
use crate::object::{new_error, Function, HashObject, Object, Quote};
use std::collections::HashMap;
use std::rc::Rc;

const TRUE: Object = Object::BOOLEAN(true);
const FALSE: Object = Object::BOOLEAN(false);
const NULL: Object = Object::NULL;

fn is_error(input: &Object) -> bool {
    matches!(input, Object::ERROR(_))
}

pub trait Evaluable {
    fn eval(self: &Self, env: Env) -> Object;
}

impl Evaluable for Program {
    fn eval(self: &Self, env: Env) -> Object {
        let mut result = Object::NULL;
        for stmt in &self.statements {
            result = eval(stmt.as_ref(), env.clone());
            if let Object::RETURN(rv) = result {
                return *rv;
            } else if matches!(result, Object::ERROR(_)) {
                return result;
            }
        }
        return result;
    }
}

impl Evaluable for Expression {
    fn eval(self: &Self, env: Env) -> Object {
        match self {
            Expression::IntLit(int_lit) => Object::INTEGER(int_lit.value),
            Expression::Identifier(identifier) => match env.borrow().get(&identifier.value) {
                Some(val) => val,
                None => {
                    let builtin_fn = builtins_fn(&identifier.value);
                    if matches!(builtin_fn, Object::NULL) {
                        return new_error(format!("Identifier not found: {}", identifier.value));
                    };
                    builtin_fn
                }
            },
            Expression::PreExp(prefix_expression) => {
                let right = eval(&prefix_expression.right, env);
                if is_error(&right) {
                    return right;
                }
                eval_prefix_expression(prefix_expression.operator.clone(), right)
            }
            Expression::InExp(infix_expression) => {
                let right = eval(&infix_expression.right, env.clone());
                if is_error(&right) {
                    return right;
                }
                let left = eval(&infix_expression.left, env);
                if is_error(&left) {
                    return left;
                }
                eval_infix_expression(infix_expression.operator.clone(), left, right)
            }
            Expression::BoolLit(boolean) => native_bool_to_boolean_object(boolean.value),
            Expression::IfExp(if_expression) => {
                let condition = eval(&if_expression.condition, env.clone());
                if is_error(&condition) {
                    return condition;
                }

                if is_truthy(condition) {
                    if_expression.consequence.eval(env)
                } else if let Some(alter) = if_expression.alternative.as_ref() {
                    alter.eval(env)
                } else {
                    NULL
                }
            }
            Expression::FncLit(function_literal) => {
                let fn_para = function_literal
                    .parameters
                    .iter()
                    .map(|p| {
                        if let Expression::Identifier(id) = p {
                            id.clone()
                        } else {
                            panic!("function parameters is not id")
                        }
                    })
                    .collect();

                Object::FUNCTION(Rc::new(Function {
                    parameters: fn_para,
                    body: function_literal.body.clone(),
                    env: Environment::extend(&env),
                }))
            }
            Expression::CallExp(call_expression) => {
                match call_expression.function.token_literal() {
                    Some(tok) => {
                        if tok == "quote" {
                            return quote(call_expression.arguments[0].clone());
                        }
                    }
                    None => return new_error("function token is null".to_string()),
                }

                let function = call_expression.function.eval(env.clone());
                if is_error(&function) {
                    return function;
                }
                let args = eval_expressions(call_expression.arguments.clone(), env);
                if args.len() == 1 && is_error(&args[0]) {
                    return args[0].clone();
                }

                apply_function(function, args)
            }
            Expression::StringLit(str_lit) => Object::STRING(str_lit.value.clone()),
            Expression::ArrayExp(array_exp) => {
                let elements = eval_expressions(array_exp.elements.clone(), env);
                if elements.len() == 1 && is_error(&elements[0]) {
                    return elements[0].clone();
                }
                Object::ARRAY(elements)
            }
            Expression::IndexExp(index_expression) => {
                let left = eval(&*index_expression.left, env.clone());
                if is_error(&left) {
                    return left;
                }
                let index = eval(&index_expression.index, env);
                if is_error(&index) {
                    return index;
                }
                eval_index_expression(left, index)
            }
            Expression::HashLit(hash_literal) => eval_hash_literal(hash_literal, env.clone()),
        }
    }
}

fn eval_hash_literal(hash_literal: &HashLiteral, env: Env) -> Object {
    let mut pairs = HashMap::new();
    for (key_node, value_node) in &hash_literal.pairs {
        let key = eval(key_node, env.clone());
        if is_error(&key) {
            return key;
        }
        let hash_key = match HashKey::new(key.clone()) {
            Ok(v) => v,
            Err(e) => return new_error(e),
        };

        let value = eval(value_node, env.clone());
        if is_error(&value) {
            return value;
        }

        pairs.insert(hash_key, HashPair { key, value });
    }

    Object::HASH(Rc::new(HashObject { pairs }))
}

fn eval_index_expression(left: Object, index: Object) -> Object {
    if matches!(left, Object::ARRAY(_)) && matches!(index, Object::INTEGER(_)) {
        eval_array_index_expression(left, index)
    } else if matches!(left, Object::HASH(_)) {
        eval_hash_index_expression(left, index)
    } else {
        new_error(format!("index operator not supported: {}", left.ob_type()))
    }
}

fn eval_hash_index_expression(left: Object, index: Object) -> Object {
    let hash_object = if let Object::HASH(v) = left {
        v
    } else {
        return new_error("Left is not a hash object".to_string());
    };
    let key = match HashKey::new(index.clone()) {
        Ok(v) => v,
        Err(_e) => return new_error(format!("unusable as hash key: {}", index.ob_type())),
    };

    match hash_object.pairs.get(&key) {
        Some(v) => v.value.clone(),
        None => Object::NULL,
    }
}

fn eval_array_index_expression(arr: Object, i: Object) -> Object {
    let array_object: Vec<Object> = match arr {
        Object::ARRAY(elements) => elements,
        _ => [].to_vec(),
    };

    let index: usize = match i {
        Object::INTEGER(i) => match i.try_into() {
            Ok(v) => v,
            Err(_) => return Object::NULL,
        },
        _ => return Object::NULL,
    };

    match array_object.get(index) {
        Some(v) => v.clone(),
        None => Object::NULL,
    }
}

fn apply_function(function: Object, args: Vec<Object>) -> Object {
    if let Object::FUNCTION(fnc) = function {
        let extended_env = extend_function_env(fnc.clone(), args);
        let val = if let Statement::BlcStmt(block) = fnc.body.as_ref() {
            eval_block_statement(block, extended_env)
        } else {
            new_error(format!("body is not a block"))
        };
        unwrap_return_value(val)
    } else if let Object::BUILTIN(b_fn) = function {
        (b_fn.function)(args)
    } else {
        new_error(format!("not a function:{}", function.ob_type()))
    }
}

fn unwrap_return_value(val: Object) -> Object {
    if let Object::RETURN(rv) = val {
        *rv
    } else {
        val
    }
}

fn extend_function_env(fn_rc: Rc<Function>, args: Vec<Object>) -> Env {
    let f_env = Environment::extend(&fn_rc.env);
    for (index, param) in fn_rc.parameters.iter().enumerate() {
        f_env.borrow_mut().set(param.value.clone(), &args[index]);
    }

    f_env
}

fn eval_expressions(arguments: Vec<Expression>, env: Env) -> Vec<Object> {
    let mut result: Vec<Object> = vec![];

    for exp in arguments {
        let val = eval(&exp, env.clone());
        if is_error(&val) {
            result.push(val);
            return result;
        }
        result.push(val);
    }

    result
}

fn is_truthy(condition: Object) -> bool {
    match condition {
        NULL | FALSE => false,
        _ => true,
    }
}

fn eval_infix_expression(operator: String, left: Object, right: Object) -> Object {
    if matches!(left, Object::INTEGER(_)) && matches!(right, Object::INTEGER(_)) {
        eval_integer_infix_expression(operator, left, right)
    } else if matches!(left, Object::STRING(_)) && matches!(right, Object::STRING(_)) {
        eval_string_infix_expression(operator, left, right)
    } else if operator.as_str() == "==" {
        native_bool_to_boolean_object(left == right)
    } else if operator.as_str() == "!=" {
        native_bool_to_boolean_object(left != right)
    } else if left.ob_type() != right.ob_type() {
        new_error(format!(
            "type mismatch:{} {} {}",
            left.ob_type(),
            operator,
            right.ob_type()
        ))
    } else {
        new_error(format!(
            "unknown operator:{} {} {}",
            left.ob_type(),
            operator,
            right.ob_type()
        ))
    }
}

fn eval_integer_infix_expression(operator: String, left: Object, right: Object) -> Object {
    let left_value = match left {
        Object::INTEGER(s) => s,
        _ => 0,
    };

    let right_value = match right {
        Object::INTEGER(s) => s,
        _ => 0,
    };

    match operator.as_str() {
        "+" => Object::INTEGER(left_value + right_value),
        "-" => Object::INTEGER(left_value - right_value),
        "*" => Object::INTEGER(left_value * right_value),
        "/" => Object::INTEGER(left_value / right_value),
        "<" => native_bool_to_boolean_object(left_value < right_value),
        ">" => native_bool_to_boolean_object(left_value > right_value),
        "==" => native_bool_to_boolean_object(left_value == right_value),
        "!=" => native_bool_to_boolean_object(left_value != right_value),
        _ => new_error(format!(
            "unknown operator:{} {} {}",
            left.ob_type(),
            operator,
            right.ob_type(),
        )),
    }
}

fn eval_string_infix_expression(operator: String, left: Object, right: Object) -> Object {
    if operator != "+" {
        return new_error(format!(
            "unknown operator: {} {} {}",
            left.ob_type(),
            operator,
            right.ob_type(),
        ));
    }
    let left_val = match left {
        Object::STRING(str) => str,
        _ => String::from(""),
    };
    let right_val = match right {
        Object::STRING(str) => str,
        _ => String::from(""),
    };

    Object::STRING(left_val + &right_val)
}

fn native_bool_to_boolean_object(value: bool) -> Object {
    if value {
        TRUE
    } else {
        FALSE
    }
}

fn eval_prefix_expression(operator: String, right: Object) -> Object {
    match operator.as_str() {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => new_error(format!("unknown operator:{}{}", operator, right.ob_type())),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    if let Object::INTEGER(s) = right {
        Object::INTEGER(-s)
    } else {
        new_error(format!("unknown operator:-{}", right.ob_type()))
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        TRUE => FALSE,
        FALSE => TRUE,
        NULL => TRUE,
        _ => FALSE,
    }
}

fn quote(args: Expression) -> Object {
    Object::QUOTE(Rc::new(Quote { node: args.clone() }))
}

impl Evaluable for Statement {
    fn eval(self: &Self, env: Env) -> Object {
        match self {
            Statement::ExpStmt(exp_stmt) => exp_stmt.expression.eval(env),
            Statement::LetStmt(let_statement) => {
                let value = let_statement.value.eval(env.clone());
                if is_error(&value) {
                    return value;
                }
                env.borrow_mut()
                    .set(let_statement.name.value.clone(), &value);
                value
            }
            Statement::RetStmt(return_statement) => {
                let value = return_statement.return_value.eval(env);
                if is_error(&value) {
                    return value;
                }
                Object::RETURN(Box::new(value))
            }
            Statement::BlcStmt(block_statement) => eval_block_statement(block_statement, env),
        }
    }
}

fn eval_block_statement(block: &BlockStatement, env: Env) -> Object {
    let mut result: Object = Object::NULL;
    let block_env = Environment::extend(&env);
    for stmt in &block.statements {
        result = eval(stmt, block_env.clone());
        if !matches!(result, Object::NULL) {
            if matches!(result, Object::RETURN(_)) || matches!(result, Object::ERROR(_)) {
                return result;
            }
        }
    }
    return result;
}

pub fn eval<T: Evaluable>(node: &T, env: Env) -> Object {
    node.eval(env)
}

#[cfg(test)]
mod test;
