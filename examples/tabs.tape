# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/tabs.tape`
Output "target/tabs.gif"
Set Width 1200
Set Height 300
Hide
Type "cargo run --example=tabs --features=crossterm"
Enter
Sleep 1s
Show
Right@1s 4
Left@1s 2
Sleep 5s
