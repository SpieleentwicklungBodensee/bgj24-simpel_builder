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
                scale: Vec3::new(5., 5., 1.),
                translation: Vec3::new(1.0, 1.0, 0.1),
                ..default()
            },
            ..default()
        },
        Player{},
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

pub fn set_target(
    mut mydcords: ResMut<MyWorldCords>,
    mut player: Query<(&Transform, &Curser, &mut Trarget)>,
    window: Query<&Window, With<PrimaryWindow>>,
    cam: Query<(&Camera, &GlobalTransform), With<MainCam>>,
){
    for (transform, _player, mut target) in &mut player {

        let (cameera, camera_tranform) = cam.single();

        if let Some(pos) = window.single().cursor_position()
            .and_then(|curser| cameera.viewport_to_world(camera_tranform, curser ))
            .map(|ray| ray.origin.truncate())
        {
            mydcords.vec2 = pos;
            target.vec2.x = pos.x;
            target.vec2.y = pos.y;
        }

    }
}

pub fn show_target(
    mut target: Query<(&Trarget, &mut Transform, &Curser)>
){
    for (target_c, mut transform, _curser) in &mut target {
        transform.translation.x = target_c.vec2.x;
        transform.translation.y = target_c.vec2.y;
    }
}

pub fn spawn_target(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
){
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle{radius: 2.5}).into(),
            transform: Transform {
                translation: Vec3::new(0.0, -5.0, 1.0),
                ..default()
            },
            material: materials.add( Color::srgb(0.0, 255.0, 0.0)),
            ..default()
        },
        Curser{},
        Trarget{
            vec2: Vec2::new(0.0, 0.0)
        }
        ));
}
