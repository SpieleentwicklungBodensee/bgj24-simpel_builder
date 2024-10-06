use crate::component::*;
use bevy::prelude::*;


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
        Player{
            running: true,
            fram: 0,
            frame_count: 0
        },
        Inventory{
            wood: 0,
            max_wood: 3,
            stone: 0,
            max_stone: 2,
        }
    ));

}

pub fn move_p(
    mut q_player: Query<(&mut Transform, &mut Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>
){
    let (ref mut transform, ref mut player) = &mut q_player.single_mut();
    let mut  moved= false;
        if input.pressed(KeyCode::KeyW) {transform.translation.y += SPEED * time.delta_seconds(); moved=true }
        if input.pressed(KeyCode::KeyS) {transform.translation.y -= SPEED * time.delta_seconds(); moved=true }
        if input.pressed(KeyCode::KeyA) {transform.translation.x -= SPEED * time.delta_seconds(); moved=true }
        if input.pressed(KeyCode::KeyD) {transform.translation.x += SPEED * time.delta_seconds(); moved=true }
    if moved {
        player.running = true
    }else {player.running = false}
}

pub fn animation(
   asset_server: Res<AssetServer>,
   mut q_player: Query<(&mut Player, &mut Handle<Image>)>,
){
    let (ref mut player,ref mut image) = &mut q_player.single_mut();
    if player.running {
        player.frame_count += 1;

        if player.frame_count == 25 {
            player.frame_count = 0;

            match player.fram {
                2 => {
                    **image = asset_server.load("player_1.png");
                    player.fram = 1
                },
                1 => {
                    **image = asset_server.load("player_2.png");
                    player.fram = 2
                },
                0 => {
                    **image = asset_server.load("player_1.png");
                    player.fram = 1
                }
                _ => ()
            }

        }
    }
    if player.fram != 0 && !player.running {
        **image = asset_server.load("player.png");
        player.fram = 0;
        player.frame_count = 24;
    }

}