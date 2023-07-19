use indent::indent_all_by;

pub fn indent(spaces: i64, string: String) -> String {
    indent_all_by(spaces as usize, string)
}

pub fn as_camel_case(string: String) -> String {
    heck::AsLowerCamelCase(string).to_string()
}

pub fn as_pascal_case(string: String) -> String {
    heck::AsPascalCase(string).to_string()
}

pub fn as_snake_case(string: String) -> String {
    heck::AsSnakeCase(string).to_string()
}

pub fn as_capital_snake_case(string: String) -> String {
    heck::AsShoutySnakeCase(string).to_string()
}

pub fn as_kebab_case(string: String) -> String {
    heck::AsKebabCase(string).to_string()
}

pub fn as_capital_kebab_case(string: String) -> String {
    heck::AsShoutyKebabCase(string).to_string()
}

pub fn as_train_case(string: String) -> String {
    heck::AsTrainCase(string).to_string()
}

pub fn as_title_case(string: String) -> String {
    heck::AsTitleCase(string).to_string()
}
