pub fn render_each<T>(
    mut items: impl Iterator<Item = T>,
    seperator: impl AsRef<str>,
    f: impl Fn(T) -> String,
) -> String {
    let Some(item) = items.next() else {
        return String::new();
    };

    let mut output = f(item);
    let seperator = seperator.as_ref();
    for item in items {
        let item = f(item);
        output += &format!("{seperator}{item}");
    }

    output
}
