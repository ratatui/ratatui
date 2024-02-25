use std::{error::Error, time::Duration};

use bevy::{
    app::{App as BevyApp, AppExit},
    prelude::*,
};
use once_cell::sync::Lazy;
use ratatui::prelude::*;

use crate::{app::App as RatApp, ui};

static mut RATAPP: Lazy<RatApp> = Lazy::new(|| RatApp::new("BEVY Demo", true));

unsafe fn get_ratapp() -> &'static mut RatApp<'static> {
    return &mut RATAPP;
}

pub fn run(ticky_rate: Duration, enhanced_graphics: bool) -> Result<(), Box<dyn Error>> {
    let ratapp = RatApp::new("Bevy Demo", enhanced_graphics);

    let mut ra = unsafe { get_ratapp() };
    *ra = ratapp;

    BevyApp::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((RatatuiPlugin))
        .insert_resource(Time::<Fixed>::from_duration(ticky_rate))
        .add_systems(Startup, camera_setup)
        .add_systems(PreUpdate, terminal_draw)
        .add_systems(FixedUpdate, app_tick)
        .add_systems(Update, (keyboard_input))
        .run();

    Ok(())
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let mut my_terminal = Terminal::new(BevyBackend::default()).unwrap();

    /*
    or something like

    let mut my_terminal = Terminal::new(BevyBackend::new(
        30,
        30,
        40,
        "fonts/Iosevka-Regular.ttf",
        "fonts/Iosevka-Oblique.ttf",
        "fonts/Iosevka-Bold.ttf",
        "fonts/Iosevka-BoldOblique.ttf",
    )).unwrap();



     */

    my_terminal.clear();

    my_terminal.show_cursor();

    commands.spawn(my_terminal);
}

fn terminal_draw(mut terminal_query: Query<(&mut Terminal<BevyBackend>)>) {
    let mut ra = unsafe { get_ratapp() };
    let mut rat_term = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");

    let _ = rat_term.draw(|f| ui::draw(f, &mut ra));
}

fn app_tick() {
    let mut ra = unsafe { get_ratapp() };
    ra.on_tick();
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    let ra = unsafe { get_ratapp() };
    if keys.just_pressed(KeyCode::KeyQ) {
        exit.send(AppExit);
    }
    if keys.just_pressed(KeyCode::KeyH) {
        ra.on_left();
    }
    if keys.just_pressed(KeyCode::KeyK) {
        ra.on_up();
    }
    if keys.just_pressed(KeyCode::KeyL) {
        ra.on_right();
    }
    if keys.just_pressed(KeyCode::KeyJ) {
        ra.on_down();
    }
    if keys.just_pressed(KeyCode::KeyC) {
        ra.on_key("c".chars().next().unwrap());
    }
    if keys.just_pressed(KeyCode::KeyT) {
        ra.on_key("t".chars().next().unwrap());
    }
}
