pub fn join<T>(
    mut iter: impl Iterator<Item = T>,
    seperator: impl AsRef<str>,
    mut f: impl FnMut(T) -> String,
) -> String {
    let mut out = match iter.next() {
        None => return String::new(),
        Some(item) => f(item),
    };

    let seperator = seperator.as_ref();
    for item in iter {
        out += seperator;
        out += &f(item);
    }

    out
}
