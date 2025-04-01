# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/vhs/table-detail-tea.tape`
Output "target/table-detail-tea.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 768
Hide
Type "cargo run -p table-detail-tea"
Enter
Sleep 1s
Show
Sleep 2s
Set TypingSpeed 1s
Down 3
Up 1
Sleep 1s
Down 3
Sleep 1s
Up 1
Sleep 1s
