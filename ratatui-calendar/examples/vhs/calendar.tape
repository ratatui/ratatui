# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./ratatui-calendar/examples/vhs/calendar.tape`
Output "target/calendar.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 800
Hide
Type "cargo run -p ratatui-calendar --example calendar"
Enter
Sleep 3s
Show
Sleep 5s
