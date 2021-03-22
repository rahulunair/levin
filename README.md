# levin

Find edit distance between two strings.


An example of Rust bindings for a C library and a python wrapper on top of that - purely for learning purposes


### How to test?

```bash
git clone https://github.com/rahulunair/levenshtein-rs/ && cd levenshtein-rs
cargo test
```

output:
```bash
running 7 tests
test tests::test_distance_by_1 ... ok
test tests::test_distance_by_2 ... ok
test tests::test_distance_empty ... ok
test tests::test_distance_empty_1 ... ok
test tests::test_distance_not_empty ... ok
test tests::test_distance_equal ... ok
test tests::test_distance_not_equal ... ok
```

### To build the python wheel

```bash
pip install maturin
maturin build
```

This will build the python package for levin.


## using python package

```bash
pip install levin
```

```python
from levin import dist
>> dist("rahul", "raul")
>> 1
```
## References
https://subscription.packtpub.com/book/application_development/9781838828103/10/ch10lvl1sec93/using-external-c-c-libraries-from-rust
 
