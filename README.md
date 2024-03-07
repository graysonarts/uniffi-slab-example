# Using slab to send opaque types over Uniffi bindings

Read the article on dev.to [Opaque Types for UniFFI](https://dev.to/therustgarden/opaque-types-for-uniffi-1j9l)

## Get Everything ready to test

1. Run `build.ps1` (powershell) (or run the commands adjusting for not-windows)
2. Copy `opaque_type.dll` from `target/release` into `target/python`
3. cd into `target/python`
4. Run commands like in the example usage

## Example Usage of Generated Python Bindings

```python
import opaque_type;

opaque_type.ffi_initialize()
x = opaque_type.ffi_make_opaque(42)
y = opaque_type.ffi_make_opaque(23)
opaque_type.ffi_process_handle(x)
opaque_type.ffi_process_handle(y)
opaque_type.ffi_shutdown()
```
