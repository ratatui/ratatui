# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/vhs/modifiers.tape`
Output "target/modifiers.gif"
Set Theme "Aardvark Blue"
Set FontSize 20
Set Padding 20
Set Width 1000
Set Height 700
Env NO_COLOR ""
Hide
Type "cargo run -p modifiers"
Enter
Wait+Screen /Foreground colors/
Show
Sleep 2s
Right@2s 4
Sleep 2s
Tab
Sleep 2s
