use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

use crate::{layout, RgbSwatch, THEME};

const RATATUI_LOGO: [&str; 32] = [
    "               ███              ",
    "             ██████             ",
    "            ███████             ",
    "           ████████             ",
    "          █████████             ",
    "         ██████████             ",
    "        ████████████            ",
    "        █████████████           ",
    "        █████████████     ██████",
    "         ███████████    ████████",
    "              █████ ███████████ ",
    "               ███ ██ee████████ ",
    "                █ █████████████ ",
    "            ████ █████████████  ",
    "           █████████████████    ",
    "           ████████████████     ",
    "           ████████████████     ",
    "            ███ ██████████      ",
    "          ██    █████████       ",
    "         █xx█   █████████       ",
    "        █xxxx█ ██████████       ",
    "       █xx█xxx█ █████████       ",
    "      █xx██xxxx█ ████████       ",
    "     █xxxxxxxxxx█ ██████████    ",
    "    █xxxxxxxxxxxx█ ██████████   ",
    "   █xxxxxxx██xxxxx█ █████████   ",
    "  █xxxxxxxxx██xxxxx█ ████  ███  ",
    " █xxxxxxxxxxxxxxxxxx█ ██   ███  ",
    "█xxxxxxxxxxxxxxxxxxxx█ █   ███  ",
    "█xxxxxxxxxxxxxxxxxxxxx█   ███   ",
    " █xxxxxxxxxxxxxxxxxxxxx█ ███    ",
    "  █xxxxxxxxxxxxxxxxxxxxx█ █     ",
];

pub struct AboutTab {
    selected_row: usize,
}

impl AboutTab {
    pub fn new(selected_row: usize) -> Self {
        Self { selected_row }
    }
}

impl Widget for AboutTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        RgbSwatch.render(area, buf);
        let area = layout(area, Direction::Horizontal, vec![34, 0]);
        render_crate_description(area[1], buf);
        render_logo(self.selected_row, area[0], buf);
    }
}

fn render_crate_description(area: Rect, buf: &mut Buffer) {
    let area = area.inner(
        &(Margin {
            vertical: 4,
            horizontal: 2,
        }),
    );
    Clear.render(area, buf); // clear out the color swatches
    Block::new().style(THEME.content).render(area, buf);
    let area = area.inner(
        &(Margin {
            vertical: 1,
            horizontal: 2,
        }),
    );
    let text = "- cooking up terminal user interfaces -

    Ratatui is a Rust crate that provides widgets (e.g. Paragraph, Table) and draws them to the \
    screen efficiently every frame.";
    Paragraph::new(text)
        .style(THEME.description)
        .block(
            Block::new()
                .title(" Ratatui ")
                .title_alignment(Alignment::Center)
                .borders(Borders::TOP)
                .border_style(THEME.description_title)
                .padding(Padding::new(0, 0, 0, 0)),
        )
        .wrap(Wrap { trim: true })
        .scroll((0, 0))
        .render(area, buf);
}

/// Use half block characters to render a logo based on the RATATUI_LOGO const.
///
/// The logo is rendered in three colors, one for the rat, one for the terminal, and one for the
/// rat's eye. The eye color alternates between two colors based on the selected row.
pub fn render_logo(selected_row: usize, area: Rect, buf: &mut Buffer) {
    let eye_color = if selected_row % 2 == 0 {
        THEME.logo.rat_eye
    } else {
        THEME.logo.rat_eye_alt
    };
    let area = area.inner(&Margin {
        vertical: 0,
        horizontal: 2,
    });
    for (y, (line1, line2)) in RATATUI_LOGO.iter().tuples().enumerate() {
        for (x, (ch1, ch2)) in line1.chars().zip(line2.chars()).enumerate() {
            let x = area.left() + x as u16;
            let y = area.top() + y as u16;
            let cell = buf.get_mut(x, y);
            let rat_color = THEME.logo.rat;
            let term_color = THEME.logo.term;
            match (ch1, ch2) {
                ('█', '█') => {
                    cell.set_char('█');
                    cell.fg = rat_color;
                }
                ('█', ' ') => {
                    cell.set_char('▀');
                    cell.fg = rat_color;
                }
                (' ', '█') => {
                    cell.set_char('▄');
                    cell.fg = rat_color;
                }
                ('█', 'x') => {
                    cell.set_char('▀');
                    cell.fg = rat_color;
                    cell.bg = term_color;
                }
                ('x', '█') => {
                    cell.set_char('▄');
                    cell.fg = rat_color;
                    cell.bg = term_color;
                }
                ('x', 'x') => {
                    cell.set_char(' ');
                    cell.fg = term_color;
                    cell.bg = term_color;
                }
                ('█', 'e') => {
                    cell.set_char('▀');
                    cell.fg = rat_color;
                    cell.bg = eye_color;
                }
                ('e', '█') => {
                    cell.set_char('▄');
                    cell.fg = rat_color;
                    cell.bg = eye_color;
                }
                (_, _) => {}
            };
        }
    }
}
