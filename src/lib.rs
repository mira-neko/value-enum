//! Macro for generating enums associated with values.
//! 
//! # Example
//! 
//! ```
//! use value_enum::value_enum;
//! 
//! value_enum!(
//!     #[derive(Clone, Copy, PartialEq, Eq, Debug)]
//!     enum Abc: char {
//!         A = 'a',
//!         B = 'b',
//!         C = 'c',
//!     }
//! );
//! 
//! assert_eq!(char::from(Abc::A), 'a');
//! assert_eq!(Abc::try_from('b'), Ok(Abc::B));
//! ```

/// Macro for generating enums associated with values.
/// 
/// # Example
/// 
/// ```
/// use value_enum::value_enum;
/// 
/// value_enum!(
///     #[derive(Clone, Copy, PartialEq, Eq, Debug)]
///     enum Abc: char {
///         A = 'a',
///         B = 'b',
///         C = 'c',
///     }
/// );
/// 
/// assert_eq!(char::from(Abc::A), 'a');
/// assert_eq!(Abc::try_from('b'), Ok(Abc::B));
/// ```
#[macro_export]
macro_rules! value_enum {
    (
        $(#[$attr:meta])*
        $vis:vis enum $name:ident: $type:ty {
            $($variant:ident = $value:expr),*
            $(,)?
        }
    ) => {
        $(#[$attr])*
        $vis enum $name {
            $($variant,)*
        }

        impl From<$name> for $type {
            fn from(val: $name) -> Self {
                match val {
                    $($name::$variant => $value,)*
                }
            }
        }

        impl TryFrom<$type> for $name {
            type Error = $type;

            #[allow(non_upper_case_globals)]
            fn try_from(val: $type) -> Result<Self, Self::Error> {
                $(
                    const $variant: $type = $value;
                )*
                match val {
                    $($variant => Ok($name::$variant),)*
                    _ => Err(val),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    value_enum!(
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        enum Abc: char {
            A = 'a',
            B = 'b',
            C = 'c',
        }
    );

    value_enum!(
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        enum Str: (&'static str, &'static str) {
            Test = ("test", "qwerty"),
            Nya = ("nya", "nyan"),
        }
    );

    #[test]
    fn test_char() {
        assert_eq!(char::from(Abc::A), 'a');
        assert_eq!(Abc::try_from('a'), Ok(Abc::A));
        assert_eq!(char::from(Abc::B), 'b');
        assert_eq!(Abc::try_from('b'), Ok(Abc::B));
        assert_eq!(char::from(Abc::C), 'c');
        assert_eq!(Abc::try_from('c'), Ok(Abc::C));
        assert_eq!(Abc::try_from('d'), Err('d'));
    }

    #[test]
    fn test_str() {
        assert_eq!(<(&str, &str)>::from(Str::Test), ("test", "qwerty"));
        assert_eq!(Str::try_from(("test", "qwerty")), Ok(Str::Test));
        assert_eq!(<(&str, &str)>::from(Str::Nya), ("nya", "nyan"));
        assert_eq!(Str::try_from(("nya", "nyan")), Ok(Str::Nya));
        assert_eq!(Str::try_from(("wtf", "wtf")), Err(("wtf", "wtf")));
    }
}
