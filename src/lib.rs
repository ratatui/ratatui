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
    // Entry rule for constraints
    // e.g. `[ ==100%, >=1, >=1 ]`
    ([ $( $constraint:tt )+ ]) => {
        // e.g. the tokens `==100%, >=1, >=1` are matched with @parse rules
        $crate::constraints!(@parse () $($constraint)+)
    };
    // Special case for `;`
    (@parse ($($acc:tt)*) ; $count:expr) => {
        $crate::constraints!(@process ($($acc)* ; $count))
    };
    // Internal parsing rules for constraints
    // This rule checks if `,` exists after the `head` token
    // e.g. acc: `==100`; head: `%`; `,`; tail: `>=1, >=1` will match this rule
    (@parse ($($acc:tt)*) $head:tt , $($tail:tt)*) => {
        // Combines the head constraint with the tail constraints into a vector
        std::iter::once(
            // e.g. (acc head): `==100%`; this can be processed as a `Constraint`
            $crate::constraints!(@process ($($acc)* $head)) // -> Constraint
        ).chain(
            // e.g. tail: `>=1, >=1`; this can be parsed as a `Iterator<type = Constraint>`
            $crate::constraints!(@parse () $($tail)*).into_iter() // -> Iterator with type Constraint
        )
    };
    // If there is no `,` then accumulate `next` token into existing `acc` tokens
    // and return `Iterator`
    // e.g. for tokens `==100%, >=1, >=1`
    // 1.   acc: ``      ; next: `=`        ; tail: `=100 %, >=1, >=1`    ; will match this rule
    //      acc: `=`     ;                    tail: `=100 %, >=1, >=1`    ; is the next parse
    // 2.   acc: `=`     ; next: `=`        ; tail: `100 %, >=1, >=1`     ; will match this rule again
    //      acc: `==`    ;                    tail: `100 %, >=1, >=1`     ; is the next parse
    // 3.   acc: `==`    ; next: `100`      ; tail: `%, >=1, >=1`         ; will match this rule again
    //      acc: `==100` ;                    tail: `%, >=1, >=1`         ; is the next parse
    // OR   acc: `==100` ; head: `%`; `,`;    tail: `>=1, >=1`            ; i.e. this is the match for the next parse
    //                                ^^^ --------------------------------> this will match previous rule because of this comma
    (@parse ($($acc:tt)*) $next:tt $($tail:tt)*) => {
        $crate::constraints!(@parse ($($acc)* $next) $($tail)*)
    };
    // At the end there will a set of tokens left after the last `,`
    // Process that as a `Constraint`
    (@parse ($($acc:tt)*)) => {
        [$crate::constraints!(@process ($($acc)*))]
    };
    // Process different types of constraints into a `Constraint`
    (@process (== $token1:tt / $token2:tt)) => {
        // Ratio constraint
        {
        let t1: u32 = $token1;
        let t2: u32 = $token2;
        ratatui::prelude::Constraint::Ratio(t1, t2)
        }
    };
    (@process (== $token:tt % ; $count:expr)) => {
        // Percentage constraint
        std::iter::repeat(ratatui::prelude::Constraint::Percentage($token)).take($count as usize)
    };
    (@process (== $token:tt %)) => {
        // Percentage constraint
        ratatui::prelude::Constraint::Percentage($token)
    };
    (@process (>= $token:expr; $count:expr)) => {
        // Percentage constraint
        std::iter::repeat(ratatui::prelude::Constraint::Min($token)).take($count as usize)
    };
    (@process (>= $token:expr)) => {
        // Minimum size constraint
        ratatui::prelude::Constraint::Min($token)
    };
    (@process (<= $token:expr; $count:expr)) => {
        // Percentage constraint
        std::iter::repeat(ratatui::prelude::Constraint::Max($token)).take($count as usize)
    };
    (@process (<= $token:expr)) => {
        // Maximum size constraint
        ratatui::prelude::Constraint::Max($token)
    };
    (@process (== $token:expr ; $count:expr)) => {
        std::iter::repeat(ratatui::prelude::Constraint::Length($token)).take($count as usize)
    };
    (@process (== $token:expr)) => {
        // Fixed size constraint
        ratatui::prelude::Constraint::Length($token)
    };
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
        $crate::layout!(@construct ratatui::prelude::Direction::Horizontal, $crate::constraints!(@parse () $($constraint)+))
    };
    // Vertical layout variant
    ([ $( $constraint:tt )+ ], direction = v) => {
        // use internal `constraint!(@parse ...)` rule directly since it will always be an iterator
        $crate::layout!(@construct ratatui::prelude::Direction::Vertical, $crate::constraints!(@parse () $($constraint)+))
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
