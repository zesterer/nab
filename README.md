# Nab

A macro that makes extracting elements from enum variants simpler.

```rust
enum Foo {
    A(i32, bool),
    B(bool, u64),
}

let input = Foo::A(42, false);
let output = nab!(input, Foo::A(x, y) => (x, y));
assert_eq!(output, Some((42, false)));
```
