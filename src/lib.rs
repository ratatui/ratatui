#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

/// This macro creates an array of constraints.
///
/// # Syntax
///
/// The macro supports the following form:
/// - `constraints![$( $constraint:tt )+]`
/// - `constraints![$( $constraint:tt )+; $count:expr]`
///
/// Constraints are defined using a specific syntax:
/// - `== $token:tt / $token2:tt`: Sets a ratio constraint between two tokens.
/// - `== $token:tt %`: Sets a percentage constraint for the token.
/// - `>= $token:expr`: Sets a minimum size constraint for the token.
/// - `<= $token:expr`: Sets a maximum size constraint for the token.
/// - `== $token:expr`: Sets a fixed size constraint for the token.
///
/// # Examples
///
/// ```
/// use ratatui_macros::constraints;
/// assert_eq!(constraints![==5, ==30%, >=3, <=1, ==1/2].len(), 5);
/// assert_eq!(constraints![==5; 5].len(), 5);
/// ```
#[macro_export]
macro_rules! constraints {
    // Note: this implementation forgoes speed for the sake of simplicity. Adding variations of the
    // comma and semicolon rules for each constraint type would be faster, but would result in a lot
    // of duplicated code.

    // Cannot start the constraints macro with a ,
    ([ , $($rest:tt)* ] -> () []) => {
        compile_error!("No rules expected the token `,` while trying to match the end of the macro")
    };

    // Comma finishes a constraint element, so parse it and continue.
    ([ , $($rest:tt)* ] -> ($($partial:tt)*) [ $($parsed:tt)* ]) => {
        $crate::constraints!([$($rest)*] -> () [$($parsed)* $crate::constraint!($($partial)*) ,])
    };

    // Semicolon indicates that there's repetition. The trailing comma is because the 'entrypoint'
    // rule adds a trailing comma.
    ([ ; $count:expr , ] -> ($($partial:tt)*) []) => {
        [$crate::constraint!($($partial)*); $count]
    };

    // Pull the first token (which can't be a comma or semicolon) onto the accumulator.
    // if first token is a comma or semicolon, previous rules will match before this rule
    //
    // [ $head:tt $($rest:tt)* ]           -> In the rule matcher, this pulls a single `head` token
    //                                          out of the previous rest, and puts
    //                                          the remaining into `rest`
    // [ $($rest)* ]                       -> This is what is fed back into the `constraints!` macro
    //                                          as the first segment for the match rule
    //
    // ($($partial:tt)*)                   -> In the rule matcher, this contains previous partial
    //                                          tokens that will make up a `Constraint` expression
    // ($($partial)* $head)                -> This combines head with the previous partial tokens
    //                                          i.e. this is the accumulated tokens
    //
    // [ $($parsed:tt)* ]                  -> In the rule matcher, this contains all parsed exprs
    // [$($parsed)* ]                      -> These are passed on to the next match untouched.
    ([ $head:tt $($rest:tt)* ] -> ($($partial:tt)*) [ $($parsed:tt)* ]) => {
        $crate::constraints!([$($rest)*] -> ($($partial)* $head) [$($parsed)* ])
    };

    // No more input tokens; emit the parsed constraints.
    ([$(,)?]  -> () [ $( $parsed:tt )* ]) => {
        [$($parsed)*]
    };

    // Entrypoint where there's a no comma at the end
    // We add a comma to make sure there's always a trailing comma.
    // Right-hand side will accumulate the actual `Constraint` literals.
    ($( $constraint:tt )+) => {
        $crate::constraints!([ $($constraint)+ , ] -> () [])
    };
}

/// Expands to a single constraint. If creating an array of constraints, you probably want to use
/// [`constraints!`] instead.
///
/// # Syntax
///
/// Constraints are defined using a specific syntax:
/// - `== $token:tt / $token2:tt`: Sets a ratio constraint between two tokens.
/// - `== $token:tt %`: Sets a percentage constraint for the token.
/// - `>= $token:tt`: Sets a minimum size constraint for the token.
/// - `<= $token:tt`: Sets a maximum size constraint for the token.
/// - `== $token:tt`: Sets a fixed size constraint for the token.
///
/// # Examples
///
/// ```
/// use ratatui_macros::constraint;
/// use ratatui::prelude::Constraint;
/// assert_eq!(constraint!(>= 3 + 4), Constraint::Min(7));
/// assert_eq!(constraint!(== 1 / 3), Constraint::Ratio(1, 3));
/// ```
#[macro_export]
macro_rules! constraint {
  ( == $token:tt % ) => {
    ratatui::prelude::Constraint::Percentage($token)
  };
  ( >= $expr:expr ) => {
    ratatui::prelude::Constraint::Min($expr)
  };
  ( <= $expr:expr ) => {
    ratatui::prelude::Constraint::Max($expr)
  };
  ( == $num:tt / $denom:tt ) => {
    ratatui::prelude::Constraint::Ratio($num as u32, $denom as u32)
  };
  ( == $expr:expr ) => {
    ratatui::prelude::Constraint::Length($expr)
  };
}

/// Creates a vertical layout with specified constraints.
///
/// This macro is a convenience wrapper around the `layout!` macro for defining vertical layouts.
/// It accepts a series of constraints and applies them to create a vertical layout. The constraints
/// can include fixed sizes, minimum and maximum sizes, percentages, and ratios.
///
/// # Syntax
///
/// - `vertical![$( $constraint:tt )+]`: Defines a vertical layout with the given constraints.
///
/// # Constraints
///
/// Constraints are defined using a specific syntax:
/// - `== $token:tt / $token2:tt`: Sets a ratio constraint between two tokens.
/// - `== $token:tt %`: Sets a percentage constraint for the token.
/// - `>= $token:expr`: Sets a minimum size constraint for the token.
/// - `<= $token:expr`: Sets a maximum size constraint for the token.
/// - `== $token:expr`: Sets a fixed size constraint for the token.
///
/// # Examples
///
/// ```
/// // Vertical layout with a fixed size and a percentage constraint
/// use ratatui_macros::vertical;
/// vertical![== 50, == 30%];
/// ```
#[macro_export]
macro_rules! vertical {
    ($( $constraint:tt )+) => {
        ratatui::prelude::Layout::default()
            .direction(ratatui::prelude::Direction::Vertical)
            .constraints($crate::constraints!( $($constraint)+ ))
    };
}

/// Creates a horizontal layout with specified constraints.
///
/// This macro is a convenience wrapper around the `layout!` macro for defining horizontal layouts.
/// It takes a series of constraints and applies them to create a horizontal layout. The constraints
/// can include fixed sizes, minimum and maximum sizes, percentages, and ratios.
///
/// # Syntax
///
/// - `horizontal![$( $constraint:tt )+]`: Defines a horizontal layout with the given constraints.
///
/// # Constraints
///
/// Constraints are defined using a specific syntax:
/// - `== $token:tt / $token2:tt`: Sets a ratio constraint between two tokens.
/// - `== $token:tt %`: Sets a percentage constraint for the token.
/// - `>= $token:expr`: Sets a minimum size constraint for the token.
/// - `<= $token:expr`: Sets a maximum size constraint for the token.
/// - `== $token:expr`: Sets a fixed size constraint for the token.
///
/// # Examples
///
/// ```
/// // Horizontal layout with a ratio constraint and a minimum size constraint
/// use ratatui_macros::horizontal;
/// horizontal![== 1/3, >= 100];
/// ```
#[macro_export]
macro_rules! horizontal {
    ($( $constraint:tt )+) => {
        ratatui::prelude::Layout::default()
            .direction(ratatui::prelude::Direction::Horizontal)
            .constraints($crate::constraints!( $($constraint)+ ))
    };
}
