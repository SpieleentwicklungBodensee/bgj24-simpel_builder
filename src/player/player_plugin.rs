use bevy::app::{App, Plugin, Startup, Update};
use crate::component::{MyWorldCords, PlayerPlugin};
use crate::player::player_inventory::*;
use crate::player::player_main::*;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MyWorldCords>()
            .add_systems(Startup, spawn_inventory_text)
            .add_systems(Update, move_p)
            .add_systems(Update, colect)
            .add_systems(Update, show_inventory)
            .add_systems(Startup, spawn);
    }
}
