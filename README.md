# Polygen

Polygen is a polyglot binding generator for rust. It leverages the power of the type system to validate and generate bindings for your language of choice at compile time.

## Usage

1. Just tag the items you would like to export with `#[polygen]`:

```rust
#[polygen]
pub struct MyStruct {
    item: u32,
    another_item: u64,
}

#[polygen]
impl MyStruct {
    pub fn new_with(item: u32) -> Self {
        Self {
            item,
            another_item: 42,
        }
    }
}

#[polygen]
pub fn create_boxed(item: MyStruct) -> PolyBox<MyStruct> {
    PolyBox::new(item)
}

#[polygen]
pub fn set_item(mut boxed: PolyBox<MyStruct>, item: u32) {
    boxed.item = item;
}
```

2. Then create a test that registers the items and generates the binding file:
    > 💡 notice that the only thing registered is the `create_boxed`/`set_items` functions and `MyStruct` impl. This is because generation is handled in a smart way where only what gets used ends up in the binding file. So since the functions use `PolyBox`, it will also be included in the final output.

```rust
static OUTPUT_DIR: &str = "target/polygen";

#[test]
fn bind() {
    // clear output folder
    let out_path = PathBuf::from(OUTPUT_DIR);
    if out_path.exists() {
        fs::remove_dir_all(out_path).unwrap();
    }

    // create the PolyBag
    let bag = PolyBag::new("Native")
        .register_impl::<MyStruct>()
        .register_function::<create_boxed>()
        .register_function::<set_item>();

    // render the csharp data to a file
    fs::create_dir_all(OUTPUT_DIR).unwrap();
    fs::write(
        PathBuf::from(OUTPUT_DIR).join("AllFeatures.cs"),
        CSharpRenderer {
            lib_name: "simple_lib".to_string(),
            namespace: "SimpleLib".to_string(),
        }
        .render(&bag),
    )
    .unwrap();
}
```

3. The final output should look like this:

```csharp
using System;
using System.Runtime.InteropServices;

namespace SimpleLib
{
    public static class Native
    {
        public class MyStruct
        {
            internal Data _data;
            public readonly ref Data data = ref _data;

            internal MyStruct(Data newData)
            {
                _data = newData;
            }

            [StructLayout(LayoutKind.Sequential)]
            public struct Data
            {
                internal uint item;
                internal ulong anotherItem;
            }

            [DllImport("simple_lib", CallingConvention = CallingConvention.Cdecl)]
            private static extern MyStruct.Data __polygen_implfn_new_with_h0ZSAp(uint item);
            private static MyStruct NewWith(uint item) => new MyStruct(__polygen_implfn_new_with_h0ZSAp(item))
        }

        [DllImport("simple_lib", CallingConvention = CallingConvention.Cdecl)]
        private static extern Polygen.PolyBox<MyStruct>.Data __polygen_fn_create_boxed_6uk4AT(MyStruct.Data item);
        public static Polygen.PolyBox<MyStruct> CreateBoxed(MyStruct item) => new Polygen.PolyBox<MyStruct>(__polygen_fn_create_boxed_6uk4AT(item._data))

        [DllImport("simple_lib", CallingConvention = CallingConvention.Cdecl)]
        private static extern void __polygen_fn_set_item_rbfCVI(Polygen.PolyBox<MyStruct>.Data boxed, uint item);
        public static void SetItem(Polygen.PolyBox<MyStruct> boxed, uint item) => __polygen_fn_set_item_rbfCVI(boxed._data, item)

        public static class Polygen
        {
            public class PolyBox
            {
                internal Data _data;
                public readonly ref Data data = ref _data;

                internal PolyBox(Data newData)
                {
                    _data = newData;
                }

                [StructLayout(LayoutKind.Sequential)]
                public struct Data
                {
                    internal nuint ptr;
                }
            }
        }
    }
}
```

### [MIT License](LICENSE.md)
