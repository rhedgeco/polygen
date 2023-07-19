fn item(item) {
    if item.struct != () { return struct(item.struct); }
    else { throw "item is unsupported"; }
}

fn struct(struct) {
    let template = #{};
    template.name = "Test";
    return template;
}
