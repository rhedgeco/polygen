mod polyfn;
mod polymod;
mod polystruct;
mod polytype;

use indent::indent_by;
use indoc::formatdoc;
use polygen::PolyBag;

use self::polymod::render_module;

pub struct PolygenCSharp {
    pub lib_name: String,
    pub namespace: String,
    pub bag: PolyBag,
}

impl PolygenCSharp {
    pub fn generate(self) -> String {
        let namespace = self.namespace;
        let doc = formatdoc! {"
            using System.Runtime.InteropServices;

            namespace {namespace}
            {{
                polygen-inner
            }}
        "};

        let inner = render_module(&self.lib_name, self.bag.root_module());
        doc.replace("polygen-inner", &indent_by(4, inner))
    }
}
