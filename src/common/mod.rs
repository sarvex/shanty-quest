use crate::{common::prelude::*, game::prelude::BoatSystem};
use audio_plus::AudioPlusPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use global_state::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CommonSystem {
    SafeToStateChange,
}

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(AudioPlusPlugin)
            .add_plugin(menu_bar::MenuBarPlugin)
            .add_plugin(mouse::MousePlugin)
            .add_plugin(assets::AssetsPlugin)
            .add_plugin(transform2::Transform2Plugin)
            .add_plugin(ldtk::LdtkPlugin)
            .add_plugin(entity_debug::EntityDebugPlugin)
            .add_plugin(y_depth::YDepthPlugin)
            .add_plugin(collision::CollisionPlugin)
            .add_plugin(time_to_live::TimeToLivePlugin)
            .add_plugin(clickable::ClickablePlugin)
            .add_plugin(screen_fade::ScreenFadePlugin)
            .add_plugin(cutscene::CutscenePlugin)
            .add_plugin(dialogue::DialoguePlugin)
            .add_plugin(follow_camera::FollowCameraPlugin)
            .add_plugin(world_locations::WorldLocationsPlugin)
            .add_plugin(map_builder::MapBuilderPlugin)
            .add_plugin(force_camera_ratio::ForceRatioPlugin)
            .add_plugin(wasm::WasmPlugin)
            .add_plugin(volume_control::VolumeControlPlugin)
            .add_global_state::<app_state::AppState>()
            .init_resource::<asset_library::AssetLibrary>()
            .add_system(nan_fix.in_base_set(CoreSet::PreUpdate))
            .add_system(safe_to_state_change.after(BoatSystem::Spawn));
    }
}

fn nan_fix(mut query: Query<&mut Transform2>) {
    for mut transform in query.iter_mut() {
        if !transform.translation.is_finite() {
            transform.translation = Vec2::new(800., -350.);
        }
    }
}

fn safe_to_state_change() {}

pub mod app_state;
pub mod asset_library;
pub mod assets;
pub mod clickable;
pub mod collision;
pub mod cutscene;
pub mod depth_layers;
pub mod dialogue;
pub mod easing;
pub mod entity_debug;
pub mod facing;
pub mod follow_camera;
pub mod force_camera_ratio;
pub mod label;
pub mod ldtk;
pub mod map_builder;
pub mod math;
pub mod menu_bar;
pub mod mouse;
pub mod prelude;
pub mod screen_fade;
pub mod sound_effects;
pub mod time_to_live;
pub mod timed_chance;
pub mod transform2;
pub mod volume_control;
pub mod wasm;
pub mod world_locations;
pub mod y_depth;
