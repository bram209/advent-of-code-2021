macro_rules! test_cases {
    ($($name:ident: $func:ident($input:expr) $operator:ident $expected:expr,)*) => {
    $(
        test_cases!($name: $func($input) $operator $expected);
    )*
    };

    ($name:ident: $func:ident($input:expr) matches $expected:expr) => (
        paste::paste! {
            #[test]
            fn [<$func _ $name>]() {
                matches!($func($input), $expected);
            }
        }
    );

    ($name:ident: $func:ident($input:expr) equals $expected:expr) => (
        paste::paste! {
            #[test]
            fn [<$func _ $name>]() {
                assert_eq!($func($input), $expected);
            }
        }
    );

    ($name:ident: $func:ident($input:expr) ok_and_equals $expected:expr) => (
        paste::paste! {
            #[test]
            fn [<$func _ $name>]() {
                let result: anyhow::Result<_> = $func($input);
                matches!(result, Ok(..));
                assert_eq!(result.unwrap(), $expected);
            }
        }
    )
}

macro_rules! tests {
    ( $($e:tt)*) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::test_utils::test_cases;

            test_cases!($($e)*);
        }
    }
}

pub(crate) use test_cases;
pub(crate) use tests;
