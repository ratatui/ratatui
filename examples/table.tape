# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/table.tape`
Output "target/table.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 800
Hide
Type "cargo run --example=table --features=crossterm"
Enter
Sleep 1s
Show
Sleep 2s
Set TypingSpeed 0.5s
Down 2
Up 2
Down 8
Up 12
Down 4
Sleep 2s
