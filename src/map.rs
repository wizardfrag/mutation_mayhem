use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::GameState;
use crate::loading::LevelAssets;

pub struct MapPlugin;

#[derive(Bundle, LdtkEntity)]
pub struct MapBundle {
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle
}

fn setup(mut commands: Commands, levels: Res<LevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: levels.testlevel.clone(),
        ..Default::default()
    });
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LdtkPlugin)
            .add_startup_system(setup.in_schedule(OnEnter(GameState::Playing)))
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_entity::<MapBundle>("Map")
            .run();
    }
}