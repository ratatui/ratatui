/// Define a color palette for use in Ratatui.
///
/// The colors are from https://tailwindcss.com/docs/customizing-colors
///
/// The following palettes are available for use:
///
/// - `SLATE`
/// - `GRAY`
/// - `ZINC`
/// - `NEUTRAL`
/// - `STONE`
/// - `RED`
/// - `ORANGE`
/// - `AMBER`
/// - `YELLOW`
/// - `LIME`
/// - `GREEN`
/// - `EMERALD`
/// - `TEAL`
/// - `CYAN`
/// - `SKY`
/// - `BLUE`
/// - `INDIGO`
/// - `VIOLET`
/// - `PURPLE`
/// - `FUCHSIA`
/// - `PINK`
/// - `ROSE`
///
/// This `palette!` macro can be used both in module and function scope.
///
/// # Examples
///
/// ```rust
/// use ratatui::prelude::Color;
/// use ratatui_macros::palette;
///
/// palette!(pub SLATE);
///
/// assert_eq!(SLATE_50, Color::Rgb(248, 250, 252));
/// assert_eq!(SLATE_900, Color::Rgb(15, 23, 42));
///
/// fn color() {
///   palette!(RED);
///
///   assert_eq!(RED_50, Color::Rgb(254, 242, 242));
///   assert_eq!(RED_900, Color::Rgb(127, 29, 29));
/// }
/// # color();
/// ```
///
/// The `palette!(pub SLATE)` macro expands to the following:
///
/// ```rust
/// # use ratatui::prelude::Color;
/// pub const SLATE_50: Color = Color::Rgb(248, 250, 252);
/// pub const SLATE_100: Color = Color::Rgb(241, 245, 249);
/// pub const SLATE_200: Color = Color::Rgb(226, 232, 240);
/// pub const SLATE_300: Color = Color::Rgb(203, 213, 225);
/// pub const SLATE_400: Color = Color::Rgb(148, 163, 184);
/// pub const SLATE_500: Color = Color::Rgb(100, 116, 139);
/// pub const SLATE_600: Color = Color::Rgb(71, 85, 105);
/// pub const SLATE_700: Color = Color::Rgb(51, 65, 85);
/// pub const SLATE_800: Color = Color::Rgb(30, 41, 59);
/// pub const SLATE_900: Color = Color::Rgb(15, 23, 42);
/// ```
#[macro_export]
macro_rules! palette {
  ($v:vis SLATE) => {
    #[allow(dead_code)]
    $v const SLATE_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(248, 250, 252);
    #[allow(dead_code)]
    $v const SLATE_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(241, 245, 249);
    #[allow(dead_code)]
    $v const SLATE_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(226, 232, 240);
    #[allow(dead_code)]
    $v const SLATE_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(203, 213, 225);
    #[allow(dead_code)]
    $v const SLATE_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(148, 163, 184);
    #[allow(dead_code)]
    $v const SLATE_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(100, 116, 139);
    #[allow(dead_code)]
    $v const SLATE_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(71, 85, 105);
    #[allow(dead_code)]
    $v const SLATE_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(51, 65, 85);
    #[allow(dead_code)]
    $v const SLATE_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(30, 41, 59);
    #[allow(dead_code)]
    $v const SLATE_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(15, 23, 42);
  };
  ($v:vis GRAY) => {
    #[allow(dead_code)]
    $v const GRAY_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(249, 250, 251);
    #[allow(dead_code)]
    $v const GRAY_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(243, 244, 246);
    #[allow(dead_code)]
    $v const GRAY_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(229, 231, 235);
    #[allow(dead_code)]
    $v const GRAY_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(209, 213, 219);
    #[allow(dead_code)]
    $v const GRAY_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(156, 163, 175);
    #[allow(dead_code)]
    $v const GRAY_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(107, 114, 128);
    #[allow(dead_code)]
    $v const GRAY_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(75, 85, 99);
    #[allow(dead_code)]
    $v const GRAY_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(55, 65, 81);
    #[allow(dead_code)]
    $v const GRAY_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(31, 41, 55);
    #[allow(dead_code)]
    $v const GRAY_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(17, 24, 39);
  };
  ($v:vis ZINC) => {
    #[allow(dead_code)]
    $v const ZINC_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(250, 250, 250);
    #[allow(dead_code)]
    $v const ZINC_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(244, 244, 245);
    #[allow(dead_code)]
    $v const ZINC_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(228, 228, 231);
    #[allow(dead_code)]
    $v const ZINC_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(212, 212, 216);
    #[allow(dead_code)]
    $v const ZINC_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(161, 161, 170);
    #[allow(dead_code)]
    $v const ZINC_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(113, 113, 122);
    #[allow(dead_code)]
    $v const ZINC_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(82, 82, 91);
    #[allow(dead_code)]
    $v const ZINC_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(63, 63, 70);
    #[allow(dead_code)]
    $v const ZINC_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(39, 39, 42);
    #[allow(dead_code)]
    $v const ZINC_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(24, 24, 27);
  };
  ($v:vis NEUTRAL) => {
    #[allow(dead_code)]
    $v const NEUTRAL_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(250, 250, 250);
    #[allow(dead_code)]
    $v const NEUTRAL_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(245, 245, 245);
    #[allow(dead_code)]
    $v const NEUTRAL_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(229, 229, 229);
    #[allow(dead_code)]
    $v const NEUTRAL_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(212, 212, 212);
    #[allow(dead_code)]
    $v const NEUTRAL_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(163, 163, 163);
    #[allow(dead_code)]
    $v const NEUTRAL_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(115, 115, 115);
    #[allow(dead_code)]
    $v const NEUTRAL_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(82, 82, 82);
    #[allow(dead_code)]
    $v const NEUTRAL_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(64, 64, 64);
    #[allow(dead_code)]
    $v const NEUTRAL_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(38, 38, 38);
    #[allow(dead_code)]
    $v const NEUTRAL_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(23, 23, 23);
  };
  ($v:vis STONE) => {
    #[allow(dead_code)]
    $v const STONE_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(250, 250, 249);
    #[allow(dead_code)]
    $v const STONE_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(245, 245, 244);
    #[allow(dead_code)]
    $v const STONE_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(231, 229, 228);
    #[allow(dead_code)]
    $v const STONE_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(214, 211, 209);
    #[allow(dead_code)]
    $v const STONE_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(168, 162, 158);
    #[allow(dead_code)]
    $v const STONE_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(120, 113, 108);
    #[allow(dead_code)]
    $v const STONE_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(87, 83, 78);
    #[allow(dead_code)]
    $v const STONE_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(68, 64, 60);
    #[allow(dead_code)]
    $v const STONE_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(41, 37, 36);
    #[allow(dead_code)]
    $v const STONE_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(28, 25, 23);
  };
  ($v:vis RED) => {
    #[allow(dead_code)]
    $v const RED_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(254, 242, 242);
    #[allow(dead_code)]
    $v const RED_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(254, 226, 226);
    #[allow(dead_code)]
    $v const RED_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(254, 202, 202);
    #[allow(dead_code)]
    $v const RED_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(252, 165, 165);
    #[allow(dead_code)]
    $v const RED_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(248, 113, 113);
    #[allow(dead_code)]
    $v const RED_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(239, 68, 68);
    #[allow(dead_code)]
    $v const RED_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(220, 38, 38);
    #[allow(dead_code)]
    $v const RED_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(185, 28, 28);
    #[allow(dead_code)]
    $v const RED_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(153, 27, 27);
    #[allow(dead_code)]
    $v const RED_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(127, 29, 29);
  };
  ($v:vis ORANGE) => {
    #[allow(dead_code)]
    $v const ORANGE_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(255, 247, 237);
    #[allow(dead_code)]
    $v const ORANGE_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(255, 237, 213);
    #[allow(dead_code)]
    $v const ORANGE_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(254, 215, 170);
    #[allow(dead_code)]
    $v const ORANGE_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(253, 186, 116);
    #[allow(dead_code)]
    $v const ORANGE_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(251, 146, 60);
    #[allow(dead_code)]
    $v const ORANGE_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(249, 115, 22);
    #[allow(dead_code)]
    $v const ORANGE_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(234, 88, 12);
    #[allow(dead_code)]
    $v const ORANGE_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(194, 65, 12);
    #[allow(dead_code)]
    $v const ORANGE_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(154, 52, 18);
    #[allow(dead_code)]
    $v const ORANGE_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(124, 45, 18);
  };
  ($v:vis AMBER) => {
    #[allow(dead_code)]
    $v const AMBER_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(255, 251, 235);
    #[allow(dead_code)]
    $v const AMBER_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(254, 243, 199);
    #[allow(dead_code)]
    $v const AMBER_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(253, 230, 138);
    #[allow(dead_code)]
    $v const AMBER_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(252, 211, 77);
    #[allow(dead_code)]
    $v const AMBER_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(251, 191, 36);
    #[allow(dead_code)]
    $v const AMBER_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(245, 158, 11);
    #[allow(dead_code)]
    $v const AMBER_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(217, 119, 6);
    #[allow(dead_code)]
    $v const AMBER_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(180, 83, 9);
    #[allow(dead_code)]
    $v const AMBER_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(146, 64, 14);
    #[allow(dead_code)]
    $v const AMBER_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(120, 53, 15);
  };
  ($v:vis YELLOW) => {
    #[allow(dead_code)]
    $v const YELLOW_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(254, 252, 232);
    #[allow(dead_code)]
    $v const YELLOW_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(254, 249, 195);
    #[allow(dead_code)]
    $v const YELLOW_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(254, 240, 138);
    #[allow(dead_code)]
    $v const YELLOW_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(253, 224, 71);
    #[allow(dead_code)]
    $v const YELLOW_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(250, 204, 21);
    #[allow(dead_code)]
    $v const YELLOW_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(234, 179, 8);
    #[allow(dead_code)]
    $v const YELLOW_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(202, 138, 4);
    #[allow(dead_code)]
    $v const YELLOW_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(161, 98, 7);
    #[allow(dead_code)]
    $v const YELLOW_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(133, 77, 14);
    #[allow(dead_code)]
    $v const YELLOW_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(113, 63, 18);
  };
  ($v:vis LIME) => {
    #[allow(dead_code)]
    $v const LIME_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(247, 254, 231);
    #[allow(dead_code)]
    $v const LIME_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(236, 252, 203);
    #[allow(dead_code)]
    $v const LIME_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(217, 249, 157);
    #[allow(dead_code)]
    $v const LIME_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(190, 242, 100);
    #[allow(dead_code)]
    $v const LIME_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(163, 230, 53);
    #[allow(dead_code)]
    $v const LIME_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(132, 204, 22);
    #[allow(dead_code)]
    $v const LIME_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(101, 163, 13);
    #[allow(dead_code)]
    $v const LIME_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(77, 124, 15);
    #[allow(dead_code)]
    $v const LIME_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(63, 98, 18);
    #[allow(dead_code)]
    $v const LIME_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(54, 83, 20);
  };
  ($v:vis GREEN) => {
    #[allow(dead_code)]
    $v const GREEN_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(240, 253, 244);
    #[allow(dead_code)]
    $v const GREEN_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(220, 252, 231);
    #[allow(dead_code)]
    $v const GREEN_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(187, 247, 208);
    #[allow(dead_code)]
    $v const GREEN_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(134, 239, 172);
    #[allow(dead_code)]
    $v const GREEN_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(74, 222, 128);
    #[allow(dead_code)]
    $v const GREEN_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(34, 197, 94);
    #[allow(dead_code)]
    $v const GREEN_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(22, 163, 74);
    #[allow(dead_code)]
    $v const GREEN_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(21, 128, 61);
    #[allow(dead_code)]
    $v const GREEN_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(22, 101, 52);
    #[allow(dead_code)]
    $v const GREEN_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(20, 83, 45);
  };
  ($v:vis EMERALD) => {
    #[allow(dead_code)]
    $v const EMERALD_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(236, 253, 245);
    #[allow(dead_code)]
    $v const EMERALD_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(209, 250, 229);
    #[allow(dead_code)]
    $v const EMERALD_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(167, 243, 208);
    #[allow(dead_code)]
    $v const EMERALD_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(110, 231, 183);
    #[allow(dead_code)]
    $v const EMERALD_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(52, 211, 153);
    #[allow(dead_code)]
    $v const EMERALD_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(16, 185, 129);
    #[allow(dead_code)]
    $v const EMERALD_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(5, 150, 105);
    #[allow(dead_code)]
    $v const EMERALD_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(4, 120, 87);
    #[allow(dead_code)]
    $v const EMERALD_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(6, 95, 70);
    #[allow(dead_code)]
    $v const EMERALD_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(6, 78, 59);
  };
  ($v:vis TEAL) => {
    #[allow(dead_code)]
    $v const TEAL_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(240, 253, 250);
    #[allow(dead_code)]
    $v const TEAL_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(204, 251, 241);
    #[allow(dead_code)]
    $v const TEAL_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(153, 246, 228);
    #[allow(dead_code)]
    $v const TEAL_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(94, 234, 212);
    #[allow(dead_code)]
    $v const TEAL_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(45, 212, 191);
    #[allow(dead_code)]
    $v const TEAL_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(20, 184, 166);
    #[allow(dead_code)]
    $v const TEAL_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(13, 148, 136);
    #[allow(dead_code)]
    $v const TEAL_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(15, 118, 110);
    #[allow(dead_code)]
    $v const TEAL_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(17, 94, 89);
    #[allow(dead_code)]
    $v const TEAL_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(19, 78, 74);
  };
  ($v:vis CYAN) => {
    #[allow(dead_code)]
    $v const CYAN_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(236, 254, 255);
    #[allow(dead_code)]
    $v const CYAN_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(207, 250, 254);
    #[allow(dead_code)]
    $v const CYAN_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(165, 243, 252);
    #[allow(dead_code)]
    $v const CYAN_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(103, 232, 249);
    #[allow(dead_code)]
    $v const CYAN_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(34, 211, 238);
    #[allow(dead_code)]
    $v const CYAN_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(6, 182, 212);
    #[allow(dead_code)]
    $v const CYAN_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(8, 145, 178);
    #[allow(dead_code)]
    $v const CYAN_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(14, 116, 144);
    #[allow(dead_code)]
    $v const CYAN_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(21, 94, 117);
    #[allow(dead_code)]
    $v const CYAN_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(22, 78, 99);
  };
  ($v:vis SKY) => {
    #[allow(dead_code)]
    $v const SKY_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(240, 249, 255);
    #[allow(dead_code)]
    $v const SKY_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(224, 242, 254);
    #[allow(dead_code)]
    $v const SKY_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(186, 230, 253);
    #[allow(dead_code)]
    $v const SKY_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(125, 211, 252);
    #[allow(dead_code)]
    $v const SKY_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(56, 189, 248);
    #[allow(dead_code)]
    $v const SKY_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(14, 165, 233);
    #[allow(dead_code)]
    $v const SKY_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(2, 132, 199);
    #[allow(dead_code)]
    $v const SKY_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(3, 105, 161);
    #[allow(dead_code)]
    $v const SKY_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(7, 89, 133);
    #[allow(dead_code)]
    $v const SKY_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(12, 74, 110);
  };
  ($v:vis BLUE) => {
    #[allow(dead_code)]
    $v const BLUE_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(239, 246, 255);
    #[allow(dead_code)]
    $v const BLUE_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(219, 234, 254);
    #[allow(dead_code)]
    $v const BLUE_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(191, 219, 254);
    #[allow(dead_code)]
    $v const BLUE_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(147, 197, 253);
    #[allow(dead_code)]
    $v const BLUE_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(96, 165, 250);
    #[allow(dead_code)]
    $v const BLUE_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(59, 130, 246);
    #[allow(dead_code)]
    $v const BLUE_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(37, 99, 235);
    #[allow(dead_code)]
    $v const BLUE_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(29, 78, 216);
    #[allow(dead_code)]
    $v const BLUE_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(30, 64, 175);
    #[allow(dead_code)]
    $v const BLUE_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(30, 58, 138);
  };
  ($v:vis INDIGO) => {
    #[allow(dead_code)]
    $v const INDIGO_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(238, 242, 255);
    #[allow(dead_code)]
    $v const INDIGO_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(224, 231, 255);
    #[allow(dead_code)]
    $v const INDIGO_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(199, 210, 254);
    #[allow(dead_code)]
    $v const INDIGO_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(165, 180, 252);
    #[allow(dead_code)]
    $v const INDIGO_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(129, 140, 248);
    #[allow(dead_code)]
    $v const INDIGO_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(99, 102, 241);
    #[allow(dead_code)]
    $v const INDIGO_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(79, 70, 229);
    #[allow(dead_code)]
    $v const INDIGO_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(67, 56, 202);
    #[allow(dead_code)]
    $v const INDIGO_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(55, 48, 163);
    #[allow(dead_code)]
    $v const INDIGO_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(49, 46, 129);
  };
  ($v:vis VIOLET) => {
    #[allow(dead_code)]
    $v const VIOLET_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(245, 243, 255);
    #[allow(dead_code)]
    $v const VIOLET_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(237, 233, 254);
    #[allow(dead_code)]
    $v const VIOLET_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(221, 214, 254);
    #[allow(dead_code)]
    $v const VIOLET_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(196, 181, 253);
    #[allow(dead_code)]
    $v const VIOLET_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(167, 139, 250);
    #[allow(dead_code)]
    $v const VIOLET_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(139, 92, 246);
    #[allow(dead_code)]
    $v const VIOLET_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(124, 58, 237);
    #[allow(dead_code)]
    $v const VIOLET_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(109, 40, 217);
    #[allow(dead_code)]
    $v const VIOLET_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(91, 33, 182);
    #[allow(dead_code)]
    $v const VIOLET_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(76, 29, 149);
  };
  ($v:vis PURPLE) => {
    #[allow(dead_code)]
    $v const PURPLE_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(250, 245, 255);
    #[allow(dead_code)]
    $v const PURPLE_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(243, 232, 255);
    #[allow(dead_code)]
    $v const PURPLE_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(233, 213, 255);
    #[allow(dead_code)]
    $v const PURPLE_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(216, 180, 254);
    #[allow(dead_code)]
    $v const PURPLE_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(192, 132, 252);
    #[allow(dead_code)]
    $v const PURPLE_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(168, 85, 247);
    #[allow(dead_code)]
    $v const PURPLE_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(147, 51, 234);
    #[allow(dead_code)]
    $v const PURPLE_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(126, 34, 206);
    #[allow(dead_code)]
    $v const PURPLE_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(107, 33, 168);
    #[allow(dead_code)]
    $v const PURPLE_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(88, 28, 135);
  };
  ($v:vis FUCHSIA) => {
    #[allow(dead_code)]
    $v const FUCHSIA_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(253, 244, 255);
    #[allow(dead_code)]
    $v const FUCHSIA_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(250, 232, 255);
    #[allow(dead_code)]
    $v const FUCHSIA_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(245, 208, 254);
    #[allow(dead_code)]
    $v const FUCHSIA_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(240, 171, 252);
    #[allow(dead_code)]
    $v const FUCHSIA_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(232, 121, 249);
    #[allow(dead_code)]
    $v const FUCHSIA_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(217, 70, 239);
    #[allow(dead_code)]
    $v const FUCHSIA_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(192, 38, 211);
    #[allow(dead_code)]
    $v const FUCHSIA_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(162, 28, 175);
    #[allow(dead_code)]
    $v const FUCHSIA_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(134, 25, 143);
    #[allow(dead_code)]
    $v const FUCHSIA_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(112, 26, 117);
  };
  ($v:vis PINK) => {
    #[allow(dead_code)]
    $v const PINK_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(253, 242, 248);
    #[allow(dead_code)]
    $v const PINK_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(252, 231, 243);
    #[allow(dead_code)]
    $v const PINK_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(251, 207, 232);
    #[allow(dead_code)]
    $v const PINK_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(249, 168, 212);
    #[allow(dead_code)]
    $v const PINK_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(244, 114, 182);
    #[allow(dead_code)]
    $v const PINK_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(236, 72, 153);
    #[allow(dead_code)]
    $v const PINK_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(219, 39, 119);
    #[allow(dead_code)]
    $v const PINK_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(190, 24, 93);
    #[allow(dead_code)]
    $v const PINK_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(157, 23, 77);
    #[allow(dead_code)]
    $v const PINK_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(131, 24, 67);
  };
  ($v:vis ROSE) => {
    #[allow(dead_code)]
    $v const ROSE_50: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(255, 241, 242);
    #[allow(dead_code)]
    $v const ROSE_100: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(255, 228, 230);
    #[allow(dead_code)]
    $v const ROSE_200: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(254, 205, 211);
    #[allow(dead_code)]
    $v const ROSE_300: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(253, 164, 175);
    #[allow(dead_code)]
    $v const ROSE_400: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(251, 113, 133);
    #[allow(dead_code)]
    $v const ROSE_500: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(244, 63, 94);
    #[allow(dead_code)]
    $v const ROSE_600: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(225, 29, 72);
    #[allow(dead_code)]
    $v const ROSE_700: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(190, 18, 60);
    #[allow(dead_code)]
    $v const ROSE_800: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(159, 18, 57);
    #[allow(dead_code)]
    $v const ROSE_900: ratatui::prelude::Color = ratatui::prelude::Color::Rgb(136, 19, 55);
  };
}

#[cfg(test)]
mod tests {

    use ratatui::prelude::Color;
    palette!(pub SLATE);

    #[test]
    fn color_test() {
        assert_eq!(SLATE_900, Color::Rgb(15, 23, 42));

        palette!(RED);

        assert_eq!(RED_900, Color::Rgb(127, 29, 29));
    }
}
