use std::{
    error::Error,
    io,
    time::{Duration, Instant},
    sync::Arc
};

use bevy::app::AppExit;
use bevy::app::App as BevyApp;
use bevy::prelude::*;

use ratatui::prelude::*;

use crate::{app::App as RatApp, ui};




#[derive(Resource)]
struct RatAppCont{
    rat_app:&'static Arc<RatApp<'static>>,
    tick_rate:Duration,


}


pub fn run(ticky_rate: Duration, enhanced_graphics: bool)  {

   


    // create app and run it
    

    BevyApp::new()
    .add_plugins(DefaultPlugins)
    .add_plugins((RatatuiPlugin))
    .add_systems(PreStartup,create_rat_app)
    .add_systems(Startup, camera_setup)
    .add_systems(PreUpdate, terminal_draw)
    .add_systems(Update, (keyboard_input))
    .run();
  

}


fn create_rat_app(mut commands: Commands){

    let ratty = Arc::new(RatApp::new("Crossterm Demo", enhanced_graphics));

 

    let meow: RatAppCont= RatAppCont{rat_app:&ratty.clone(),tick_rate:ticky_rate};


    commands.insert_resource(meow);






}





fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let mut my_terminal = Terminal::new(BevyBackend::new(60, 20,40)).unwrap();


    my_terminal.clear();

    my_terminal.show_cursor();


    commands.spawn(my_terminal);
}

fn terminal_draw(mut terminal_query:  Query<(&mut Terminal<BevyBackend>)>, mut rat_app_cont: ResMut<RatAppCont>) {
    let mut last_tick = Instant::now();

    let mut rat_term = terminal_query.get_single_mut().expect("More than one terminal with a bevybackend");

    let _ = rat_term.draw(|f| ui::draw(f, &mut rat_app_cont.rat_app));


    if last_tick.elapsed() >= rat_app_cont.tick_rate {
        rat_app_cont.rat_app.on_tick();
        last_tick = Instant::now();
    }
}

fn keyboard_input(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>, mut rat_app_cont: ResMut<RatAppCont>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        exit.send(AppExit);
    }
    if keys.just_pressed(KeyCode::KeyH) {
        rat_app_cont.rat_app.on_left();
    }
    if keys.just_pressed(KeyCode::KeyK) {
        rat_app_cont.rat_app.on_up();
    }
    if keys.just_pressed(KeyCode::KeyL) {
        rat_app_cont.rat_app.on_right();
    }
    if keys.just_pressed(KeyCode::KeyJ) {
        rat_app_cont.rat_app.on_down();
    }
    if keys.just_pressed(KeyCode::KeyC) {
        rat_app_cont.rat_app.on_key("c".chars().next().unwrap());
    }
    if keys.just_pressed(KeyCode::KeyT) {
        rat_app_cont.rat_app.on_key("t".chars().next().unwrap());
    }
}





