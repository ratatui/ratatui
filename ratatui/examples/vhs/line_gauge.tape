# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/line_gauge.tape`
Output "target/line_gauge.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 850
Hide
Type "cargo run -p ratatui-widgets --example line_gauge"
Enter
Sleep 1s
Show
Sleep 6s
