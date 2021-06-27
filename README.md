# tacview-splitter-rust

## TODO / questions
```
   Compiling tacview-splitter v0.1.0 (/home/timeshifter/rust/tacview-splitter)
error[E0597]: `body` does not live long enough
  --> src/main.rs:26:56
   |
26 |     let bodies_by_coalition = divide_body_by_coalition(&body);
   |                               -------------------------^^^^^-
   |                               |                        |
   |                               |                        borrowed value does not live long enough
   |                               argument requires that `body` is borrowed for `'static`
...
36 | }
   | - `body` dropped here while still borrowed
```

Trait implementation for struct templates?

