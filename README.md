# Polygen

Polygen is a polyglot binding generator for `rust`. It leverages the power of the type system to validate and generate bindings for your language of choice at compile time.

It is extensible via a powerful scripting engine, encouraging you to add or modify any bindings to your specification.

# Usage

Just tag an item you would like to export with `#[polygen]` and the rest will be taken care of!

```rust
use polygen::polygen;

#[polygen]
#[repr(C)]
struct NormalStruct {
    pub item: u32,
    another_item: bool,
    pub(crate) third_item: i64,
}

#[polygen]
extern "C" fn _cool_function(value: i8, normal_struct: NormalStruct) -> u32 {
    42
}

```

Using the c-sharp [generator](#generators) the above translates to:

```csharp
using System;
using System.Runtime.InteropServices;

public static partial class NativeLibrary
{
    public const string NativeLib = "native_library";

    [StructLayout(LayoutKind.Sequential)]
    private struct NormalStruct
    {
        public uint item;
        private bool anotherItem;
        private long thirdItem;
    }

    [DllImport(NativeLib, EntryPoint = "_cool_function", CallingConvention = CallingConvention.Cdecl)]
    private static extern uint CoolFunction(sbyte value, NormalStruct normalStruct);
}
```

# Generators
Polygen utilizes the [rhai scripting language](rhai.rs) to generate bindings. This makes it extensible in any way you like. Either modify existing scripts, or create a generator for a whole new language youself! No pull request needed!
