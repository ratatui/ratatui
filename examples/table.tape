# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/table.tape`
Output "target/table.gif"
Set Width 1600
Set Height 864
Hide
Type "cargo run --example=table --features=crossterm"
Enter
Sleep 1s
Show
Sleep 2s
Set TypingSpeed 0.75s
Down 2
Up 2
Down 2
Right 1
Down 2
Right 1
Down 2
Right 1
Down 2
Right 1
Left 3
Up 3
Right 1
Up 3
Right 1
Set TypingSpeed 0.4s
Up 7
Sleep 2s
