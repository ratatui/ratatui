use strum::{Display, EnumString};

#[allow(unused_imports)]
use super::constraint::Constraint;

/// Defines the options for layout flex justify content in a container.
///
/// This enumeration controls the distribution of space when layout constraints are met.
///
/// - `StretchLast`: Fills the available space within the container, putting excess space into the
///   last element.
/// - `Stretch`: Always fills the available space within the container.
/// - `Start`: Aligns items to the start of the container.
/// - `End`: Aligns items to the end of the container.
/// - `Center`: Centers items within the container.
/// - `SpaceBetween`: Adds excess space between each element.
/// - `SpaceAround`: Adds excess space around each element.
#[derive(Copy, Debug, Default, Display, EnumString, Clone, Eq, PartialEq, Hash)]
pub enum Flex {
    /// Fills the available space within the container, putting excess space into the last
    /// constraint of the lowest priority. This matches the default behavior of ratatui and tui
    /// applications without [`Flex`]
    ///
    /// The following examples illustrate the allocation of excess in various combinations of
    /// constraints. As a refresher, the priorities of constraints are as follows:
    ///
    /// 1. [`Constraint::Fixed`]
    /// 2. [`Constraint::Min`] / [`Constraint::Max`]
    /// 3. [`Constraint::Length`] / [`Constraint::Percentage`] / [`Constraint::Ratio`]
    /// 4. [`Constraint::Proportional`]
    ///
    /// When every constraint is `Length`, the last element gets the excess.
    ///
    /// ```plain
    /// <----------------------------------- 80 px ------------------------------------>
    /// ┌──────20 px───────┐┌──────20 px───────┐┌────────────────40 px─────────────────┐
    /// │    Length(20)    ││    Length(20)    ││              Length(20)              │
    /// └──────────────────┘└──────────────────┘└──────────────────────────────────────┘
    ///                                         ^^^^^^^^^^^^^^^^ EXCESS ^^^^^^^^^^^^^^^^
    /// ```
    ///
    /// If we replace the constraint at the end with a `Fixed`, because it has a
    /// higher priority, the last constraint with the lowest priority, i.e. the last
    /// `Length` gets the excess.
    ///
    /// ```plain
    /// <----------------------------------- 80 px ------------------------------------>
    /// ┌──────20 px───────┐┌────────────────40 px─────────────────┐┌──────20 px───────┐
    /// │    Length(20)    ││              Length(20)              ││     Fixed(20)    │
    /// └──────────────────┘└──────────────────────────────────────┘└──────────────────┘
    ///                     ^^^^^^^^^^^^^^^^ EXCESS ^^^^^^^^^^^^^^^^
    /// ```
    ///
    /// Violating a `Max` is lower priority than `Fixed` but higher
    /// than `Length`.
    ///
    /// ```plain
    /// <----------------------------------- 80 px ------------------------------------>
    /// ┌────────────────40 px─────────────────┐┌──────20 px───────┐┌──────20 px───────┐
    /// │              Length(20)              ││      Max(20)     ││     Fixed(20)    │
    /// └──────────────────────────────────────┘└──────────────────┘└──────────────────┘
    /// ^^^^^^^^^^^^^^^^ EXCESS ^^^^^^^^^^^^^^^^
    /// ```
    ///
    /// It's important to note that while not violating a `Min` or `Max` constraint is
    /// prioritized higher than a `Length`, `Min` and `Max` constraints allow for a range
    /// of values and excess can (and will) be dumped into these ranges first, if possible,
    /// even if it not the last constraint.
    ///
    /// ```plain
    /// <----------------------------------- 80 px ------------------------------------>
    /// ┌──────20 px───────┐┌────────────────40 px─────────────────┐┌──────20 px───────┐
    /// │    Length(20)    ││                Min(20)               ││     Fixed(20)    │
    /// └──────────────────┘└──────────────────────────────────────┘└──────────────────┘
    ///                     ^^^^^^^^^^^^^^^^ EXCESS ^^^^^^^^^^^^^^^^
    ///
    /// <----------------------------------- 80 px ------------------------------------>
    /// ┌────────────────40 px─────────────────┐┌──────20 px───────┐┌──────20 px───────┐
    /// │                Min(20)               ││    Length(20)    ││     Fixed(20)    │
    /// └──────────────────────────────────────┘└──────────────────┘└──────────────────┘
    /// ^^^^^^^^^^^^^^^^ EXCESS ^^^^^^^^^^^^^^^^
    /// ```
    ///
    /// Proportional constraints have the lowest priority amongst all the constraints and hence
    /// will always take up any excess space available.
    ///
    /// ```plain
    /// <----------------------------------- 80 px ------------------------------------>
    /// ┌──────20 px───────┐┌──────20 px───────┐┌──────20 px───────┐┌──────20 px───────┐
    /// │  Proportional(0) ││      Min(20)     ││    Length(20)    ││     Fixed(20)    │
    /// └──────────────────┘└──────────────────┘└──────────────────┘└──────────────────┘
    /// ^^^^^^ EXCESS ^^^^^^
    /// ```
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    /// ┌───────────30 px────────────┐┌───────────30 px────────────┐┌──────20 px───────┐
    /// │       Percentage(20)       ││         Length(20)         ││     Fixed(20)    │
    /// └────────────────────────────┘└────────────────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌──────────────────────────60 px───────────────────────────┐┌──────20 px───────┐
    /// │                          Min(20)                         ││      Max(20)     │
    /// └──────────────────────────────────────────────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌────────────────────────────────────80 px─────────────────────────────────────┐
    /// │                                    Max(20)                                   │
    /// └──────────────────────────────────────────────────────────────────────────────┘
    /// ```
    #[default]
    StretchLast,

    /// Always fills the available space within the container.
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    /// ┌────16 px─────┐┌──────────────────44 px───────────────────┐┌──────20 px───────┐
    /// │Percentage(20)││                Length(20)                ││     Fixed(20)    │
    /// └──────────────┘└──────────────────────────────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌──────────────────────────60 px───────────────────────────┐┌──────20 px───────┐
    /// │                          Min(20)                         ││      Max(20)     │
    /// └──────────────────────────────────────────────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌────────────────────────────────────80 px─────────────────────────────────────┐
    /// │                                    Max(20)                                   │
    /// └──────────────────────────────────────────────────────────────────────────────┘
    /// ```
    Stretch,

    /// Aligns items to the start of the container.
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    /// ┌────16 px─────┐┌──────20 px───────┐┌──────20 px───────┐
    /// │Percentage(20)││    Length(20)    ││     Fixed(20)    │
    /// └──────────────┘└──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌──────20 px───────┐┌──────20 px───────┐
    /// │      Min(20)     ││      Max(20)     │
    /// └──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌──────20 px───────┐
    /// │      Max(20)     │
    /// └──────────────────┘
    /// ```
    Start,

    /// Aligns items to the end of the container.
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    ///                         ┌────16 px─────┐┌──────20 px───────┐┌──────20 px───────┐
    ///                         │Percentage(20)││    Length(20)    ││     Fixed(20)    │
    ///                         └──────────────┘└──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///                                         ┌──────20 px───────┐┌──────20 px───────┐
    ///                                         │      Min(20)     ││      Max(20)     │
    ///                                         └──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///                                                             ┌──────20 px───────┐
    ///                                                             │      Max(20)     │
    ///                                                             └──────────────────┘
    /// ```
    End,

    /// Centers items within the container.
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    ///             ┌────16 px─────┐┌──────20 px───────┐┌──────20 px───────┐
    ///             │Percentage(20)││    Length(20)    ││     Fixed(20)    │
    ///             └──────────────┘└──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///                     ┌──────20 px───────┐┌──────20 px───────┐
    ///                     │      Min(20)     ││      Max(20)     │
    ///                     └──────────────────┘└──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///                               ┌──────20 px───────┐
    ///                               │      Max(20)     │
    ///                               └──────────────────┘
    /// ```
    Center,

    /// Adds excess space between each element.
    ///
    /// # Examples
    ///
    /// ```plain
    /// 
    /// <------------------------------------80 px------------------------------------->
    /// ┌────16 px─────┐            ┌──────20 px───────┐            ┌──────20 px───────┐
    /// │Percentage(20)│            │    Length(20)    │            │     Fixed(20)    │
    /// └──────────────┘            └──────────────────┘            └──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌──────20 px───────┐                                        ┌──────20 px───────┐
    /// │      Min(20)     │                                        │      Max(20)     │
    /// └──────────────────┘                                        └──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    /// ┌────────────────────────────────────80 px─────────────────────────────────────┐
    /// │                                    Max(20)                                   │
    /// └──────────────────────────────────────────────────────────────────────────────┘
    /// ```
    SpaceBetween,

    /// Adds excess space around each element.
    ///
    /// # Examples
    ///
    /// ```plain
    /// <------------------------------------80 px------------------------------------->
    ///       ┌────16 px─────┐      ┌──────20 px───────┐      ┌──────20 px───────┐
    ///       │Percentage(20)│      │    Length(20)    │      │     Fixed(20)    │
    ///       └──────────────┘      └──────────────────┘      └──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///              ┌──────20 px───────┐              ┌──────20 px───────┐
    ///              │      Min(20)     │              │      Max(20)     │
    ///              └──────────────────┘              └──────────────────┘
    ///
    /// <------------------------------------80 px------------------------------------->
    ///                               ┌──────20 px───────┐
    ///                               │      Max(20)     │
    ///                               └──────────────────┘
    /// ```
    SpaceAround,
}
#[cfg(test)]
mod tests {}
