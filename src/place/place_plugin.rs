use bevy::prelude::*;
use crate::component::*;
use crate::place::place_main::*;

impl Plugin for PlacePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_house,spawn_compeshenlist).chain())
            .add_systems(Startup, spawn_big_house)
            .add_systems(Update, update_list)
            .add_systems(Update, build);
    }
}