# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/layout.tape`
Output "target/layout.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 1410
Hide
Type "cargo run --example=layout --features=crossterm"
Enter
Sleep 1s
Show
Sleep 2s
