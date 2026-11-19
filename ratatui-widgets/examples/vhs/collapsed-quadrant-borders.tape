# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./ratatui-widgets/examples/vhs/collapsed-quadrant-borders.tape`
Output "target/collapsed-quadrant-borders.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 800
Hide
Type "cargo run -p ratatui-widgets --example collapsed-quadrant-borders"
Enter
Sleep 2s
Show
Sleep 1s
Type "b"
Sleep 1s
Type "b"
Sleep 1s
Type "b"
Sleep 1s
Type "m"
Sleep 1s
Type "m"
Sleep 1s
Hide
Type "q"
