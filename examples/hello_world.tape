# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/hello_world.tape`
Output "target/hello_world.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 200
Hide
Type "cargo run --example=hello_world --features=crossterm"
Enter
Sleep 1s
Show
Sleep 5s
