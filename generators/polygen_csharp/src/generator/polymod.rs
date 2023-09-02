use indent::indent_by;
use indoc::formatdoc;
use polygen::PolyMod;

use crate::utils;

use super::{polyfn::render_function, polystruct::render_struct};

pub fn render_module(lib_name: &str, m: &PolyMod) -> String {
    let class_name = heck::AsPascalCase(m.name());
    let doc = formatdoc! {"
        public static class {class_name}
        {{
            polygen-inner
        }}"
    };

    let mut inner = Vec::new();
    inner.push(utils::render_each(m.structs(), "\n\n", |s| {
        render_struct(s)
    }));
    inner.push(utils::render_each(m.functions(), "\n\n", |f| {
        render_function(lib_name, f)
    }));
    inner.push(utils::render_each(m.modules(), "\n\n", |m| {
        render_module(lib_name, m)
    }));

    let inner = utils::render_each(inner.into_iter().filter(|s| s != ""), "\n\n", |s| s);
    doc.replace("polygen-inner", &indent_by(4, inner))
}
