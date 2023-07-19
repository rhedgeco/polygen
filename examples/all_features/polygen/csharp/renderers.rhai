fn render_with_namespace(items, dll_name, namespace) {
`namespace ${namespace}
{
${render_bare(items, dll_name)}
}`
}

fn render_bare(items, dll_name) {
`public class ${dll_name} {}`
}
