use bevy::prelude::*;
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
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(BoardPlugin)
        .add_plugins(PiecesPlugin)
        .add_plugins(InputPlugin)
        // .add_plugin(UIPlugin)
        .add_plugins(Shape2dPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

// TODO: Make turn plugin

fn setup(mut commands: Commands) {
    let val = SIZE * (MAX as f32 * 0.5);

    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(val, val, 0.)),
        ..Default::default()
    });
}
