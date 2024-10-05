
mod component;
mod player;

use bevy::prelude::*;
use crate::component::{MainCam, PlayerPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PlayerPlugin{})
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut command: Commands
){
    command.spawn((Camera2dBundle::default(), MainCam{}));
}