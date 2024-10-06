use bevy::prelude::*;
use crate::component::*;

impl Plugin for ColektebelsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, spawn_wood)
            .add_systems(Update, spawn_stone)
            .add_systems(Startup, spawn_stone_one)
            .add_systems(Startup, spawn_wood_one);

    }
}

fn spawn_wood_one(
    mut commands: Commands,
    asset_sever: Res<AssetServer>
){

   let poses: [Vec3; 10] = [
       Vec3::new (100.0, 100.0, 2.0),
       Vec3::new (200.0, -100.0, 2.0),
       Vec3::new (300.0, 100.0, 2.0),
       Vec3::new (400.0, -100.0, 2.0),
       Vec3::new (500.0, 100.0, 2.0),
       Vec3::new (100.0, 150.0, 2.0),
       Vec3::new (200.0, 100.0, 2.0),
       Vec3::new (300.0, 250.0, 2.0),
       Vec3::new (400.0, 100.0, 2.0),
       Vec3::new (500.0, 150.0, 2.0),
   ];

    for i in poses {
        commands.spawn((
                           SpriteBundle{
                               texture : asset_sever.load("wood.png"),
                               transform : Transform{
                                   translation: i,
                                   scale: IMGE_SCALE,
                                   ..default()
                               },
                               ..default()
                           },
                           ColMaterial::Wood,
                       ));
    }
}

fn spawn_stone_one(
    mut commands: Commands,
    asset_sever: Res<AssetServer>
){

    let poses: [Vec3; 10] = [
        Vec3::new (-100.0, 100.0, 2.0),
        Vec3::new (-200.0, -100.0, 2.0),
        Vec3::new (-300.0, 100.0, 2.0),
        Vec3::new (-400.0, -100.0, 2.0),
        Vec3::new (-500.0, 100.0, 2.0),
        Vec3::new (-100.0, 150.0, 2.0),
        Vec3::new (-200.0, 100.0, 2.0),
        Vec3::new (-300.0, 250.0, 2.0),
        Vec3::new (-400.0, 100.0, 2.0),
        Vec3::new (-500.0, 150.0, 2.0),
    ];

    for i in poses {
        commands.spawn((
            SpriteBundle{
                texture : asset_sever.load("stone.png"),
                transform : Transform{
                    translation: i,
                    scale: IMGE_SCALE,
                    ..default()
                },
                ..default()
            },
            ColMaterial::Stone,
        ));
    }
}


pub fn spawn_wood(
    commands: Commands,
    asset_server: Res<AssetServer>,
    q_wood: Query<(&ColMaterial, &Transform)>
){

    let ore = 5;
    let mut wood = 0;

    for (material, _transform) in &q_wood {
        if material == &ColMaterial::Wood {
            wood+=1
        }
    }

    if wood<ore {
       spawn_wood_one(commands, asset_server)
    }

}

pub fn spawn_stone(
    commands: Commands,
    asset_server: Res<AssetServer>,
    q_wood: Query<(&ColMaterial, &Transform)>
){

    let ore = 5;
    let mut stone = 0;

    for (material, _transform) in &q_wood {
        if material == &ColMaterial::Stone {
            stone+=1
        }
    }

    if stone<ore {
        spawn_stone_one(commands, asset_server)
    }

}