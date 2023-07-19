fn item(item) {
    if item.struct != () { return struct(item.struct); }
}

fn struct(struct) {
    let template = #{};
    template.name = "Test";
    return template;
}
