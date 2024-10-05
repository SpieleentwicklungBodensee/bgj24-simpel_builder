use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::IntoSystemConfigs;
use crate::component::{MyWorldCords, PlayerPlugin};
use crate::player::player_main::*;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MyWorldCords>()
        .add_systems(Startup, spawn_target)
            .add_systems(Update, (set_target, show_target).chain())
            .add_systems(Update, move_p)
            .add_systems(Startup, spawn);
    }
}
