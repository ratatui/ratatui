# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/vhs/inline.tape`
Output "target/inline-resizable.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600
Type "cargo run -p inline-resizable"
Enter
Set TypingSpeed 250ms
Sleep 2s Type "iiiii"
Sleep 1s Type "+++++"
Sleep 1s Type "iiiii"
Sleep 1s Type "+++++"
Sleep 1s Type "iiiii"
Sleep 1s Type "-----"
Sleep 1s Type "iiiii"
Sleep 1s Type "+++++"
Sleep 1s Type "iiiii"
Sleep 1s Type "---------------"
Sleep 1s Type "iiiii"
Sleep 1s Type "+++++"
Sleep 2s
Type "q"
Sleep 1s
