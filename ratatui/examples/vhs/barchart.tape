# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/barchart.tape`
Output "target/barchart.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 600
Hide
Type "cargo run --example=barchart"
Enter
Sleep 1s
Show
Sleep 1s
