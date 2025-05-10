fn main() {
    #[cfg(all(feature = "crossterm_0_28", feature = "crossterm_0_29"))]
    compile_error!(
        "Both crossterm_0_28 and crossterm_0_29 features are enabled. Please enable only one."
    );

    #[cfg(not(any(feature = "crossterm_0_28", feature = "crossterm_0_29")))]
    compile_error!(
        "Neither crossterm_0_28 nor crossterm_0_29 features are enabled. Please enable one."
    );
}
