#![feature(decl_macro)]

//!  Macro for generating enums associated with values.

/// Macro for generating enums associated with values.
/// 
/// Examples
/// 
/// ```
/// use value_enum::value_enum;
/// 
/// value_enum!(
///     char =>
///     #[derive(Clone, Copy, PartialEq, Eq, Debug)]
///     enum Abc {
///         A = 'a',
///         B = 'b',
///         C = 'c',
///     }
/// );
/// 
/// assert_eq!(char::from(Abc::A), 'a');
/// assert_eq!(Abc::try_from('b'), Ok(Abc::B));
/// ```
pub macro value_enum(
    $type:ty =>
    $(#[$attr:meta])*
    $vis:vis enum $name:ident {
        $($variant:ident = $value:literal),*
        $(,)?
    }
) {
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

        fn try_from(val: $type) -> Result<Self, Self::Error> {
            match val {
                $($value => Ok($name::$variant),)*
                _ => Err(val),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    value_enum!(
        char =>
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        enum Abc {
            A = 'a',
            B = 'b',
            C = 'c',
        }
    );

    value_enum!(
        &'static str =>
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        enum Str {
            Test = "test",
            Nya = "nya",
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
        assert_eq!(<&str>::from(Str::Test), "test");
        assert_eq!(Str::try_from("test"), Ok(Str::Test));
        assert_eq!(<&str>::from(Str::Nya), "nya");
        assert_eq!(Str::try_from("nya"), Ok(Str::Nya));
        assert_eq!(Str::try_from("wtf"), Err("wtf"));
    }
}
