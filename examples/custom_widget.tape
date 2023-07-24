# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/custom_widget.tape`
Output "target/custom_widget.gif"
Set Width 1200
Set Height 200
Hide
Type "cargo run --example=custom_widget --features=crossterm"
Enter
Sleep 1s
Show
Sleep 5s
