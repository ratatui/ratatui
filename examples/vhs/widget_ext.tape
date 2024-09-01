# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/hello_world.tape`
Output "target/widget_ext.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 300
Set TypingSpeed 10ms
Type "cargo run --example=widget_ext --features=unstable-widget-ext"
Enter
Sleep 2s
Screenshot "target/widget_ext.png"
Sleep 1s
