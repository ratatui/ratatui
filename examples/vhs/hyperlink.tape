# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/hyperlink.tape`
Output "target/hyperlink.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 200
Hide
Type "cargo run --example=hyperlink --features=crossterm,unstable-widget-ref"
Enter
Sleep 3s
Show
Sleep 1s
Hide
Type "q"

