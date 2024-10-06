use bevy::prelude::*;

pub const SPEED:f32 = 200.0;
pub const RANGE:f32 = 50.0;
pub const IMGE_SCALE:Vec3 = Vec3::new(5.0, 5.0, 1.0);
pub const CAM_LERP_FACKTOR:f32 = 2.;
pub const INVETORI_TEXE_SICE:f64 = 60.0;

#[derive(Resource, Default)]
pub struct MyWorldCords{
    pub vec2: Vec2
}

#[derive(Component, PartialEq)]
pub enum ColMaterial{
    None,
    Wood,
    Stone,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerPlugin;

#[derive(Component)]
pub struct ColektebelsPlugin;
#[derive(Component)]
pub struct PlacePlugin;

#[derive(Component)]
pub struct Target{
    pub vec2: Vec2
}

#[derive(Component)]
pub struct Curser{}

#[derive(Component)]
pub struct MainCam{}

#[derive(Component)]
pub struct Inventory{
    pub wood:u8,
    pub max_wood:u8,
    pub stone:u8,
    pub max_stone:u8,
    pub none:u8,
}

#[derive(Component)]
pub struct InventoryText{
    pub material: ColMaterial
}

#[derive(Component)]
pub struct HouseSpawner{
    pub build: String,
    pub w_state: u32,
    pub s_state: u32,
    pub w_finish: u32,
    pub s_finish: u32,
    pub code: u8
}

#[derive(Component)]
pub struct HuoseSpawnerText{
    pub material: ColMaterial,
    pub code: u8
}

#[derive(Component)]
pub struct House{}
