use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::GameState;
use crate::loading::LevelAssets;

pub struct MapPlugin;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,

    pub player: Player,

    #[worldly]
    pub worldly: Worldly,
}

fn setup(mut commands: Commands, levels: Res<LevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: levels.testlevel.clone(),
        ..Default::default()
    });
}

fn entity_added(
    mut query: Query<&mut Transform, With<Player>>
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(90.0);
    }
}

fn camera_follow_player(
    mut cam_query: Query<&mut Transform, With<Camera2d>>,
    player: Query<&Transform, (With<Player>, Without<Camera2d>)>
) {
    for mut transform in cam_query.iter_mut() {
        let Ok(player_transform) = player.get_single() else { return };
        transform.translation.x = player_transform.translation.x;
        transform.translation.y = player_transform.translation.y;
    }
}

fn player_movement(keeb: Res<Input<KeyCode>>, mut player: Query<&mut Transform, With<Player>>) {
    for mut player in player.iter_mut() {
        if keeb.any_pressed([KeyCode::A, KeyCode::Left]) {
            player.translation.x -= 5.0;
        }
    }
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LdtkPlugin)
            .add_plugin(WorldInspectorPlugin::new())
            // .add_startup_system(setup)
            .add_system(setup.in_schedule(OnEnter(GameState::Playing)))
            .add_system(entity_added)
            .add_system(camera_follow_player)
            .add_system(player_movement)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_entity::<PlayerBundle>("Player")
            .run();
    }
}