use strum::{Display, EnumString};

/// Option for layout flex justify content
///
/// This controls how the space is distributed when the constraints are satisfied.
#[derive(Copy, Debug, Default, Display, EnumString, Clone, Eq, PartialEq, Hash)]
pub enum Flex {
    #[default]
    StretchLast, /* Always fill available space within the container putting excess into last
                  * element */
    Stretch,      // Always fill available space within the container
    Start,        // Align items to the start of the container
    End,          // Align items to the end of the container
    Center,       // Center items within the container
    SpaceBetween, // Add excess space between each element
    SpaceAround,  // Add excess space around each element
}
#[cfg(test)]
mod tests {}
