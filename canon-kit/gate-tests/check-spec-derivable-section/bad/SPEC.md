# widget — SPEC

## Directory Structure

```text
src/
  widget.rs
  gadget.rs
  thing.rs
  parts/
    left.rs
    right.rs
lib/
  helper.rs
  shared.rs
```

## Behaviour

A widget is processed once, in arrival order.

## Public API

- ```rust
  pub fn one();
  pub fn two();
  pub fn three();
  pub fn four();
  ```
