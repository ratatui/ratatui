# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/colors.tape`
Output "target/colors.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 1410
Hide
Type "cargo run --example=colors --features=crossterm"
Enter
Sleep 2s
Show
Sleep 1s
