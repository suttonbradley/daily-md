# Build issues and how to fix


1. When you get this error about macro expansion in `syn`:
```Compiling wasm-bindgen-macro-support v0.2.83
error[E0433]: failed to resolve: could not find `parse_quote_spanned` in syn
```
delete your Cargo.lock and rebuild.
