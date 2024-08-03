# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/docsrs.tape`
# NOTE: Requires VHS 0.6.1 or later for Screenshot support
Output "target/docsrs.gif"
Set Theme "Aardvark Blue"
# The reason for this strange size is that the social preview image for this
# demo is 1280x64 with 80 pixels of padding on each side. We want a version
# without the padding for README.md, etc.
Set Width 640
Set Height 160
Set Padding 0
Hide
Type "cargo run --example docsrs --features crossterm"
Enter
Sleep 2s
Show
Sleep 1s
Screenshot "target/docsrs-hello.png"
Sleep 1s
Hide
Type "q"
Type "cargo run --example docsrs --features crossterm -- layout"
Enter
Sleep 2s
Show
Sleep 1s
Screenshot "target/docsrs-layout.png"
Sleep 1s
Hide
Type "q"
Type "cargo run --example docsrs --features crossterm -- styling"
Enter
Sleep 2s
Show
Sleep 1s
Screenshot "target/docsrs-styling.png"
Sleep 1s
Hide
Type "q"
