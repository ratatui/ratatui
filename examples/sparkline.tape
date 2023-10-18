# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/sparkline.tape`
Output "target/sparkline.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600
Hide
Type "cargo run --example=sparkline --features=crossterm"
Enter
Sleep 1s
Show
Sleep 5s
