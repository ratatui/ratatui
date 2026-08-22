# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/vhs/large-table.tape`
Output "target/large-table.gif"
Set Theme "Aardvark Blue"
Set Width 1400
Set Height 768
Hide
Type "cargo run -p large-table -- -n 100"
Enter
Sleep 4s
Show
Set TypingSpeed 1s
Down 6
Sleep 1s
PageDown
Sleep 2s
PageDown
Sleep 2s
PageUp
Sleep 2s
Down 10
Sleep 2s
Left 2
Sleep 1s
Shift+L
Sleep 1s
Shift+H
Sleep 2s
