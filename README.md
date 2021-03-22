# levin

Find edit distance between two strings.


An example of Rust bindings for a C library and a python wrapper on top of that - purely for learning purposes


### How to test?

```bash
git clone https://github.com/rahulunair/levenshtein-rs/ && cd levenshtein-rs
cargo build
cargo run --example hello
```

output:
```bash
   Compiling levinstien-rs v0.1.0 (Coding/levenshtein-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 2.46s
     Running `target/debug/examples/hello`
distance is :: 1
```

### To build the python wheel

```bash
pip install maturin
maturin build
```

This will build the python package for levin.


## References
https://subscription.packtpub.com/book/application_development/9781838828103/10/ch10lvl1sec93/using-external-c-c-libraries-from-rust
 
