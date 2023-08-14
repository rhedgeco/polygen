fn render_each(items, seperator, function) {
    if items.len() == 0 { return ""; }
    let output = "";
    output += function.call(items[0]);
    for i in 1..items.len() {
        let item = items[i];
        output += seperator + function.call(item);
    }
    return output;
}
