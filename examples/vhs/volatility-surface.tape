# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/vhs/volatility-surface.tape`
Output "target/volatility-surface.gif"
Set Theme "Aardvark Blue"
Set Width 1080
Set Height 720
Set Framerate 18
Hide
Type "cargo run -p volatility-surface"
Enter
Sleep 2s
Show
# Show initial 3D surface
Sleep 1.5s
Screenshot "target/volatility-surface-1.png"
# Rotate around
Sleep 0.5s
Right@400ms 3
Sleep 0.5s
Up@400ms 2
Sleep 0.5s
Screenshot "target/volatility-surface-2.png"
# Zoom
Sleep 0.5s
Type "z"
Sleep 0.3s
Type "z"
Sleep 0.5s
# Pause
Type " "
Sleep 0.8s
Screenshot "target/volatility-surface-3.png"
# Resume
Type " "
Sleep 1s
Type "q"
