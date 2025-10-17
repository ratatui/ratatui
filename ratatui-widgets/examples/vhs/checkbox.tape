# This is a vhs script. See https://github.com/charmbracelet/vhs for more info.
# To run this script, install vhs and run `vhs ./ratatui-widgets/examples/vhs/checkbox.tape`
# Note: The images/widget-examples directory is in the 'images' branch
# Create the directory first if running locally: mkdir -p images/widget-examples
Output "images/widget-examples/checkbox.gif"
Set Theme "Aardvark Blue"
Set Width 1600
Set Height 1200
Hide
Type "cargo run -p ratatui-widgets --example checkbox"
Enter
Sleep 10s
Show
Sleep 2s

# Row 0: Basic Checkbox - Notifications (starts checked)
# Deselect it
Space
Sleep 1s

# Select it back
Space
Sleep 1s

# Move down to Row 1: Styled Checkboxes
Down
Sleep 1s

# Row 1, Col 0: Success (checked) - deselect it
Space
Sleep 1s

# Move right to Error (unchecked)
Right
Sleep 1s

# Select Error
Space
Sleep 1s

# Move right to Info (checked)
Right
Sleep 1s

# Deselect Info
Space
Sleep 1s

# Move right to Warning (unchecked)
Right
Sleep 1s

# Select Warning
Space
Sleep 1s

# Move down to Row 2: Emoji & Unicode (3 checkboxes)
Down
Sleep 1s

# Row 2, Col 0: Emoji style (checked) - deselect it
Space
Sleep 1s

# Move right to Circle (unchecked)
Right
Sleep 1s

# Select Circle
Space
Sleep 1s

# Move right to Diamond (checked)
Right
Sleep 1s

# Deselect Diamond
Space
Sleep 1s

# Move down to Row 3: Custom Symbols
Down
Sleep 1s

# Row 3, Col 0: ASCII style (checked) - deselect it
Space
Sleep 1s

# Move right to Asterisk (unchecked)
Right
Sleep 1s

# Select Asterisk
Space
Sleep 1s

# Move right to Plus/Minus (checked, label says "Plus sign")
Right
Sleep 1s

# Deselect it (changes to "Minus sign")
Space
Sleep 1.5s

# Select it back (changes back to "Plus sign")
Space
Sleep 1.5s

# Move right to X/O style (unchecked)
Right
Sleep 1s

# Select X/O
Space
Sleep 1s

# Navigate back to show final state
Up
Sleep 0.5s
Up
Sleep 0.5s
Up
Sleep 0.5s

# Show final state
Sleep 3s

Hide
Escape
