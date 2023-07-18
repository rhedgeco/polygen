use super::{
    types::{self, PolyStruct},
    PolyMap,
};
use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    path::PathBuf,
    sync::Mutex,
};
use syn::Item;
use tera::{Context, Tera};

pub type Result<T> = anyhow::Result<T>;

pub struct PolygenBuilder {
    input: &'static str,
    output: &'static str,
    inner: Mutex<Option<InnerBuilder>>,
}

impl PolygenBuilder {
    pub const fn new(input_glob: &'static str, output_dir: &'static str) -> Self {
        Self {
            input: input_glob,
            output: output_dir,
            inner: Mutex::new(None),
        }
    }

    pub fn write_item(&self, item: &Item) -> Result<()> {
        let mut guard = self.inner.lock().expect("Builder lock poisoned");
        let inner = match &mut *guard {
            Some(inner) => inner,
            option => {
                *option = Some(InnerBuilder::new(self.input)?);
                option.as_mut().unwrap()
            }
        };

        inner.add_item(item)?;
        if env::var("POLYGEN") != Ok("0".to_string()) {
            inner.flush_to_disk(self.output)?;
        }

        Ok(())
    }
}

struct InnerBuilder {
    tera: Tera,
    structs: PolyMap<types::PolyStruct>,
}

impl InnerBuilder {
    pub fn new(input_glob: &str) -> Result<Self> {
        Ok(Self {
            tera: Tera::new(input_glob)?,
            structs: PolyMap::default(),
        })
    }

    pub fn add_item(&mut self, item: &Item) -> Result<()> {
        match item {
            syn::Item::Struct(s) => {
                let s: PolyStruct = s.into();
                self.structs.insert(s.name.clone(), s);
            }
            _ => return Err(anyhow::Error::msg("unsupported item")),
        }
        Ok(())
    }

    pub fn flush_to_disk(&self, output_dir: &str) -> Result<()> {
        let mut context = Context::new();
        context.insert("structs", self.structs.as_slice());

        let target_dir = PathBuf::from(output_dir);
        fs::create_dir_all(target_dir.clone())?;
        for name in self.tera.get_template_names() {
            let path = target_dir.join(name);
            if path.extension() == Some(OsStr::new("polygen")) {
                let path = path.with_extension("");
                let file = File::create(path)?;
                self.tera.render_to(name, &context, file)?;
            }
        }

        Ok(())
    }
}
