use bevy::prelude::*;
use bevy::window::*;
use bevy_vector_shapes::prelude::*;

mod pieces;
use pieces::*;
mod board;
use board::*;
mod input;
use input::*;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.65,
        })
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(BoardPlugin)
        .add_plugins(PiecesPlugin)
        .add_plugins(InputPlugin)
        // .add_plugin(UIPlugin)
        .add_plugins(Shape2dPlugin::default())
        .add_systems(PreStartup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let val = SIZE * (MAX as f32 * 0.5);

    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(val, val, 0.)),
        ..Default::default()
    });
}
