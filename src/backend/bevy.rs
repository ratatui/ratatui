//! This module provides the `BevyBackend` implementation for the [`Backend`] trait.
//! It is used in the integration tests to verify the correctness of the library.

use std::io;

use bevy::{
    ecs::system::RunSystemOnce,
    prelude::{Color as BevyColor, *},
    utils::HashMap,
    window::{PrimaryWindow, WindowResized, WindowResolution},
};
use bevy::ui::ContentSize;
use crate::{
    backend::{Backend, ClearType, WindowSize},
    buffer::{Buffer, Cell},
    layout::{Rect, Size},
    style::{Color as RatColor,Modifier},
    terminal::Terminal,
};


pub struct RatatuiPlugin;

impl Plugin for RatatuiPlugin {
    fn build(&self, app: &mut App) {
        let world = &mut app.world;
        
        app.init_state::<TermState>();
       

        app.add_systems(
            Last,
            (init_bevy_terminals.run_if(in_state(TermState::TermNeedsIniting))),
        );
        app.add_systems(First, (query_term_for_init));
        app.add_systems(
            First,
            (handle_primary_window_resize).run_if(on_event::<WindowResized>()),
        );
        app.add_systems(
            PostUpdate,
            (update_ents_from_vcupdate).run_if(in_state(TermState::AllTermsInited)),
        );
        app.add_systems(
            PostUpdate,
            (debug_entities).run_if(in_state(TermState::AllTermsInited)),
        );


        app.add_systems(
            PreUpdate,
            (update_ents_from_comp).run_if(in_state(TermState::AllTermsInited)),
        );
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum TermState {
    #[default]
    NoTermsInited,
    TermInited,
    TermHasChangesToVCBuffer,
    TermResized,

    AllTermsInited,
    TermNeedsIniting,
}

fn query_term_for_init(
    mut terminal_query: Query<(&mut Terminal<BevyBackend>)>,
    mut app_state: ResMut<NextState<TermState>>,
    
) {
    println!("entering   query_term_for_init");
    let mut termy = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let termy_backend = termy.backend_mut();

    if (termy_backend.bevy_initialized == false) {
        app_state.set(TermState::TermNeedsIniting);
        termy_backend.bevy_initialized = true;
    }

    println!("RUNNING   query_term_for_init");
    println!("{:?}",termy_backend.bevy_initialized);
}

fn set_terms_inited(mut next_game_state: ResMut<NextState<TermState>>) {
    println!("entering   set_terms_inited");
    next_game_state.set(TermState::AllTermsInited);
    println!("RUNNING   set_terms_inited");
}

fn clear_virtual_cells(
    mut commands: Commands,
    mut terminal_query: Query<(Entity,&mut Terminal<BevyBackend>)>,
) {
    println!("entering   clear_virtual_cells");
    let (e,mut termy) = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let mut termy_backend = termy.backend_mut();

    commands.entity(e).insert(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    });

    for (_, entity) in termy_backend.entity_map.iter_mut() {
        commands.entity(*entity).despawn();
    }

    termy_backend.entity_map = HashMap::new();
    println!("RUNNING   clear_virtual_cells");
}

fn init_bevy_terminals(world: &mut World) {
    println!("entering   init_bevy_terminals");
    apply_deferred(world);
    println!("0");
    world.run_system_once(clear_virtual_cells);
    
    println!("1");
    world.run_system_once(font_setup);
    println!("2");
    world.run_system_once(init_virtual_cells);
    //   world.run_system_once(add_render_to_cells);
    println!("3");
    world.run_system_once(update_ents_from_vcupdate);
    println!("4");
    world.run_system_once(update_ents_from_comp);
    println!("5");
    world.run_system_once(set_terms_inited);
    println!("RUNNING   init_bevy_terminals");
}

fn init_virtual_cells(
    mut commands: Commands,
    mut terminal_query: Query<(Entity, &mut Terminal<BevyBackend>)>,
    
) {
    println!("entering   init_virtual_cells");
    let (e,mut termy) = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let termy_backend = termy.backend_mut();
    let rows = termy_backend.height;
    let columns = termy_backend.width;
    termy_backend.entity_map = HashMap::new();

    

    

   
    

    

   // commands.entity(e).with_children(|parent| {

        // left vertical fill (border)
        for y in 0..rows {
            for x in 0..columns {
                let ratcell = termy_backend.buffer.get(x, y);
                let vcell = commands
                    .spawn((VirtualCell::to_virtual(x, y, ratcell),NodeBundle{
                   
                        ..default()
                    }) )
                    .id();
                println!("Spawning a child");
                termy_backend.entity_map.insert((x, y), vcell);
            }
        }
    
   // });
     //   


    
    println!("RUNNING   init_virtual_cells");
}

fn update_ents_from_vcupdate(
    mut commands: Commands,
    mut terminal_query: Query<(&mut Terminal<BevyBackend>)>,
) {
    println!("entering   update_ents_from_vcupdate");
    let mut termy = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let termy_backend = termy.backend_mut();
    let boop = termy_backend.entity_map.clone();

    while let Some((x, y, vc)) = termy_backend.vcupdate.pop() {
        let xy = (x, y);
        match boop.get(&xy) {
            Some(wow) => {
                commands.entity(wow.clone()).insert(vc);
                ()
            }
            None => (),
        };

        //commands.entity(eid.clone()).remove::<TextBundle>();
    }
    println!("RUNNING   update_ents_from_vcupdate");
}

fn handle_primary_window_resize(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut terminal_query: Query<(&mut Terminal<BevyBackend>)>,
    mut resize_event: EventReader<WindowResized>,
) {
    println!("entering   handle_primary_window_resize");
    for wr in resize_event.read() {
        let mut termy = terminal_query
            .get_single_mut()
            .expect("More than one terminal with a bevybackend");
        let termy_backend = termy.backend_mut();

        let terminal_font_size = termy_backend.term_font_size as f32;
        let terminal_font_aspect_ratio = termy_backend.font_aspect_ratio as f32;

        let w_wid = (terminal_font_size * terminal_font_aspect_ratio);
        let w_hei = terminal_font_size as f32;

        let new_wid = (wr.width / w_wid) as u16;
        let new_hei = (wr.height / w_hei) as u16;

        termy_backend.resize(new_wid as u16, new_hei as u16);
        termy_backend.bevy_initialized = false;

        println!("WINDOW IS RESING");

        for mut window in windows.iter_mut() {
            window.resolution = WindowResolution::new(
                new_wid as f32 * termy_backend.font_aspect_ratio * terminal_font_size as f32,
                new_hei as f32 * terminal_font_size as f32,
            );

            // Query returns one window typically.
        }
    }
    println!("RUNNING   handle_primary_window_resize");
}

fn debug_entities(query_cells: Query<(Entity, &Node)>,){

    for (entity_id, cs) in query_cells.iter() {

        


        println!("the calculated size is {:?}",cs.size());

    }

}

fn update_ents_from_comp(
    //this should run after update from vcbuffer
    query_cells: Query<(Entity,&Node, &VirtualCell), (Changed<VirtualCell>)>,
    mut commands: Commands,
    terminal_query: Query<((&Terminal<BevyBackend>))>,
) {
    println!("entering   update_ents_from_comp");
    let termy = terminal_query
    .get_single()
    .expect("More than one terminal with a bevybackend");
let termy_backend = termy.backend();
let fontsize = termy_backend.term_font_size as f32;
    


    

   


    for (entity_id, nodik, cellii) in query_cells.iter() {

        if !cellii.skip{
            let node_size = nodik.size();

            let mut proper_font = Handle::weak_from_u128(101);

           if (cellii.bold && cellii.italic){proper_font=termy_backend.italicbold_handle.clone();}
           else if (cellii.bold){proper_font=termy_backend.bold_handle.clone();}
           else if (cellii.italic){proper_font=termy_backend.italic_handle.clone();}
           else { proper_font=termy_backend.normal_handle.clone();}




        commands.entity(entity_id).insert(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                &cellii.symbol,
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: proper_font,
                    font_size: fontsize,
                    color: cellii.fg,
                    ..default()
                },
            ) // Set the justification of the Text
            .with_background_color(cellii.bg)
            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                display:Display::Grid,
                position_type: PositionType::Absolute,
                align_items:AlignItems::Stretch,
                margin:UiRect::ZERO,
                padding:UiRect::ZERO,
                border:UiRect::ZERO,
                grid_auto_flow: GridAutoFlow::Column,
                top: Val::Px(cellii.row as f32 * node_size.y),
                left: Val::Px(cellii.column as f32 * node_size.x),
            //  grid_row: GridPlacement::start(cellii.row as i16 +1),
            //  grid_column: GridPlacement::start(cellii.column as i16 +1),
                ..default()
            }),
        );
    }
    }
    println!("RUNNING   update_ents_from_comp");
}

fn font_setup( mut commands: Commands,asset_server: Res<AssetServer>,mut terminal_query: Query<((Entity,&mut Terminal<BevyBackend>))>) {

    println!("entering   font_setup");
        let  (e,mut termy) = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let mut termy_backend = termy.backend_mut();




 

    termy_backend.normal_handle = asset_server.load(&termy_backend.normal_font_path);
    termy_backend.italic_handle = asset_server.load(&termy_backend.italic_font_path);
    termy_backend.bold_handle = asset_server.load(&termy_backend.bold_font_path);
    termy_backend.italicbold_handle = asset_server.load(&termy_backend.italicbold_font_path);
       
        println!("RUNNING   font_setup");

    
    
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
            fg: bevy::prelude::Color::WHITE,
            bg: bevy::prelude::Color::BLACK,
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
    fn to_virtual(x: u16, y: u16, given_cell: &Cell) -> VirtualCell;
}

impl FromRatCell for VirtualCell {
    fn to_virtual(x: u16, y: u16, given_cell: &Cell) -> VirtualCell {

     //   println!("AAAAAAAAAA{}",given_cell.modifier.intersects(Modifier::BOLD));
        VirtualCell {
            symbol: given_cell.symbol().into(),
            fg: BevyColor::from_rat_color(given_cell.fg, true),
            bg: BevyColor::from_rat_color(given_cell.bg, false),
            #[cfg(not(feature = "underline-color"))]
            underline_color: Some(BevyColor::from_rat_color(given_cell.fg, true)),
            #[cfg(feature = "underline-color")]
            underline_color: Some(BevyColor::from_rat_color(given_cell.underline_color, true)),
            bold: given_cell.modifier.intersects(Modifier::BOLD),
            dim: given_cell.modifier.intersects(Modifier::DIM),
            italic: given_cell.modifier.intersects(Modifier::ITALIC),
            underlined: given_cell.modifier.intersects(Modifier::UNDERLINED),
            slow_blink: given_cell.modifier.intersects(Modifier::SLOW_BLINK),
            rapid_blink: given_cell.modifier.intersects(Modifier::RAPID_BLINK),
            reversed: given_cell.modifier.intersects(Modifier::REVERSED),
            hidden: given_cell.modifier.intersects(Modifier::HIDDEN),
            crossed_out: given_cell.modifier.intersects(Modifier::CROSSED_OUT),//FIX THIUS SHOULD NOT BE ALL FALSE

            skip: given_cell.skip,
            row: y,
            column: x,
        }
    }
}

impl FromRatColor<RatColor> for BevyColor {
    fn from_rat_color(color: RatColor, fg: bool) -> Self {
        match color {
            RatColor::Reset => {
                if fg {
                    BevyColor::WHITE
                } else {
                    BevyColor::DARK_GRAY
                }
            }
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
            RatColor::Rgb(r, g, b) => BevyColor::rgb_u8(r, g, b),
        }
    }
}

trait FromAnsi<u8> {
    fn from_ansi(beep: u8) -> BevyColor;
}

impl FromAnsi<u8> for BevyColor {
    fn from_ansi(beep: u8) -> BevyColor {
        BevyColor::rgb_u8(beep, beep, beep)
    }
}

trait FromRatColor<RatColor> {
    fn from_rat_color(color: RatColor, fg: bool) -> BevyColor;
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
    prev_buffer: Buffer,
    vcupdate: Vec<(u16, u16, VirtualCell)>,
    font_aspect_ratio: f32,
    cursor: bool,
    cursor_pos: (u16, u16),
    bevy_initialized: bool,
    normal_font_path: String,
    italic_font_path: String,
    bold_font_path: String,
    italicbold_font_path: String,
    normal_handle:Handle<Font>,
    italic_handle:Handle<Font>,
    bold_handle:Handle<Font>,
    italicbold_handle:Handle<Font>,

}

impl Default for BevyBackend {
    fn default() -> Self {
        BevyBackend {
            height: 30,
            width: 10,
            term_font_size: 40,
            entity_map: HashMap::new(),
            buffer: Buffer::empty(Rect::new(0, 0, 10, 30)),
            prev_buffer: Buffer::empty(Rect::new(0, 0, 10, 30)),
            vcupdate: Vec::default(),
            cursor: false,
            cursor_pos: (0, 0),
            bevy_initialized: false,
            font_aspect_ratio: 0.5,
            normal_font_path: "NO FONT PROVIDED".to_string(),
        italic_font_path: "NO FONT PROVIDED".to_string(),
         bold_font_path: "NO FONT PROVIDED".to_string(),
         italicbold_font_path: "NO FONT PROVIDED".to_string(),
         normal_handle:Handle::weak_from_u128(101),
         italic_handle:Handle::weak_from_u128(101),
         bold_handle:Handle::weak_from_u128(101),
         italicbold_handle:Handle::weak_from_u128(101),
        }
    }
}

impl BevyBackend {
    /// Creates a new BevyBackend with the specified width and height.
    pub fn new(width: u16, height: u16, font_size: u16, font_aspect_ratio:f32,   normal_font_path: &str,
        italic_font_path: &str,
        bold_font_path: &str,
        italicbold_font_path: &str,) -> BevyBackend {
        BevyBackend {
            height: height,
            width: width,
            term_font_size: font_size,
            entity_map: HashMap::new(),
            buffer: Buffer::empty(Rect::new(0, 0, width, height)),
            prev_buffer: Buffer::empty(Rect::new(0, 0, width, height)),
            vcupdate: Vec::default(),
            cursor: false,
            cursor_pos: (0, 0),
            font_aspect_ratio: font_aspect_ratio,
            bevy_initialized: false,
            normal_font_path: normal_font_path.to_string(),
    italic_font_path: italic_font_path.to_string(),
    bold_font_path: bold_font_path.to_string(),
    italicbold_font_path: italicbold_font_path.to_string(),
    normal_handle:Handle::weak_from_u128(101),
    italic_handle:Handle::weak_from_u128(101),
    bold_handle:Handle::weak_from_u128(101),
    italicbold_handle:Handle::weak_from_u128(101),
        }
    }

    /// Resizes the BevyBackend to the specified width and height.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.prev_buffer = self.buffer.clone();
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
            let vc = VirtualCell::to_virtual(x, y, c);

            self.vcupdate.push((x, y, vc));

            let cell = self.buffer.get_mut(x, y);
            *cell = c.clone();

            // println!("{} {}", x, y);
            //  println!("{:?}", c);
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
        let window_pixel_size: Size = Size {
            width: 600,
            height: 480,
        };
        Ok(WindowSize {
            columns_rows: (self.width, self.height).into(),
            pixels: window_pixel_size,
        })
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}
