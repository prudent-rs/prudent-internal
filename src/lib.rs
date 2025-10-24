#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(doc, test)), no_std)]
#![cfg_attr(not(any(doc, test)), forbid(unsafe_code))]
//#[cfg(doc)]
//extern crate alloc;

#[macro_export]
macro_rules! unsafe_method {
    ($self:expr, $fn:expr $(, $arg:expr)+ ) => {
    };
}

#[macro_export]
macro_rules! unsafe_call {
    ($fn:expr $(, $arg:expr)+ ) => {
        // Enclosed in a block, so the result can be used as a value in an outer expression.
        {
            let tuple = $crate::unsafe_call!{~ $($arg),* };
            unsafe {
                $crate::unsafe_call! {~~
                    $fn,
                    tuple,
                    ( $( $arg ),* ),
                    (0)
                }
            }
        }
    };
    ($fn:expr) => {
        ::core::compile_error!("For now, we require the (potentially unsafe) function to have at least 1 argument.")
    };

    // Construct the tuple:
    (~ $first:expr, $($rest:expr),+ ) => {
        (
            $first, $crate::unsafe_call!{ ~ $($rest),+ }
        )
    };
    (~ $last:expr) => {
        ($last,)
    };
    // Commented out: For now, we require the (potentially unsafe) function to have at least 1 argument.
    //
    //(~) => { () };

    // Access tuple parts and call the function:
    (~~ $fn:expr, $tuple:ident,
     ( $_first_arg:expr, $($other_arg:expr),+ ),
     $( ( $($accessor_part:tt),+ ) ),*
    ) => {
        $crate::unsafe_call!{ ~~
            $fn, $tuple, ( $($other_arg),+ ),
            // Insert a new accessor to front (left): 0.
            (0),
            $(  // Prepend 1 to each supplied/existing accessor
                 ( 1, $($accessor_part),+ )
            ),*
        }
    };
    (~~ $fn:expr, $tuple:ident,
     ( $_last_or_only_arg:expr ),
     $( ( $($accessor_part:tt),+
        )
     ),*
    ) => {
        $fn( $(
                $crate::unsafe_call!{ ~~~ $tuple, $($accessor_part),+ }
             ),*
           )
    };

    // Expand an accessor group/list to access a field in the tuple:
    (~~~ $tuple:ident, $($accessor_part:tt),* ) => {
        $tuple $(. $accessor_part )*
    };
}

#[cfg(test)]
mod tests {
    unsafe fn unsafe_a(_: char, b: bool, _: u8, _: i32) -> bool {
        b
    }

    #[test]
    fn it_works() {
        //let tuple = unsafe_call!{~  'c', true, 1, -5 };
        unsafe_call!(unsafe_a, 'c', true, 1, -5);

        //unsafe fn f() {}
        //unsafe_call!( f);

        /*let args = ('c', (true, (1, (0,))));
        unsafe {
            let _ = unsafe_a(args.0, args.1.0, args.1.1.0, args.1.1.1.0);
        }
        */
    }
}
