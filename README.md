## exactly

Numeric refinement types who's type automatically adjusts after operations.

## Example
```rust
use exactly::SetU8;

fn main() {
  // `foo` is known to be in the set {1, 2, 3} 
  let foo: SetU8![1, 2, 3] = SetU8::new(2).unwrap();

  // `bar` is known to be in the set {5, 6} 
  let bar: SetU8![5, 6] = SetU8::new(6).unwrap();

  // `baz` is known to be in the set {5, 10, 15, 6, 12, 18} 
  let baz: SetU8![5, 6, 10, 12, 15, 18] = foo * bar;

  assert_eq!(baz.inner(), 12);
}
```
