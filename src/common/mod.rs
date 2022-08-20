use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use global_state::prelude::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(menu_bar::MenuBarPlugin)
            .add_plugin(mouse::MousePlugin)
            .add_global_state::<app_state::AppState>()
            .init_resource::<asset_library::AssetLibrary>();
    }
}

pub mod app_state;
pub mod asset_library;
pub mod label;
pub mod menu_bar;
pub mod mouse;
pub mod prelude;
