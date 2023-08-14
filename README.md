# Polygen

Polygen is a polyglot binding generator for `rust`. It leverages the power of the type system to validate and generate bindings for your language of choice at compile time.

It is extensible via a powerful scripting engine, encouraging you to add or modify any bindings to your specification and provide compile time errors if something is unsupported.

# Usage

Just tag an item you would like to export with `#[polygen]` and the rest will be taken care of!

```rust
use polygen::polygen;

#[polygen]
#[repr(C)]
pub struct NormalStruct {
    item: u32,
    another_item: bool,
}

#[polygen]
#[no_mangle]
pub extern "C" fn create_struct() -> NormalStruct {
    NormalStruct {
        item: 42,
        another_item: true,
    }
}

#[polygen]
#[no_mangle]
pub extern "C" fn get_item(normal_struct: &NormalStruct) -> u32 {
    normal_struct.item
}

```

Using the c-sharp [generator](#generators) the above translates to:

```csharp
using System;
using System.Runtime.InteropServices;

public static partial class ComplexLib
{
    public const string NativeLib = "simple_lib";

    [StructLayout(LayoutKind.Sequential)]
    public partial struct NormalStruct
    {
        private uint item;
        private bool anotherItem;
    }

    [DllImport(NativeLib, EntryPoint = "create_struct", CallingConvention = CallingConvention.Cdecl)]
    public static extern NormalStruct CreateStruct();

    [DllImport(NativeLib, EntryPoint = "get_item", CallingConvention = CallingConvention.Cdecl)]
    public static extern uint GetItem(ref readonly NormalStruct normalStruct);
}

```

# Generators
Polygen utilizes the [rhai scripting language](rhai.rs) to generate bindings. This makes it extensible in any way you like. Either modify existing scripts, or create a generator for a whole new language youself! No pull request needed!

To get started, check out the [existing library of generators](./generators/)!
