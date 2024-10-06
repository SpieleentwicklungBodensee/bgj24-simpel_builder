use crate::component::*;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::PrimaryWindow;


pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        SpriteBundle{
            texture: asset_server.load("player.png"),
            transform: Transform {
                scale: IMGE_SCALE,
                translation: Vec3::new(1.0, 1.0, 3.0),
                ..default()
            },
            ..default()
        },
        Player,
        Inventory{
            wood: 0,
            max_wood: 3,
            stone: 0,
            max_stone: 2,
            none: 0,
        }
    ));

}

pub fn move_p(
    mut player: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
){
    for (mut transform, _player) in &mut player {
        if input.pressed(KeyCode::KeyW) {transform.translation.y += SPEED * time.delta_seconds()}
        if input.pressed(KeyCode::KeyS) {transform.translation.y -= SPEED * time.delta_seconds()}
        if input.pressed(KeyCode::KeyA) {transform.translation.x -= SPEED * time.delta_seconds()}
        if input.pressed(KeyCode::KeyD) {transform.translation.x += SPEED * time.delta_seconds()}
    }
}