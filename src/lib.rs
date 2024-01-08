#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

/// This macro creates an iterator of constraints.
///
/// # Syntax
///
/// The macro supports the following form:
/// - `constraints!([$( $constraint:tt )+])`
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
/// use ratatui_macros::constraints;
/// constraints!([==5, ==30%, >=3, <=1, ==1/2]);
/// constraints!([==5; 5]);
/// ```
///
/// # Internal Implementation
///
/// - `@parse`: Internal rule to parse and accumulate constraints.
/// - `@process`: Internal rule to convert tokens into constraints.
///
/// This macro simplifies the process of creating various constraints.
#[macro_export]
macro_rules! constraints {
    // Rules for parsing one constraint at a time.

    // Entrypoint; we add a comma to make sure there's always a trailing comma. Right-hand side
    // will accumulate the actual Constraint literals.
    ([ $( $constraint:tt )+ ]) => {
        $crate::constraints!([ $($constraint)+ , ] -> () [])
    };

    // No more input tokens; emit the parsed constraints.
    ([$(,)?]  -> () [ $( $parsed:tt )* ]) => {
        [$($parsed)*]
    };

    ([ , $($rest:tt)* ] -> ($($partial:tt)*) [ $($parsed:tt)* ]) => {
        $crate::constraints!([$($rest)*] -> () [$($parsed)* $crate::constraint!($($partial)*) ,])
    };

    // the comma here is because we always have a trailing comma
    ([ ; $count:expr , ] -> ($($partial:tt)*) []) => {
        [$crate::constraint!($($partial)*); $count]
    };

    ([ $head:tt $($rest:tt)* ] -> ($($partial:tt)*) [ $($parsed:tt)* ]) => {
        $crate::constraints!([$($rest)*] -> ($($partial)* $head) [$($parsed)* ])
    };
}

#[macro_export]
macro_rules! constraint {
    ( == $token:tt % ) => { ratatui::prelude::Constraint::Percentage($token) };
    ( >= $expr:expr ) => { ratatui::prelude::Constraint::Min($expr) };
    ( <= $expr:expr ) => { ratatui::prelude::Constraint::Max($expr) };
    ( == $num:tt / $denom:tt ) => { ratatui::prelude::Constraint::Ratio($num as u32, $denom as u32) };
    ( == $expr:expr ) => { ratatui::prelude::Constraint::Length($expr) };
}

/// This macro creates a layout with specified constraints and direction.
///
/// # Syntax
///
/// The macro supports three main forms:
/// - `layout!([$( $constraint:tt )+])`: Defines a default layout (vertical) with constraints.
/// - `layout!([$( $constraint:tt )+], direction = h)`: Defines a horizontal layout with
///   constraints.
/// - `layout!([$( $constraint:tt )+], direction = v)`: Defines a vertical layout with constraints.
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
/// // Vertical layout with fixed size and percentage constraints
/// use ratatui_macros::layout;
/// layout!([== 50, == 30%], direction = v);
/// ```
///
/// ```
/// // Horizontal layout with ratio and minimum size constraints
/// use ratatui_macros::layout;
/// layout!([== 1/3, >= 100, <=4], direction = h);
/// ```
///
/// # Internal Implementation
///
/// - `@construct`: Internal rule to construct the final Layout with the specified direction and
///   constraints.
///
/// This macro simplifies the process of creating complex layouts with various constraints.
#[macro_export]
macro_rules! layout {
    // Horizontal layout variant
    ([ $( $constraint:tt )+ ], direction = h) => {
        // use internal `constraint!(@parse ...)` rule directly since it will always be an iterator
        $crate::layout!(@construct ratatui::prelude::Direction::Horizontal, $crate::constraints!( [ $($constraint)+ ]))
    };
    // Vertical layout variant
    ([ $( $constraint:tt )+ ], direction = v) => {
        // use internal `constraint!(@parse ...)` rule directly since it will always be an iterator
        $crate::layout!(@construct ratatui::prelude::Direction::Vertical, $crate::constraints!( [ $($constraint)+ ] ))
    };
    // Construct the final `Layout` object
    (@construct $direction:expr, $constraints:expr) => {
        ratatui::prelude::Layout::default()
            .direction($direction)
            .constraints($constraints)
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
/// - `vertical!([$( $constraint:tt )+])`: Defines a vertical layout with the given constraints.
///
/// # Constraints
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
/// // Vertical layout with a fixed size and a percentage constraint
/// use ratatui_macros::vertical;
/// vertical!([== 50, == 30%]);
/// ```
#[macro_export]
macro_rules! vertical {
    ([ $( $constraint:tt )+ ]) => {
        $crate::layout!([ $( $constraint )+ ], direction = v)
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
/// - `horizontal!([$( $constraint:tt )+])`: Defines a horizontal layout with the given constraints.
///
/// # Constraints
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
/// // Horizontal layout with a ratio constraint and a minimum size constraint
/// use ratatui_macros::horizontal;
/// horizontal!([== 1/3, >= 100]);
/// ```
#[macro_export]
macro_rules! horizontal {
    ([ $( $constraint:tt )+ ]) => {
        $crate::layout!([ $( $constraint )+ ], direction = h)
    };
}
