# Using slab to send opaque types over Uniffi bindings

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
