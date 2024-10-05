use bevy::prelude::*;

pub const SPEED:f32 = 200.0;
pub const RANGE:f32 = 50.0;

#[derive(Resource, Default)]
pub struct MyWorldCords{
    pub vec2: Vec2
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct PlayerPlugin{}

#[derive(Component)]
pub struct Trarget{
    pub vec2: Vec2
}

#[derive(Component)]
pub struct Curser{}

#[derive(Component)]
pub struct MainCam{}