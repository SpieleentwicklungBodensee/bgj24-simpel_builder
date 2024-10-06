use bevy::prelude::*;
use crate::component::*;

pub fn update_cam(
    time: Res<Time>,
    mut cam: Query<&mut Transform, (With<MainCam>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<MainCam>)>
){
    let Ok(mut cam) = cam.get_single_mut() else { return; };

    let Ok(player) = player.get_single() else { return; };

    let Vec3 {x, y, ..} = player.translation;
    let direktion = Vec3::new(x, y, cam.translation.z);

    cam.translation = cam.translation
        .lerp(direktion, time.delta_seconds()*CAM_LERP_FACKTOR)
}