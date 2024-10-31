# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/line_gauge.tape`
Output "target/line_gauge.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 850
Hide
Type "cargo run --example=line_gauge --features=crossterm"
Enter
Sleep 2s
Show
Sleep 2s
Enter 1
Sleep 15s
