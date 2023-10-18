# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/inline.tape`
Output "target/inline.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600
Type "cargo run --example=inline --features=crossterm"
Enter
Sleep 20s
