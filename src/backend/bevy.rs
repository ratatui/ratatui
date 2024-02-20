//! This module provides the `BevyBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use std::io;
use std::collections::HashMap;

use bevy::{
    prelude::{Color as BevyColor, *},
   // utils::HashMap,
};

use crate::{
    backend::{Backend, ClearType, WindowSize},
    buffer::{Buffer, Cell},
    layout::{Rect, Size},
};

pub struct RatatuiPlugin;

impl Plugin for RatatuiPlugin {
    fn build(&self, app: &mut App) {
        let world = &mut app.world;
        world.init_resource::<BevyBackend>();
        world.init_resource::<FontHandlers>();
        app.add_systems(PreStartup, (font_setup));
        app.add_systems(Startup, init_virtual_cells);
        app.add_systems(PostStartup, add_render_to_cells);
    }
}

fn init_virtual_cells(mut commands: Commands, mut terminal_res: ResMut<BevyBackend>) {
    let rows = terminal_res.height;
    let columns = terminal_res.width;

    for y in 0..rows {
        for x in 0..columns {
            let cell = commands.spawn((VirtualCell::new(x, y))).id();
            terminal_res.entity_map.insert((x, y), cell);
        }
    }
}

fn add_render_to_cells(
    query_cells: Query<(Entity, &VirtualCell)>,
    terminal_res: Res<BevyBackend>,
    mut commands: Commands,
    font_handlers: Res<FontHandlers>,
) {
    let mut fontsize = terminal_res.term_font_size as f32;

    let pixel_shift = fontsize  / 2.0;

    for (entity_id, cellii) in query_cells.iter() {
        commands.entity(entity_id).insert(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                &cellii.symbol,
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: font_handlers.normal.clone(),
                    font_size: fontsize ,
                    ..default()
                },
            ) // Set the justification of the Text
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(cellii.row as f32 * fontsize ),
                left: Val::Px(cellii.column as f32 * pixel_shift),
                ..default()
            }),
        );
    }
}

fn font_setup(asset_server: Res<AssetServer>, mut font_handlers: ResMut<FontHandlers>) {
    let big_handle: Handle<Font> = asset_server.load("fonts/DejaVuSansMono.ttf");
    font_handlers.normal = big_handle;
}

#[derive(Resource)]
struct FontHandlers {
    normal: Handle<Font>,
}

impl Default for FontHandlers {
    fn default() -> Self {
        FontHandlers {
            normal: Handle::weak_from_u128(101),
        }
    }
}

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct VirtualCell {
    symbol: String,
    fg: BevyColor,
    bg: BevyColor,
    underline_color: Option<BevyColor>,
    skip: bool,
    bold: bool,
    dim: bool,
    italic: bool,
    underlined: bool,
    slow_blink: bool,
    rapid_blink: bool,
    reversed: bool,
    hidden: bool,
    crossed_out: bool,
    row: u16,
    column: u16,
}

impl VirtualCell {
    fn new(x: u16, y: u16) -> Self {
        VirtualCell {
            symbol: "╬".to_string(),
            fg: bevy::prelude::Color::TOMATO,
            bg: bevy::prelude::Color::ORANGE,
            underline_color: None,
            skip: false,
            bold: false,
            dim: false,
            italic: false,
            underlined: false,
            slow_blink: false,
            rapid_blink: false,
            reversed: false,
            hidden: false,
            crossed_out: false,
            row: y,
            column: x,
        }
    }
}


trait FromRatCell {
    fn to_virtual(&mut self, given_cell: &Cell);
}

impl FromRatCell for VirtualCell {
    fn to_virtual(&mut self, given_cell: &Cell) {
        self.symbol = given_cell.symbol().into();
    }
}


///
///
///
/// RATATUI SPECIFIC STUFF STARTS HERE

#[derive(Resource, Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BevyBackend {
    height: u16,
    width: u16,
    term_font_size: u16,
    entity_map: HashMap<(u16, u16), Entity>,
    buffer: Buffer,
   
    cursor: bool,
    cursor_pos: (u16, u16),
}

impl Default for BevyBackend {
    fn default() -> Self {
        BevyBackend {
            height: 17,
            width: 3,
            term_font_size: 40,
            entity_map: HashMap::new(),
            buffer: Buffer::empty(Rect::new(0, 0, 3, 17)),
            cursor: false,
            cursor_pos: (0, 0),
        }
    }
}

impl BevyBackend {
    /// Creates a new BevyBackend with the specified width and height.
    pub fn new(width: u16, height: u16) -> BevyBackend {
        BevyBackend {
            height: height,
            width: width,
            term_font_size: 40,
            entity_map: HashMap::new(),
            buffer: Buffer::empty(Rect::new(0, 0, width, height)),
            cursor: false,
            cursor_pos: (0, 0),
        }
    }

    /// Resizes the BevyBackend to the specified width and height.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.buffer.resize(Rect::new(0, 0, width, height));
        self.width = width;
        self.height = height;
    }
}

impl Backend for BevyBackend {
    fn draw<'a, I>(&mut self, content: I) -> Result<(), io::Error>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        for (x, y, c) in content {
            let cell = self.buffer.get_mut(x, y);
            *cell = c.clone();
            println!("{} {}", x, y);
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor = false;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), io::Error> {
        self.cursor = true;
        Ok(())
    }

    fn get_cursor(&mut self) -> Result<(u16, u16), io::Error> {
        Ok(self.cursor_pos)
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), io::Error> {
        self.cursor_pos = (x, y);
        Ok(())
    }

    fn clear(&mut self) -> Result<(), io::Error> {
        self.buffer.reset();
        Ok(())
    }

    fn clear_region(&mut self, clear_type: super::ClearType) -> io::Result<()> {
        match clear_type {
            ClearType::All => self.clear()?,
            ClearType::AfterCursor => {
                let index = self.buffer.index_of(self.cursor_pos.0, self.cursor_pos.1) + 1;
                self.buffer.content[index..].fill(Cell::default());
            }
            ClearType::BeforeCursor => {
                let index = self.buffer.index_of(self.cursor_pos.0, self.cursor_pos.1);
                self.buffer.content[..index].fill(Cell::default());
            }
            ClearType::CurrentLine => {
                let line_start_index = self.buffer.index_of(0, self.cursor_pos.1);
                let line_end_index = self.buffer.index_of(self.width - 1, self.cursor_pos.1);
                self.buffer.content[line_start_index..=line_end_index].fill(Cell::default());
            }
            ClearType::UntilNewLine => {
                let index = self.buffer.index_of(self.cursor_pos.0, self.cursor_pos.1);
                let line_end_index = self.buffer.index_of(self.width - 1, self.cursor_pos.1);
                self.buffer.content[index..=line_end_index].fill(Cell::default());
            }
        }
        Ok(())
    }

    /// Inserts n line breaks at the current cursor position.
    ///
    /// After the insertion, the cursor x position will be incremented by 1 (unless it's already
    /// at the end of line). This is a common behaviour of terminals in raw mode.
    ///
    /// If the number of lines to append is fewer than the number of lines in the buffer after the
    /// cursor y position then the cursor is moved down by n rows.
    ///
    /// If the number of lines to append is greater than the number of lines in the buffer after
    /// the cursor y position then that number of empty lines (at most the buffer's height in this
    /// case but this limit is instead replaced with scrolling in most backend implementations) will
    /// be added after the current position and the cursor will be moved to the last row.
    fn append_lines(&mut self, n: u16) -> io::Result<()> {
        let (cur_x, cur_y) = self.get_cursor()?;

        // the next column ensuring that we don't go past the last column
        let new_cursor_x = cur_x.saturating_add(1).min(self.width.saturating_sub(1));

        let max_y = self.height.saturating_sub(1);
        let lines_after_cursor = max_y.saturating_sub(cur_y);
        if n > lines_after_cursor {
            let rotate_by = n.saturating_sub(lines_after_cursor).min(max_y);

            if rotate_by == self.height - 1 {
                self.clear()?;
            }

            self.set_cursor(0, rotate_by)?;
            self.clear_region(ClearType::BeforeCursor)?;
            self.buffer
                .content
                .rotate_left((self.width * rotate_by).into());
        }

        let new_cursor_y = cur_y.saturating_add(n).min(max_y);
        self.set_cursor(new_cursor_x, new_cursor_y)?;

        Ok(())
    }

    fn size(&self) -> Result<Rect, io::Error> {
        Ok(Rect::new(0, 0, self.width, self.height))
    }

    fn window_size(&mut self) -> Result<WindowSize, io::Error> {
        // Some arbitrary window pixel size, probably doesn't need much testing.
        static WINDOW_PIXEL_SIZE: Size = Size {
            width: 640,
            height: 480,
        };
        Ok(WindowSize {
            columns_rows: (self.width, self.height).into(),
            pixels: WINDOW_PIXEL_SIZE,
        })
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
