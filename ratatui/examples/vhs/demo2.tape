# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/demo2.tape`
# NOTE: Requires VHS 0.6.1 or later for Screenshot support
Output "target/demo2.gif"
Set Theme "Aardvark Blue"
# The reason for this strange size is that the social preview image for this
# demo is 1280x64 with 80 pixels of padding on each side. We want a version
# without the padding for README.md, etc.
Set Width 1120
Set Height 480
Set Padding 0
Hide
Type "cargo run --example demo2 --features crossterm,widget-calendar"
Enter
Sleep 2s
Show
# About screen
Screenshot "target/demo2-about.png"
Sleep 3.5s
Down # Red eye
Sleep 0.4s
Down # black eye
Sleep 1s
Tab
# Recipe
Screenshot "target/demo2-recipe.png"
Sleep 1s
Set TypingSpeed 500ms
Down 7
Sleep 1s
Tab
# Email
Screenshot "target/demo2-email.png"
Sleep 2s
Down 4
Sleep 2s
Tab
# Trace route
Screenshot "target/demo2-trace.png"
Sleep 1s
Set TypingSpeed 200ms
Down 10
Sleep 2s
Tab
# Weather
Screenshot "target/demo2-weather.png"
Set TypingSpeed 100ms
Down 40
Sleep 2s
