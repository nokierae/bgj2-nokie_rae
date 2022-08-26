use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.19, 0.16, 0.2)));

    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());

    app.run();
}
