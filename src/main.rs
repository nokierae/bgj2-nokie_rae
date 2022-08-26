use bevy::prelude::*;

use bevy::asset::AssetServer;
use bevy::ecs::{component::Component, system::Commands, system::Res};
use bevy::scene::SceneBundle;
use bevy::utils::default;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

#[derive(Reflect, Component, Default)]
struct Player;

fn load_scene(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn_bundle(HookedSceneBundle {
        hook: SceneHook::new(|entity, cmds| {
            match entity.get::<Name>().map(|t| t.as_str()) {
                Some("Player") => cmds.insert(Player),
                _ => cmds,
            };
        }),
        scene: SceneBundle {
            scene: asset_server.load("scenes/level1.glb#Scene0"),
            ..default()
        },
    });
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.19, 0.16, 0.2)))
        .add_plugin(HookPlugin)
        .add_startup_system(load_scene);

    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());

    app.run();
}
