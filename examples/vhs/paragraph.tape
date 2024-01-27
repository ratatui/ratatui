# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/paragraph.tape`
Output "target/paragraph.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 1800
Hide
Type "cargo run --example=paragraph --features=crossterm"
Enter
Sleep 1s
Show
Sleep 5s
