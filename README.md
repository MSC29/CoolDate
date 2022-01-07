# CoolDate

Cool Date rust lib

### dev

src/lib.rs is the lib
test/test.rs is a just a quick & dirty check whether the compiled lib runs properly

### build lib

```rust
cargo build --release
```

### create testable executable

```rust
cd test && rustc test.rs --extern lib=../target/release/libfoo.rlib && cd ..
./test/test
```
