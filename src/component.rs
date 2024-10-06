use bevy::prelude::*;

pub const SPEED:f32 = 200.0;
pub const IMGE_SCALE:Vec3 = Vec3::new(5.0, 5.0, 1.0);
pub const CAM_LERP_FACKTOR:f32 = 2.;

#[derive(Resource, Default)]
pub struct MyWorldCords{
}

#[derive(Component, PartialEq)]
pub enum ColMaterial{
    Wood,
    Stone,
}

#[derive(Component)]
pub struct Player{
    pub running: bool,
    pub fram: u8,
    pub frame_count: u32
}

#[derive(Component)]
pub struct PlayerPlugin;

#[derive(Component)]
pub struct ColektebelsPlugin;
#[derive(Component)]
pub struct PlacePlugin;

#[derive(Component)]
pub struct MainCam{}

#[derive(Component)]
pub struct Inventory{
    pub wood:u8,
    pub max_wood:u8,
    pub stone:u8,
    pub max_stone:u8,
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
