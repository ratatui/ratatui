//! # [Ratatui] Hello Bevy example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

use bevy::{app::AppExit, prelude::*};
use ratatui::{
    prelude::{BevyBackend, RatatuiPlugin, Terminal, *},
    text::*,
    widgets::*,
};

/// This is a bare minimum example. There are many approaches to running a bevy program, so
/// this is not meant to be prescriptive. It is only meant to demonstrate the basic setup and
/// teardown of a bevy ratatui terminal application.

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((RatatuiPlugin))
        .add_systems(Startup, camera_and_terminal_setup)
        .add_systems(PreUpdate, terminal_draw)
        .add_systems(Update, (keyboard_input))
        .add_systems(Startup, (bevy_draw))
        .run();
}

fn camera_and_terminal_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let mut my_terminal = Terminal::new(BevyBackend::default()).unwrap();

    my_terminal.clear();

    commands.spawn(my_terminal);
}

fn terminal_draw(mut terminal_query: Query<(&mut Terminal<BevyBackend>)>, mut commands: Commands) {
    let text = "Hello Bevy! From Ratatui with love. :D   (press 'q' to quit)   ";

    let mut rat_term = terminal_query
        .get_single_mut()
        .expect("More than one terminal with a bevybackend");
    let _ = rat_term.draw(|frame| {
        let area = frame.size();
        frame.render_widget(text::Line::from(text), area);
    });

    for (pos, e) in rat_term.backend().entity_map.iter() {
        if pos.1 > 2 {
            commands.entity(e.clone()).insert(Visibility::Hidden);
        }
    }
}

fn bevy_draw(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("logo.png"),
        ..default()
    });
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        exit.send(AppExit);
    }
}
