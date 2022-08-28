use crate::common::prelude::*;
use asset_struct::AssetStruct;
use bevy::prelude::*;

#[derive(Default)]
struct LoadingState {
    fading: bool,
}

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadingState>()
            .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(loading_init))
            .add_system_set(SystemSet::on_update(AppState::Loading).with_system(loading_update));
    }
}

fn loading_init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    asset_library.load_assets(&asset_server);
    asset_library.create_texture_atlases(texture_atlas_assets.as_mut());
    asset_library.create_sound_effects();
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "Loading".to_owned(),
                TextStyle {
                    font: asset_library.font_default.clone(),
                    font_size: 68.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            }),
            ..Default::default()
        })
        .insert(Transform2::new().with_depth((DepthLayer::Front, 0.)));
}

fn loading_update(
    mut app_state: ResMut<State<AppState>>,
    asset_library: Res<AssetLibrary>,
    asset_server: Res<AssetServer>,
    mut screen_fade: ResMut<ScreenFade>,
    mut ev_dialogue_init: EventWriter<DialogueInitEvent>,
    mut state: ResMut<LoadingState>,
) {
    use bevy::asset::LoadState;
    match asset_library.load_state(&asset_server) {
        LoadState::Failed => {
            panic!("Failed to load assets.");
        }
        LoadState::Loaded => {
            if state.fading && screen_fade.faded_out() {
                app_state.set(AppState::TownOutside).unwrap();
                ev_dialogue_init.send_default();
            }
            if !state.fading {
                screen_fade.enable();
                screen_fade.set(0.);
                screen_fade.fade_out(0.1);
                state.fading = true;
            }
        }
        _ => {}
    }
}
