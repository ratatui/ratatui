# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/canvas.tape`
Output "target/canvas.gif"
Set Width 1200
Set Height 800
Hide
Type "cargo run --example=canvas --features=crossterm"
Enter
Sleep 1s
Show
Sleep 5s
