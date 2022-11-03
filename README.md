# Rust

## Attributes

1. Are actually defined by ECMA (?) and represent a unified way of creating metadata. (Reference)[https://doc.rust-lang.org/reference/attributes.html].

2. Useful when writing structs and enums, since you can automatically derive functions instead of writing them yourself.

```rust
#[derive(PartialEq)]
struct Point { x: i32, y: i32 }

let p1 = Point { x: 1, y: 2 }
let p2 = Point { x: 1, y: 2 }
p1 == p2 // true, without writing code for equality

struct Circle { c: Point, r: i32 }
let c1 = Circle { c: p1, r: 1 }
let c2 = Circle { c: p1, r: 1 }

c1 == c2 // invalid! don't know how to compare them
```

## Variables

1. Even though every variable is **immutable** by default, you can redeclare the same binding. This implies shadowing the previous declarations.

```rust
let mut a = 1;
a = 2; // valid, a is mutable

let b = 1;
b = 2; // invalid!

let c = "hello";
let c = 3; // valid, but value prev. referenced by c is lost
```
