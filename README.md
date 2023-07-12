### Building

```
# adding build target with rustup, likely require mingw as well
rustup target add i686-pc-windows-gnu
# debug build
cargo build --target i686-pc-windows-gnu
# release build
cargo build --target i686-pc-windows-gnu -r
```

### Testing on wine after building

```
wine target/i686-pc-windows-gnu/debug/reqwest_wine_tls_test.exe
```
