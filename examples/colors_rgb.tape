# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/colors_rgb.tape`
Output "target/colors_rgb.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 800
Hide
Type "cargo run --example=colors_rgb --features=crossterm"
Enter
Sleep 2s
Screenshot "target/colors_rgb.png"
Show
Sleep 1s
