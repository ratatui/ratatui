# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./examples/colors.tape`
Output "target/colors.gif"
# The OceanicMaterial theme is a good choice for this example (Obsidian is almost as good) because:
# - Black is dark and distinct from the default background
# - White is light and distinct from the default foreground
# - Normal and bright colors are distinct
# - Black and DarkGray are distinct
# - White and Gray are distinct
Set Theme "OceanicMaterial"
Set Width 1200
Set Height 1410
Hide
Type "cargo run --example=colors --features=crossterm"
Enter
Sleep 2s
Show
Sleep 1s
