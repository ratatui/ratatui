# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/list.tape`
Output "target/list.gif"
Set Width 1200
Set Height 600
Hide
Type "cargo run --example=list --features=crossterm"
Enter
Sleep 1s
Show
Down@1s 4
Up@1s 2
Left@1s 1
Sleep 5s
