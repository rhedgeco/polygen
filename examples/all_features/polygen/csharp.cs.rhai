import "csharp/renderers" as rend;
import "csharp/processors" as proc;

fn process(item) { proc::item(item) }

fn render(name, items) { rend::render_with_namespace("RustNative", name, items) }
