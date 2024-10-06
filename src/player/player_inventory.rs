use bevy::prelude::*;
use crate::component::*;

pub fn colect(
    mut q_player: Query<(&Player, &mut Inventory, &Transform)>,
    q_material: Query<(Entity, &ColMaterial, &Transform)>,
    mut command: Commands
){
    let (_player, mut inventory, p_pos) = q_player.single_mut();

    for (entety, material, m_pos) in &q_material {
       if p_pos.translation.x < m_pos.translation.x + 25. && p_pos.translation.x > m_pos.translation.x -25. {
           if p_pos.translation.y < m_pos.translation.y + 25. && p_pos.translation.y > m_pos.translation.y -25. {


                   let max_wood = inventory.max_wood;
                   let max_stone = inventory.max_stone;

                   match material {
                       ColMaterial::Wood => {
                           add_material(&mut inventory.wood, entety, &mut command, max_wood);
                       },
                       ColMaterial::Stone => {
                           add_material(&mut inventory.stone, entety, &mut command, max_stone);
                       },
                       _ => ()
                   };



           }
       }
    }
}

fn add_material(
    mut material:&mut u8,
    entity: Entity,
    mut command: &mut Commands,
    max: u8
) {
   if *material < max {
        *material +=1;
       command.entity(entity).despawn_recursive();
   }

}

pub fn spawn_inventory_text(
    mut commands: Commands,
){
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Wood: ",
                TextStyle {
                    font_size: 60.0,
                    color: Color::srgb(0.0, 0.0, 0.0),
                    ..default()
                },
            ),
            TextSection::from_style(
                TextStyle {
                    font_size: 60.0,
                    color: Color::srgb(0.0, 0.0, 0.0),
                    ..default()
                }
            ),
        ]),
        InventoryText{
            material: ColMaterial::Wood,
        },
    ));

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Stone: ",
                TextStyle {
                    font_size: 60.0,
                    color: Color::srgb(0.0, 0.0, 0.0),

                    ..default()
                },
            ),
            TextSection::from_style(
                TextStyle {
                    font_size: 60.0,
                    color: Color::srgb(0.0, 0.0, 0.0),

                    ..default()
                }
            ),
        ]),

        InventoryText{
            material: ColMaterial::Stone,
        },
    ));


}

pub fn show_inventory(
   mut q_text: Query<(&mut Text, &InventoryText)>,
   q_inventory: Query<(&Player, &Inventory)>
){
    let (_player, invertory) = q_inventory.single();

    for (mut text, i_text) in &mut q_text {
        match i_text.material {
            ColMaterial::Wood => text.sections[1].value = format!("{}", invertory.wood),
            ColMaterial::Stone => text.sections[1].value = format!("{}", invertory.stone),
            _ => ()
        }
    }
}