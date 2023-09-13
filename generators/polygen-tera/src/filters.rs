use std::collections::HashMap;

use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTitleCase, ToTrainCase};
use tera::{Result, Value};

pub fn to_pascal_case(input: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    case_convert(input, |s| s.to_pascal_case())
}

pub fn to_camel_case(input: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    case_convert(input, |s| s.to_lower_camel_case())
}

pub fn to_snake_case(input: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    case_convert(input, |s| s.to_snake_case())
}

pub fn to_kebab_case(input: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    case_convert(input, |s| s.to_kebab_case())
}

pub fn to_train_case(input: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    case_convert(input, |s| s.to_train_case())
}

pub fn to_title_case(input: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    case_convert(input, |s| s.to_title_case())
}

fn case_convert(input: &Value, f: impl Fn(&str) -> String) -> Result<Value> {
    match input {
        Value::String(s) => Ok(f(s).into()),
        _ => Err("invalid input string".into()),
    }
}
