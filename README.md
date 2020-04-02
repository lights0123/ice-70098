# Building
Make sure you're running on Rust nightly. Then, install
[cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild).
```bash
rustup component add rust-src
cargo install cargo-xbuild
```
Then, build it:
```bash
cargo xbuild --target armv5te-nspire-eabi.json --release
```

If you do get it to work, you will get a linker error. Unless you really
*want* to install the
[full toolchain](https://github.com/ndless-nspire/Ndless/wiki/Ndless-SDK:-C-and-assembly-development-introduction),
you can only test that the compiler does not error.
