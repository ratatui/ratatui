# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/colors_rgb.tape`

# note that this script sometimes results in the gif having screen tearing
# issues. I'm not sure why, but it's not a problem with the library.
Output "target/colors_rgb.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 1200

# unsure if these help the screen tearing issue, but they don't hurt
Set Framerate 60
Set CursorBlink false

Hide
Type "cargo run --example=colors_rgb --features=crossterm --release"
Enter
Sleep 2s
# Screenshot "target/colors_rgb.png"
Show
Sleep 10s
