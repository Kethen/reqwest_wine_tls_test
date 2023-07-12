### Building

```
# adding build target with rustup, likely require mingw as well
rustup target add i686-pc-windows-gnu
cargo build --target i686-pc-windows-gnu -r
```

### Testing on wine after building

```
wine target/i686-pc-windows-gnu/release/reqwest_wine_tls_test.exe
```
