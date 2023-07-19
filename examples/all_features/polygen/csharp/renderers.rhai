fn render_with_namespace(namespace, class_name, items) {
`namespace ${as_pascal_case(namespace)}
{
${indent(4, render_bare(class_name, items))}
}`
}

fn render_bare(class_name, items) {
`public class ${as_pascal_case(class_name)}
{
}`
}
