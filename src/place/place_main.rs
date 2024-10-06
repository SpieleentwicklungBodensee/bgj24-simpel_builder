use bevy::prelude::*;
use crate::component::*;

pub fn spawn_house(
    mut command: Commands,
    asset_server: Res<AssetServer>,
){
    command.spawn((
        SpriteBundle{
            texture: asset_server.load("house_build.png"),
            transform: Transform{
                translation: Vec3::new(-250.0, -100.0, 1.0),
                scale: IMGE_SCALE,
                ..default()
            },
            ..default()
        },
        HouseSpawner{
            build: String::from("house.png"),
            w_state: 0,
            w_finish: 5,
            s_state: 0,
            s_finish: 0,
            code: 1
        }
    ));
}

pub fn spawn_big_house(
    mut command: Commands,
    asset_server: Res<AssetServer>,
){
    command.spawn((
        SpriteBundle{
            texture: asset_server.load("house_build.png"),
            transform: Transform{
                translation: Vec3::new(150.0, -100.0, 1.0),
                scale: IMGE_SCALE,
                ..default()
            },
            ..default()
        },
        HouseSpawner{
            build: String::from("big_house.png"),
            w_state: 0,
            w_finish: 10,
            s_state: 0,
            s_finish: 10,
            code: 2
        }
    ));
}


pub fn build(
    asset_server: Res<AssetServer>,
    mut command: Commands,
    mut q_spawner: Query<(Entity, &mut HouseSpawner, &Transform)>,
    mut q_player: Query<(&Player, &Transform, &mut Inventory)>
){
    let (_player, p_transform, mut inventory) = q_player.single_mut();

    for (entity, mut spawner, s_transfrorm) in &mut q_spawner {

        let mut stone:bool = false;
        let mut wood:bool = false;

        if spawner.w_finish == spawner.w_state {wood = true}
        if spawner.s_state == spawner.s_finish {stone = true}
        if wood && stone{
            command.entity(entity).despawn_recursive();
            command.spawn((SpriteBundle{
                texture: asset_server.load(spawner.build.clone()),
                transform: Transform{
                    translation: Vec3::new(s_transfrorm.translation.x, s_transfrorm.translation.y, 2.0),
                    scale: IMGE_SCALE,
                    ..default()
                },
                ..default()
            },
                House{}
            ));
        }else {
            if p_transform.translation.x < s_transfrorm.translation.x + 25. && p_transform.translation.x > s_transfrorm.translation.x -25. {
                if p_transform.translation.y < s_transfrorm.translation.y + 25. && p_transform.translation.y > s_transfrorm.translation.y -25. {
                    if inventory.wood > 0 && !wood {
                        inventory.wood -= 1;
                        spawner.w_state += 1
                    }
                    if inventory.stone > 0 && !stone {
                        inventory.stone -= 1;
                        spawner.s_state += 1
                    }
                }
            }
        }
    }
}

pub fn spawn_compeshenlist(
   mut commands: Commands,
   q_spowner: Query<(&Transform, &HouseSpawner)>,
){

    let text_style = TextStyle {
        font_size: 20.0,
        ..default()
    };

   for (transform, spawner) in &q_spowner {
       if spawner.s_finish > 0 {
           commands.spawn((Text2dBundle{
               text: Text::from_section("stone 0/0", text_style.clone()),
               transform: Transform{
                   translation: Vec3::new(transform.translation.x, transform.translation.y +55.0 , 2.0 ),
                   ..default()
               },
                   ..default()
           },
               HuoseSpawnerText{
                   material: ColMaterial::Stone,
                   code: spawner.code
               },

           ));

       }
       if spawner.w_finish > 0 {
           commands.spawn(( Text2dBundle{
               text: Text::from_section("wood 0/0", text_style.clone()),
               transform: Transform{
                   translation: Vec3::new(transform.translation.x, transform.translation.y +35.0 , 2.0 ),
                   ..default()
               },
               ..default()
           },
                HuoseSpawnerText {
                    code: spawner.code,
                    material: ColMaterial::Wood
                }
           ));
       }
   }
}

pub fn update_list(
    q_bulding: Query<&HouseSpawner>,
    mut q_text: Query<(&HuoseSpawnerText, &mut Text, Entity)>,
    mut commands: Commands
){
    for inventory in &q_bulding {
        for (components, mut text, entity) in &mut q_text {

            if components.code == inventory.code {


               match components.material {
                   ColMaterial::Wood => {
                       if inventory.w_finish == inventory.w_state {
                           commands.entity(entity).despawn_recursive()
                       }
                       else {text.sections[0].value = format!("Wood: {}/{}", inventory.w_state, inventory.w_finish).to_string()}
                   },
                   ColMaterial::Stone => {
                       if inventory.s_finish == inventory.s_state {commands.entity(entity).despawn_recursive()}
                       else{text.sections[0].value = format!("Stone: {}/{}", inventory.s_state, inventory.s_finish).to_string()}
                   },
                   _ => ()
               }

            }
        }
    }
}