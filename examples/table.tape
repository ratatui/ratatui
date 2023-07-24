# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/table.tape`
Output "target/table.gif"
Set Width 1200
Set Height 600
Hide
Type "cargo run --example=table --features=crossterm"
Enter
Sleep 1s
Show
Down@1s 4
Up@1s 2
Down@1s 8
Up@1s 12
Sleep 5s
