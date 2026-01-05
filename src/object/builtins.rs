use std::rc::Rc;

use crate::object::new_error;

use super::{Builtin, Object};

pub fn builtins_fn(input: &str) -> Object {
    match input {
        "len" => Object::BUILTIN(Rc::new(Builtin {
            function: |parameter| {
                if parameter.len() != 1 {
                    return new_error(format!(
                        "wrong number of arguments. got={}, want 1",
                        parameter.len()
                    ));
                };

                match parameter[0].clone() {
                    Object::ARRAY(arr) => Object::INTEGER(arr.len() as i64),
                    Object::STRING(str) => Object::INTEGER(str.len() as i64),
                    _ => new_error(format!(
                        "argument to `len` not supported, got={}",
                        parameter[0].ob_type()
                    )),
                }
            },
        })),
        "first" => Object::BUILTIN(Rc::new(Builtin {
            function: |parameter| {
                if parameter.len() != 1 {
                    return new_error(format!(
                        "wrong number of arguments.got ={}, want 1",
                        parameter.len()
                    ));
                };

                match parameter[0].clone() {
                    Object::ARRAY(arr) => match arr.first() {
                        Some(v) => v.clone(),
                        None => Object::NULL,
                    },
                    _ => {
                        return new_error(format!(
                            "argument to 'first' must be ARRAY, got {}",
                            parameter[0].ob_type()
                        ))
                    }
                }
            },
        })),
        "last" => Object::BUILTIN(Rc::new(Builtin {
            function: |parameter| {
                if parameter.len() != 1 {
                    return new_error(format!(
                        "wrong number of arguments.got ={}, want 1",
                        parameter.len()
                    ));
                };

                match parameter[0].clone() {
                    Object::ARRAY(arr) => match arr.last() {
                        Some(v) => v.clone(),
                        None => Object::NULL,
                    },
                    _ => {
                        return new_error(format!(
                            "argument to 'last' must be ARRAY, got {}",
                            parameter[0].ob_type()
                        ))
                    }
                }
            },
        })),
        "rest" => Object::BUILTIN(Rc::new(Builtin {
            function: |parameter| {
                if parameter.len() != 1 {
                    return new_error(format!(
                        "wrong number of arguments.got ={}, want 1",
                        parameter.len()
                    ));
                };

                match parameter[0].clone() {
                    Object::ARRAY(arr) => match arr.split_first() {
                        Some((_first, rest)) => Object::ARRAY(rest.to_vec()),
                        None => Object::NULL,
                    },
                    _ => {
                        return new_error(format!(
                            "argument to 'rest' must be ARRAY, got {}",
                            parameter[0].ob_type()
                        ))
                    }
                }
            },
        })),
        "push" => Object::BUILTIN(Rc::new(Builtin {
            function: |parameter| {
                if parameter.len() != 2 {
                    return new_error(format!(
                        "wrong number of arguments.got ={}, want 2",
                        parameter.len()
                    ));
                };

                match parameter[0].clone() {
                    Object::ARRAY(arr) => {
                        let mut clone = arr.clone();
                        clone.push(parameter[1].clone());
                        Object::ARRAY(clone)
                    }
                    _ => {
                        return new_error(format!(
                            "argument to 'push' must be ARRAY, got {}",
                            parameter[0].ob_type()
                        ))
                    }
                }
            },
        })),
        "put" => Object::BUILTIN(Rc::new(Builtin {
            function: |parameter| {
                for arg in parameter {
                    println!("{}", arg.inspect())
                }
                Object::NULL
            },
        })),
        _ => Object::NULL,
    }
}
