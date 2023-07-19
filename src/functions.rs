use indent::indent_all_by;

pub fn indent(spaces: i64, string: &str) -> String {
    indent_all_by(spaces as usize, string)
}

pub fn replace<'a>(string: &str, from: &str, to: &str) -> String {
    string.replace(from, to)
}

pub fn as_camel_case(string: &str) -> String {
    heck::AsLowerCamelCase(string).to_string()
}

pub fn as_pascal_case(string: &str) -> String {
    heck::AsPascalCase(string).to_string()
}

pub fn as_snake_case(string: &str) -> String {
    heck::AsSnakeCase(string).to_string()
}

pub fn as_capital_snake_case(string: &str) -> String {
    heck::AsShoutySnakeCase(string).to_string()
}

pub fn as_kebab_case(string: &str) -> String {
    heck::AsKebabCase(string).to_string()
}

pub fn as_capital_kebab_case(string: &str) -> String {
    heck::AsShoutyKebabCase(string).to_string()
}

pub fn as_train_case(string: &str) -> String {
    heck::AsTrainCase(string).to_string()
}

pub fn as_title_case(string: &str) -> String {
    heck::AsTitleCase(string).to_string()
}

pub fn docformat(string: &str) -> String {
    // trim leading and trailing whitespace
    let string = string.trim();

    // check every line to find the min leading whitespace
    // skip the first line and dont use any lines that are only whitespace
    let mut min_white = usize::MAX;
    for line in string.lines().skip(1) {
        let white = line.chars().take_while(|c| c.is_whitespace()).count();
        if white != line.len() && white < min_white {
            min_white = white;
        }
    }

    // if there was never a line found just early return the string
    if min_white == usize::MAX {
        return string.to_string();
    }

    // build the final string
    // add the first line
    let mut lines = string.lines();
    let mut output = match lines.next() {
        Some(line) => line.to_string(),
        None => return String::new(),
    };

    // add the rest and chop off whitespace
    for line in lines {
        output += "\n";
        if min_white < line.len() {
            output += &line[min_white..];
        }
    }

    // return the formatted string
    output.to_string()
}
