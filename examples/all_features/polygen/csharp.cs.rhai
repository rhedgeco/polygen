import "csharp/renderers" as rend;
import "csharp/processors" as proc;

fn process(item) { proc::item(item) }

fn render(items) { rend::render_with_namespace(items, "libtest", "RustNative") }
