# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/user_input.tape`
Output "target/user_input.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600
Hide
Type "cargo run --example=user_input --features=crossterm"
Enter
Sleep 1s
Show
Sleep 2s
Type e
Sleep 1s
Type "Hello, world!"
Enter
Sleep 2s
Backspace 13
Sleep 1s
Type "Goodbye, world!"
Enter
Sleep 5s
