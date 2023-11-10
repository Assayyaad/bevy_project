use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
// use bevy_mod_picking::*;

// mod pieces;
// use pieces::*;
mod board;
use board::*;
// mod ui;
// use ui::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::YELLOW_GREEN))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.65,
        })
        // .init_resource::<PickingCamera>()
        // .add_plugins(DefaultPickingPlugins)
        .add_plugins(DefaultPlugins)
        .add_plugins(BoardPlugin)
        // .add_plugins(PiecesPlugin)
        // .add_plugin(UIPlugin)
        .add_plugins(Shape2dPlugin::default())
        .add_systems(Startup, setup)
        // .add_systems(FixedUpdate, keyboard_input)
        .run();
}

fn setup(mut commands: Commands) {
    let val = SIZE * (MAX as f32 * 0.5);
    // .insert(PickableBundle::default());

    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(val, val, 0.)),
        ..Default::default()
    });
}

// fn keyboard_input(keyboard_input: Res<Input<KeyCode>>, query: Query<&Piece>) {
//     if keyboard_input.pressed(KeyCode::Space) {
//         for piece in query.iter() {
//             println!(
//                 "{:?} {:?} is at position {:?},",
//                 piece.colour, piece.name, piece.pos
//             );
//         }
//     }
// }
