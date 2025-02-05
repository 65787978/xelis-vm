use xelis_types::{Type, Value};
use xelis_environment::{FnInstance, FnParams, FnReturnType};
use super::EnvironmentBuilder;

pub fn register(env: &mut EnvironmentBuilder) {
    env.register_native_function("is_none", Some(Type::Optional(Box::new(Type::T))), vec![], is_none, 1, Some(Type::Bool));
    env.register_native_function("is_some", Some(Type::Optional(Box::new(Type::T))), vec![], is_some, 1, Some(Type::Bool));
    env.register_native_function("unwrap", Some(Type::Optional(Box::new(Type::T))), vec![], unwrap, 1, Some(Type::T));
    env.register_native_function("unwrap_or", Some(Type::Optional(Box::new(Type::T))), vec![Type::T], unwrap_or, 1, Some(Type::T));
}

fn is_none(zelf: FnInstance, _: FnParams) -> FnReturnType {
    Ok(Some(Value::Boolean(zelf?.as_optional(&Type::T)?.is_none())))
}

fn is_some(zelf: FnInstance, _: FnParams) -> FnReturnType {
    Ok(Some(Value::Boolean(zelf?.as_optional(&Type::T)?.is_some())))
}

fn unwrap(zelf: FnInstance, _: FnParams) -> FnReturnType {
    let opt = zelf?.take_from_optional(&Type::T)?;
    Ok(Some(opt.into_inner()))
}

fn unwrap_or(zelf: FnInstance, mut parameters: FnParams) -> FnReturnType {
    let default = parameters.remove(0);
    let optional = zelf?.take_optional()?;
    match optional {
        Some(value) => Ok(Some(value.into_inner())),
        None => Ok(Some(default.into_owned()))
    }
}