# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/ratatui-logo.tape`
Output "target/ratatui-logo.gif"
Set Theme "Aardvark Blue"
Set Width 550
Set Height 220
Hide
Type "cargo run --example=ratatui-logo --features=crossterm"
Enter
Sleep 2s
Show
Sleep 2s
Hide
Escape
