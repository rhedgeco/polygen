mod filters;

use std::{
    fs::{self, File},
    path::PathBuf,
};

use polygen::PolyBag;
use tera::{Context, Tera};

pub type Result<T> = tera::Result<T>;

pub struct PolyTera {
    tera: Tera,
}

impl PolyTera {
    pub fn new(input_glob: impl AsRef<str>) -> Result<Self> {
        let mut tera = Tera::new(input_glob.as_ref())?;
        tera.register_filter("to_pascal_case", filters::to_pascal_case);
        tera.register_filter("to_camel_case", filters::to_camel_case);
        tera.register_filter("to_snake_case", filters::to_snake_case);
        tera.register_filter("to_kebab_case", filters::to_kebab_case);
        tera.register_filter("to_train_case", filters::to_train_case);
        tera.register_filter("to_title_case", filters::to_title_case);

        Ok(Self { tera })
    }

    pub fn render(&self, out_dir: impl AsRef<str>, bag: PolyBag) -> Result<()> {
        let out_dir = out_dir.as_ref();

        // construct context by serializing PolyBag
        let context = Context::from_serialize(bag)?;

        // iterate over all templates and render them
        for template in self.tera.get_template_names() {
            // get the relative path and skip if its not a template
            if !template.ends_with(".tera") {
                continue;
            }

            // create the output path for the file
            let path = PathBuf::from(out_dir).join(template.trim_end_matches(".tera"));
            if let Some(folder) = path.parent() {
                fs::create_dir_all(folder).unwrap();
            }

            // create and render the file
            let file = File::create(path).unwrap();
            self.tera.render_to(template, &context, file).unwrap();
        }

        Ok(())
    }
}
