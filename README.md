# levenshtein-rs

An example of Rust bindings for a C library and a python wrapper on top of that - purely for learning purposes


### How to test?

```bash
git clone https://github.com/rahulunair/levenshtein-rs/ && cd levenshtein-rs
cargo build
mv liblevenshtein.so.a liblevenshtein.so
cargo run --example hello
```

output:
```bash
   Compiling levinstien-rs v0.1.0 (Coding/levenshtein-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 2.46s
     Running `target/debug/examples/hello`
distance is :: 1
```
