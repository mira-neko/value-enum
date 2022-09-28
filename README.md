# value-enum [![](https://img.shields.io/crates/v/value-enum.svg)](https://crates.io/crates/value-enum) [![docs.rs](https://img.shields.io/docsrs/value-enum)](https://docs.rs/value-enum) ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/miralushch/value-enum) ![Crates.io](https://img.shields.io/crates/d/value-enum) ![Crates.io](https://img.shields.io/crates/l/value-enum)

Macro for generating enums associated with values.

## Example

```Rust
use value_enum::value_enum;

value_enum!(
  #[derive(Clone, Copy, PartialEq, Eq, Debug)]
  enum Abc: char {
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
