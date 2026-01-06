# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/vhs/volatility-surface.tape`
Output "target/volatility-surface.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 800
Hide
Type "cargo run -p volatility-surface"
Enter
Sleep 2s
Show
# Show initial 3D surface
Sleep 2s
Screenshot "target/volatility-surface-1.png"
# Rotate around
Sleep 1s
Right@500ms 3
Sleep 1s
Up@500ms 2
Sleep 1s
Screenshot "target/volatility-surface-2.png"
# Cycle palette
Sleep 1s
Type "p"
Sleep 1s
Type "p"
Sleep 1s
Screenshot "target/volatility-surface-3.png"
# Zoom
Type "z"
Sleep 0.5s
Type "z"
Sleep 1s
# Pause
Type " "
Sleep 1s
# Resume
Type " "
Sleep 2s
Type "q"
