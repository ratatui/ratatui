//! This module provides the `BevyBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use std::io;

use bevy::{
    ecs::system::RunSystemOnce,
    prelude::{Color as BevyColor, *},
    window::{PrimaryWindow,WindowResolution,WindowResized},
    utils::HashMap,
};

use crate::{
    backend::{Backend, ClearType, WindowSize},
    buffer::{Buffer, Cell},
    layout::{Rect, Size},
    terminal::Terminal,
    style::{Color as RatColor},
};


trait FromAnsi<u8>  {
    fn from_ansi(beep: u8) -> BevyColor;
}


impl FromAnsi<u8> for BevyColor{

    fn from_ansi(beep: u8) -> BevyColor {

        BevyColor::rgb_u8(beep,beep,beep)







    }
}

impl From<RatColor> for BevyColor {
    fn from(color:RatColor) -> Self {
        match color {
           RatColor::Reset => BevyColor::TOMATO,
           RatColor::Black => BevyColor::BLACK,
           RatColor::Red => BevyColor::MAROON,
           RatColor::Green => BevyColor::DARK_GREEN,
           RatColor::Yellow => BevyColor::GOLD,
           RatColor::Blue => BevyColor::MIDNIGHT_BLUE,
           RatColor::Magenta => BevyColor::FUCHSIA,
           RatColor::Cyan => BevyColor::CYAN,
           RatColor::Gray => BevyColor::GRAY,
           RatColor::DarkGray => BevyColor::DARK_GRAY,
           RatColor::LightRed => BevyColor::RED,
           RatColor::LightGreen => BevyColor::GREEN,
           RatColor::LightBlue => BevyColor::BLUE,
           RatColor::LightYellow => BevyColor::BISQUE,
           RatColor::LightMagenta => BevyColor::PINK,
           RatColor::LightCyan => BevyColor::AQUAMARINE,
           RatColor::White => BevyColor::WHITE,
           RatColor::Indexed(i) => BevyColor::from_ansi(i),
           RatColor::Rgb(r, g, b) => BevyColor::rgb_u8( r, g, b ),
        }
    }
}

impl From<BevyColor> for RatColor {
    fn from(value: BevyColor) -> Self {
        match value {
            BevyColor::TOMATO => Self::Reset,
            BevyColor::BLACK => Self::Black,
            BevyColor::MAROON => Self::Red,
            BevyColor::DARK_GRAY => Self::Green,
            BevyColor::GOLD => Self::Yellow,
            BevyColor::MIDNIGHT_BLUE => Self::Blue,
            BevyColor::FUCHSIA => Self::Magenta,
            BevyColor::CYAN => Self::Cyan,
            BevyColor::GRAY => Self::Gray,
            BevyColor::DARK_GRAY => Self::DarkGray,
            BevyColor::RED => Self::LightRed,
            BevyColor::GREEN => Self::LightGreen,
            BevyColor::BLUE => Self::LightBlue,
            BevyColor::BISQUE => Self::LightYellow,
            BevyColor::PINK => Self::LightMagenta,
            BevyColor::AQUAMARINE => Self::LightCyan,
            BevyColor::WHITE => Self::White,
            BevyColor::Rgba{ red,green,blue,alpha} => Self::Rgb(red as u8,green as u8,blue as u8),
            BevyColor::Rgba{ red,green,blue,alpha} => Self::Indexed(red as u8),
            _ =>Self::Reset,
        }
    }
}

pub struct RatatuiPlugin;

impl Plugin for RatatuiPlugin {
    fn build(&self, app: &mut App) {
        let world = &mut app.world;
        world.init_resource::<FontHandlers>();
        app.init_state::<AppState>();
        app.add_systems(PreStartup, (font_setup));
        app.add_systems(
            First,
            (init_bevy_terminals.run_if(in_state(AppState::TermNeedsIniting))),
        );
        app.add_systems(First, (query_term_for_init,handle_primary_window_resize));
        app.add_systems(First, (handle_primary_window_resize));

        app.add_systems(PreUpdate, (update_ents_from_buffer, update_ents_from_comp).run_if(in_state(AppState::AllTermsInited)));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum AppState {
    #[default]
    NoTermsInited,
    AllTermsInited,
    TermNeedsIniting,
}

fn init_virtual_cells(
    mut commands: Commands,
    mut terminal_query: Query<(&mut Terminal<BevyBackend>)>,
) {
    let mut termy = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let termy_backend = termy.backend_mut();
    let rows = termy_backend.height;
    let columns = termy_backend.width;

    for y in 0..rows {
        for x in 0..columns {
            let cell = commands.spawn((VirtualCell::new(x, y))).id();
            termy_backend.entity_map.insert((x, y), cell);
        }
    }
}

fn query_term_for_init(
    mut terminal_query: Query<(&mut Terminal<BevyBackend>)>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let mut termy = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let termy_backend = termy.backend_mut();

    if (termy_backend.bevy_initialized == false) {
        app_state.set(AppState::TermNeedsIniting);
        termy_backend.bevy_initialized = true;
    }
}

fn set_terms_inited(mut next_game_state: ResMut<NextState<AppState>>) {
    next_game_state.set(AppState::AllTermsInited);
}

fn init_bevy_terminals(world: &mut World) {
    world.run_system_once(set_terms_inited);

    world.run_system_once(init_virtual_cells);
    world.run_system_once(add_render_to_cells);
}

fn update_ents_from_buffer(
    mut commands: Commands,
    mut terminal_query: Query<(&mut Terminal<BevyBackend>)>,
) {
    let mut termy = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let termy_backend = termy.backend_mut();
    let boop = termy_backend.entity_map.clone();

    while let Some((x, y, vc)) = termy_backend.vcupdate.pop() {
        let xy = (x, y);
        let eid = boop
            .get(&xy)
            .expect("ENTITY MAP IS MISSING THIS ENTITY at {x}{y}");

        //commands.entity(eid.clone()).remove::<TextBundle>();

        commands.entity(eid.clone()).insert(vc);
    }
}


fn handle_primary_window_resize(mut windows: Query<&mut Window, With<PrimaryWindow>>,terminal_query: Query<(&Terminal<BevyBackend>)>,mut resize_event: EventReader<WindowResized>,) {

for _ in resize_event.read(){

    let termy = terminal_query
    .get_single()
    .expect("More than one terminal with a bevybackend");
let termy_backend = termy.backend();
    let terminal_width = termy_backend.width;
    let terminal_height = termy_backend.height;
    let terminal_font_size = termy_backend.term_font_size;
    // Query returns one window typically.
    for mut window in windows.iter_mut() {
        let w_wid = (terminal_width*terminal_font_size) as f32 * 0.5; 
        let w_hei = (terminal_height*terminal_font_size) as f32 ; 
        window.resolution = WindowResolution::new(w_wid,w_hei);
    }
}


}

 




fn update_ents_from_comp(
    query_cells: Query<(Entity, &VirtualCell), (Changed<VirtualCell>)>,
    mut commands: Commands,
    font_handlers: Res<FontHandlers>,
    terminal_query: Query<(&Terminal<BevyBackend>)>,
) {
    let termy = terminal_query
        .get_single()
        .expect("More than one terminal with a bevybackend");
    let termy_backend = termy.backend();
    let fontsize = termy_backend.term_font_size as f32;

    let pixel_shift = fontsize / 2.0;

    for (entity_id, cellii) in query_cells.iter() {
        commands.entity(entity_id).insert(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                &cellii.symbol,
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: font_handlers.normal.clone(),
                    font_size: fontsize,
                    color:cellii.fg,
                    ..default()
                },
            ) // Set the justification of the Text
            .with_background_color(cellii.bg)
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(cellii.row as f32 * fontsize),
                left: Val::Px(cellii.column as f32 * pixel_shift),
                ..default()
            }),
        );
    }
}

fn add_render_to_cells(
    query_cells: Query<(Entity, &VirtualCell)>,
    mut terminal_query: Query<(&mut Terminal<BevyBackend>)>,
    mut commands: Commands,
    font_handlers: Res<FontHandlers>,
) {
    let mut termy = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let termy_backend = termy.backend();
    let fontsize = termy_backend.term_font_size as f32;

    let pixel_shift = fontsize / 2.0;

    for (entity_id, cellii) in query_cells.iter() {
        commands.entity(entity_id).insert(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                &cellii.symbol,
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: font_handlers.normal.clone(),
                    font_size: fontsize,
                    ..default()
                },
            ) // Set the justification of the Text
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(cellii.row as f32 * fontsize),
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
#[derive(Component, Debug, Clone, PartialEq)]
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
        self.fg = BevyColor::from( given_cell.fg);
        self.bg = BevyColor::from( given_cell.bg);
        #[cfg(not(feature="underline-color"))]    
         let beep = given_cell.fg;
        #[cfg(feature="underline-color")]    
         let beep = given_cell.underline_color;
        self.underline_color = Some(BevyColor::from( beep));
        self.skip = given_cell.skip;
    
        
    }
}

///
///
///
/// RATATUI SPECIFIC STUFF STARTS HERE

#[derive(Component, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BevyBackend {
    height: u16,
    width: u16,
    term_font_size: u16,
    entity_map: HashMap<(u16, u16), Entity>,
    buffer: Buffer,
    vcupdate: Vec<(u16, u16, VirtualCell)>,

    cursor: bool,
    cursor_pos: (u16, u16),
    bevy_initialized: bool,
}

impl Default for BevyBackend {
    fn default() -> Self {
        BevyBackend {
            height: 30,
            width: 10,
            term_font_size: 40,
            entity_map: HashMap::new(),
            buffer: Buffer::empty(Rect::new(0, 0, 3, 17)),
            vcupdate: Vec::default(),
            cursor: false,
            cursor_pos: (0, 0),
            bevy_initialized: false,
        }
    }
}

impl BevyBackend {
    /// Creates a new BevyBackend with the specified width and height.
    pub fn new(width: u16, height: u16, font_size: u16) -> BevyBackend {
        BevyBackend {
            height: height,
            width: width,
            term_font_size: font_size,
            entity_map: HashMap::new(),
            buffer: Buffer::empty(Rect::new(0, 0, width, height)),
            vcupdate: Vec::default(),
            cursor: false,
            cursor_pos: (0, 0),
            bevy_initialized: false,
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
            let mut vc = VirtualCell::new(x, y);
            vc.to_virtual(c);
            self.vcupdate.push((x, y, vc));

            let cell = self.buffer.get_mut(x, y);
            *cell = c.clone();

            println!("{} {}", x, y);
            println!("{:?}", c);
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
