# value-enum [![](https://img.shields.io/crates/v/value-enum.svg)](https://crates.io/crates/value-enum)

Macro for generating enums associated with values.

## Example

```Rust
use value_enum::value_enum;

value_enum!(
  char =>
  #[derive(Clone, Copy, PartialEq, Eq, Debug)]
  enum Abc {
    A = 'a',
    B = 'b',
    C = 'c',
  }
);

assert_eq!(
  char::from(Abc::A),
  'a'
);
assert_eq!(
  Abc::try_from('b'),
  Ok(Abc::B)
);
```
