
mod component;
mod player;
mod materials;
mod camera;
mod place;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use crate::component::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.3, 0.6, 0.0)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PlayerPlugin{})
        .add_plugins(ColektebelsPlugin)
        .add_plugins(PlacePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, camera::update_cam)
        .run();
}

fn setup(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    command.spawn((Camera2dBundle::default(), MainCam{}));

    command.spawn(
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle{radius: 2.5}).into(),
            transform: Transform {
                translation: Vec3::new(0.0, -5.0, 1.0),
                ..default()
            },
            material: materials.add( Color::srgb(0.0, 0.0, 255.0)),
            ..default()
        },
    );

}