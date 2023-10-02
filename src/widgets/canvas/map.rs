use strum::{Display, EnumString};

use crate::{
    style::Color,
    widgets::canvas::{
        world::{WORLD_HIGH_RESOLUTION, WORLD_LOW_RESOLUTION},
        Painter, Shape,
    },
};

#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
pub enum MapResolution {
    #[default]
    Low,
    High,
}

impl MapResolution {
    fn data(self) -> &'static [(f64, f64)] {
        match self {
            MapResolution::Low => &WORLD_LOW_RESOLUTION,
            MapResolution::High => &WORLD_HIGH_RESOLUTION,
        }
    }
}

/// Shape to draw a world map with the given resolution and color
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Map {
    pub resolution: MapResolution,
    pub color: Color,
}

impl Shape for Map {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.resolution.data() {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::ParseError;

    use super::*;
    use crate::{
        assert_buffer_eq,
        prelude::*,
        widgets::{canvas::Canvas, Widget},
    };

    #[test]
    fn map_resolution_to_string() {
        assert_eq!(MapResolution::Low.to_string(), "Low");
        assert_eq!(MapResolution::High.to_string(), "High");
    }

    #[test]
    fn map_resolution_from_str() {
        assert_eq!("Low".parse(), Ok(MapResolution::Low));
        assert_eq!("High".parse(), Ok(MapResolution::High));
        assert_eq!(
            "".parse::<MapResolution>(),
            Err(ParseError::VariantNotFound)
        );
    }

    #[test]
    fn default() {
        let map = Map::default();
        assert_eq!(map.resolution, MapResolution::Low);
        assert_eq!(map.color, Color::Reset);
    }

    #[test]
    fn draw_low() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 40));
        let canvas = Canvas::default()
            .marker(Marker::Dot)
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .paint(|context| {
                context.draw(&Map::default());
            });
        canvas.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "                                                                                ",
            "                   ••••••• •• •• •• •                                           ",
            "            ••••••••••••••       •••      ••••  •••  ••    ••••                 ",
            "            ••••••••••••••••     ••                ••• ••••••• •• •• •••        ",
            "• • •• •••••• •••••••••••• ••   •••  •    •••••  •••••••••          ••  • • • • ",
            "•••••       ••••  •••••••• •• ••  •••    •••• ••••    •• •                    • ",
            "   ••••••••  ••••••• •••••  •••       ••••••••                        • •••••   ",
            "  •• ••   ••    •••••••  ••          ••• ••••                        ••    •    ",
            "•••       •••    •••••• ••••         ••••                             •• •   •• ",
            "            •      •••••••••          ••  •   ••• • •• ••            ••         ",
            "            • •     ••••             •• ••••••••• •••   •         • • ••        ",
            "            •         •               ••••• ••••  ••             ••••••         ",
            "             •      ••               •   • •• •                  •••••          ",
            "              ••  •• •              •         ••  ••              •             ",
            "    ••        •••   •••            •           •  •••••    •   •••              ",
            "     •           •••• •••                       •   •  •    •  • ••             ",
            "                  •••• •           •            •• •     •  ••   ••             ",
            "                     ••• ••         •           • •     ••   ••• •••            ",
            "                      •    •        • •• •              •   •   •  •            ",
            "                   •  •     •            •    • •            ••• •  •           ",
            "                     •        •           •   •              •• •   • •         ",
            "                               •                •              ••   ••• •       ",
            " •                    •       •           •     • •                • •          ",
            "                        •                 •    • ••               •  • •   •  • ",
            "                              •                •                •       •       ",
            "                       •    •                 •  •              •        •      ",
            "                       •   ••              • •                  • • ••       •  ",
            "                       •  •                •                         ••••    •• ",
            "                       • •                                             ••   ••• ",
            "                       ••                                                   •   ",
            "                       •• •                                                     ",
            "                       ••                                                       ",
            "                                                                                ",
            "                        •••                        •      •••• • • •• •         ",
            "                       ••••           •••••• •••••• ••••••             • •••    ",
            "         •• •••••• ••••• ••      • ••• •                                   ••   ",
            "•  •••••             ••  •• ••••••                                         • •• ",
            "•    •                 •   •  •                                             • • ",
            "       •                                                                        ",
            "                                                                                ",
        ]);
        assert_buffer_eq!(buffer, expected);
    }

    #[test]
    fn draw_high() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 80, 40));
        let canvas = Canvas::default()
            .marker(Marker::Braille)
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .paint(|context| {
                context.draw(&Map {
                    resolution: MapResolution::High,
                    ..Default::default()
                });
            });
        canvas.render(buffer.area, &mut buffer);
        let expected = Buffer::with_lines(vec![
            "                                                                                ",
            "                  ⢀⣠⠤⠤⠤⠔⢤⣤⡄⠤⡠⣄⠢⠂⢢⠰⣠⡄⣀⡀                      ⣀                   ",
            "            ⢀⣀⡤⣦⠲⢶⣿⣮⣿⡉⣰⢶⢏⡂        ⢀⣟⠁     ⢺⣻⢿⠏   ⠈⠉⠁ ⢀⣀    ⠈⠓⢳⣢⣂⡀               ",
            "            ⡞⣳⣿⣻⡧⣷⣿⣿⢿⢿⣧⡀⠉⠉⠙⢆      ⣰⠇               ⣠⠞⠃⢉⣄⣀⣠⠴⠊⠉⠁ ⠐⠾⠤⢤⠤⡄⠐⣻⠜⢓⠂      ",
            "⢍ ⢀⡴⠊⠙⠓⠒⠒⠤⠖⠺⠿⠽⣷⣬⢬⣾⣷⢻⣷⢲⢲⣍⠱⡀ ⠹⡗   ⢀⢐⠟        ⡔⠒⠉⠲⠤⢀⢄⡀⢩⣣⠦⢷⢼⡏⠈          ⠉⠉⠉ ⠈⠈⠉⠖⠤⠆⠒⠭",
            "⠶⢽⡲⣽⡆             ⠈⣠⣽⣯⡼⢯⣘⡯⠃⠘⡆ ⢰⠒⠁ ⢾⣚⠟    ⢀⠆ ⣔⠆ ⢷⠾⠋⠁    ⠙⠁                     ⠠⡤",
            "  ⠠⢧⣄⣀⡶⠦⠤⡀        ⢰⡁ ⠉⡻⠙⣎⡥  ⠘⠲⠇       ⢀⡀⠨⣁⡄⣸⢫⡤⠄                        ⣀⢠⣤⠊⣼⠅⠖⠋⠁",
            "   ⣠⠾⠛⠁  ⠈⣱        ⠋⠦⢤⡼ ⠈⠈⠦⡀         ⢀⣿⣇ ⢹⣷⣂⡞⠃                       ⢀⣂⡀  ⠏⣜    ",
            "          ⠙⣷⡄        ⠘⠆ ⢀⣀⡠⣗         ⠘⣻⣽⡟⠉⠈                           ⢹⡇  ⠟⠁    ",
            "           ⠈⡟           ⢎⣻⡿⠾⠇         ⠘⠇  ⣀⡀  ⣤⣤⡆ ⡠⡦                 ⢀⠎⡏        ",
            "            ⡇          ⣀⠏⠋           ⢸⠒⢃⡖⢻⢟⣷⣄⣰⣡⠥⣱ ⢏⣧              ⣀ ⡴⠚⢰⠟        ",
            "            ⢳         ⢸⠃             ⠸⣄⣼⣠⢼⡴⡟⢿⢿⣀⣄  ⠸⡹             ⠘⡯⢿⡇⡠⢼⠁        ",
            "             ⢳⣀      ⢀⠞⠁             ⢠⠋⠁ ⠐⠧⡄⣬⣉⣈⡽                  ⢧⠘⢽⠟⠉         ",
            "              ⣿⣄  ⡴⠚⠛⣿⣀             ⢠⠖     ⠈⠁ ⠹⣧  ⢾⣄⡀             ⡼ ⠈           ",
            "    ⣀         ⠘⣿⡄ ⡇  ⣘⣻             ⡏          ⢻⡄ ⠘⠿⢿⠒⠲⡀   ⢀⡀   ⢀⡰⣗             ",
            "    ⠉⠷          ⢫⡀⢧⡼⡟⠉⣛⣳⣦⡀         ⠈⡇          ⠸⣱  ⢀⡼  ⢺  ⡸⠉⢇  ⣾⡏ ⣁             ",
            "                 ⠉⠒⢆⡓⡆             ⠠⡃           ⢳⣇⡠⠏   ⠐⡄⡞  ⠘⣇⡀⢱  ⣾⡀            ",
            "                    ⢹⣇⣀⣾⡷⠤⡆         ⢣            ⠯⢺⠇    ⢣⣅   ⣽⢱⡔ ⢠⢿⣗            ",
            "                     ⠙⢱   ⠘⠦⡄       ⠈⢦⡠⣠⢶⣀        ⡜     ⠈⠿  ⢠⣽⢆ ⢀⣼⡜⠿            ",
            "                     ⢀⡞     ⢱⡀           ⢸       ⡔⠁          ⢻⢿⢰⠏⢸⣤⣴⣆           ",
            "                     ⢘⠆      ⠙⠢⢄         ⠸⡀     ⡸⠁           ⠈⣞⡎⠥⡟⣿⠠⠿⣷⠒⢤⢀⣆      ",
            "                     ⠘⠆        ⢈⠂         ⢳     ⡇             ⠈⠳⠶⣤⣭⣠ ⠋⢧⡬⣟⠉⠷⡄    ",
            "                      ⢨        ⡜          ⢸     ⠸ ⣠               ⠁⢁⣰⢶ ⡇⠉⠁ ⠛    ",
            "⠆                     ⠈⢱⡀      ⡆          ⡇    ⢀⡜⡴⢹               ⢰⠏⠁⠘⢶⠹⡀   ⠸ ⢠⡶",
            "                        ⠅     ⣸           ⢸    ⢫ ⡞⡊             ⢠⠔⠋     ⢳⡀ ⠐⣦   ",
            "                        ⡅    ⡏            ⠈⡆  ⢠⠎ ⠳⠃             ⢸        ⢳      ",
            "                       ⠨    ⡸⠁             ⢱  ⡸                 ⠈⡇ ⢀⣀⡀   ⢸      ",
            "                       ⠸  ⠐⡶⠁              ⠘⠖⠚                   ⠣⠒⠋ ⠱⣇ ⢀⠇   ⠰⡄ ",
            "                       ⠽ ⣰⡖⠁                                          ⠘⢚⡊    ⢀⣿⠇",
            "                       ⡯⢀⡟                                             ⠘⠏   ⢠⢾⠃ ",
            "                       ⠇⢨⠆                            ⢠⡄                    ⠈⠁  ",
            "                       ⢧⣷⡀⠚                                                     ",
            "                        ⠉⠁                                                      ",
            "                          ⢀⡀                                                    ",
            "                        ⢠⡾⠋                      ⣀⡠⠖⢦⣀⣀  ⣀⠤⠦⢤⠤⠶⠤⠖⠦⠤⠤⠤⠴⠤⢤⣄       ",
            "                ⢀⣤⣀ ⡀  ⣼⣻⠙⡆         ⢀⡤⠤⠤⠴⠒⠖⠒⠒⠒⠚⠉⠋⠁    ⢰⡳⠊⠁              ⠈⠉⠉⠒⠤⣤  ",
            "    ⢀⣀⣀⡴⠖⠒⠒⠚⠛⠛⠛⠒⠚⠳⠉⠉⠉⠉⢉⣉⡥⠔⠃     ⢀⣠⠤⠴⠃                                      ⢠⠞⠁  ",
            "   ⠘⠛⣓⣒⠆              ⠸⠥⣀⣤⡦⠠⣞⣭⣇⣘⠿⠆                                         ⣖⠛   ",
            "⠶⠔⠲⠤⠠⠜⢗⠤⠄                 ⠘⠉  ⠁                                            ⠈⠉⠒⠔⠤",
            "                                                                                ",
        ]);
        assert_buffer_eq!(buffer, expected);
    }
}
