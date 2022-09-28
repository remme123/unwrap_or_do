//! Unwrap the [`Result`] or do the expression
//!
//! Associated macros: unwrap_or_break, unwrap_or_continue, unwrap_or_log

#![cfg_attr(not(feature = "std"), no_std)]

/// Unwrap [`Result`] or do the expression
#[macro_export]
macro_rules! unwrap_or_do {
    ($expr:expr, $($fmt:tt)+) => {
        match $expr {
            Ok(val) => val,
            #[allow(unused_variables)]
            Err(e) => {
                #[cfg(feature = "std")]
                println!("Error: {}", e);

                #[allow(unreachable_code)]
                {
                    return $($fmt)+;
                }

                // {
                //     $($fmt)+
                // };
                //
                // {
                //     return Err(e);
                // }
            }
        }
    };
}

/// Unwrap [`Result`] or break the loop
#[macro_export]
macro_rules! unwrap_or_break {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            #[allow(unused_variables)]
            Err(e) => {
                #[cfg(feature = "std")]
                println!("Error: {}", e);

                break;
            }
        }
    };
}

/// Unwrap [`Result`] or break the loop
#[macro_export]
macro_rules! unwrap_or_continue {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            #[allow(unused_variables)]
            Err(e) => {
                #[cfg(feature = "std")]
                println!("Error: {}", e);

                continue;
            }
        }
    };
}

/// Unwrap [`Result`] or return the function after logging the [`Err`]
#[cfg(feature = "std")]
#[macro_export]
macro_rules! unwrap_or_log {
    ($expr:expr, $($fmt:tt)+) => {
        match $expr {
            Ok(val) => val,
            Err(e) => {
                print!($($fmt)+);
                println!(" (Error: {})", e);
                return Err(e.into());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unwrap_ok() -> Result<(), i32> {
        let a: Result<i32, i32> = Ok(1);
        assert_eq!(
            unwrap_or_do!(a, Err(-1)),
            1
        );
        Ok(())
    }

    #[test]
    fn unwrap_err() {
        let foo = || -> i32 {
            let a: Result<i32, i32> = Err(2);
            unwrap_or_do!(a, {
                return -2;
            })
        };
        assert_eq!(foo(), -2)
    }

    #[test]
    fn unwrap_break() {
        let mut n = 0;
        let vec: [Result<i32, i32>; 4] = [Ok(-1), Ok(-2), Err(-3), Err(-4)];
        for v in vec {
            unwrap_or_break!(v);
            n += 1;
        }
        assert_eq!(n, 2);
    }

    #[test]
    fn unwrap_continue() {
        let mut n = 0;
        let vec: [Result<i32, i32>; 6] = [Err(-1), Ok(-2), Err(-3), Ok(-4), Err(-5), Ok(-6)];
        for v in vec {
            unwrap_or_continue!(v);
            n += 1;
        }
        assert_eq!(n, 3);
    }
}
