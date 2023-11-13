use super::*;

#[derive(Default, Resource)]
pub struct Selection {
    pub old: Vec2,
    pub new: Vec2,
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Selection {
            old: Vec2::NEG_ONE,
            new: Vec2::NEG_ONE,
        })
        .insert_resource(TurnManager::default())
        .add_systems(Startup, spawn_turn_text)
        .add_systems(Update, (click_input, bevy::window::close_on_esc))
        .add_systems(FixedUpdate, update_turn_text);
    }
}

#[derive(Resource, Default, PartialEq, Debug)]
pub struct TurnManager(PieceColor);

impl TurnManager {
    pub fn next_turn(&mut self) {
        match self.0 {
            PieceColor::Black => self.0 = PieceColor::White,
            PieceColor::White => self.0 = PieceColor::Black,
        }
    }
    pub fn same_color(&self, color: PieceColor) -> bool {
        self.0 == color
    }
}

#[derive(Component)]
struct TurnText;

fn spawn_turn_text(mut commands: Commands) {
    commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection::default()],
                alignment: TextAlignment::Center,
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,

                bottom: Val::Percent(90.0),
                right: Val::Percent(35.0),
                left: Val::Percent(35.0),

                width: Val::Percent(30.0),
                ..default()
            },
            ..default()
        },
        TurnText,
    ));
}

fn update_turn_text(
    mut query: Query<&mut Text, With<TurnText>>,
    windows: Query<&Window>,
    turn_manager: Res<TurnManager>,
) {
    if !turn_manager.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:?} player turn", turn_manager.0);
        text.sections[0].style.font_size = windows.single().resolution.width() * 0.032;
    }
}

fn click_input(
    mut selection: ResMut<Selection>,
    turn_manager: Res<TurnManager>,
    mouse_button_input: Res<Input<MouseButton>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    pieces: Query<&Piece>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        selection.old = Vec2::NEG_ONE;
        selection.new = Vec2::NEG_ONE;
        return;
    }

    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_pos) = windows.single().cursor_position() else {
        return;
    };

    let (camera, camera_transform) = cameras.single();
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    if !inside_board(point.x, point.y) {
        return;
    }

    let pos = square_center(point.x, point.y) / SIZE;

    if selection.old == Vec2::NEG_ONE {
        for piece in pieces.iter() {
            let center = Vec2::new(piece.x as f32, piece.y as f32) * SIZE;
            let min = center - HALF_SIZE;
            let max = center + HALF_SIZE;

            let inside_square =
                point.x > min.x && point.y > min.y && point.x < max.x && point.y < max.y;

            if inside_square && turn_manager.same_color(piece.color) {
                selection.old = pos;
                break;
            }
        }
    } else if pos != selection.old
        && valid_path(
            (
                (selection.old.x / SIZE) as u8,
                (selection.old.y / SIZE) as u8,
            ),
            (
                (selection.new.x / SIZE) as u8,
                (selection.new.y / SIZE) as u8,
            ),
            pieces,
        )
    {
        selection.new = pos;
    }
}

fn valid_path(from: (u8, u8), to: (u8, u8), pieces: Query<&Piece>) -> bool {
    return true;

    // NOTE: This code is stolen
    // let x_diff = (self.x - to.0).abs();
    // let y_diff = (self.y - to.1).abs();
    // let same_x = self.x == to.0;
    // let same_y = self.y == to.1;
    // let path_empty = is_path_empty((self.x, self.y), to, &pieces);

    // match self.my_type {
    //     PieceType::King => {
    //         path_empty
    //             && ((x_diff == 1 && same_y)
    //                 || (y_diff == 1 && same_x)
    //                 || (x_diff == 1 && y_diff == 1))
    //     }
    //     PieceType::Queen => {
    //         path_empty && (x_diff == y_diff || ((same_x && !same_y) || (same_y && !same_x)))
    //     }
    //     PieceType::Bishop => path_empty && x_diff == y_diff,
    //     PieceType::Knight => (x_diff == 2 && y_diff == 1) || (x_diff == 1 && y_diff == 2),
    //     PieceType::Rook => path_empty && ((same_x && !same_y) || (same_y && !same_x)),
    //     PieceType::Pawn => {
    //         let color = piece_color(to, &pieces);

    //         if self.color == PieceColor::White {
    //             if same_x && path_empty {
    //                 if self.y == 1 {
    //                     if y_diff == 2 {
    //                         if color.is_none() {
    //                             return true;
    //                         }
    //                     }
    //                 } else {
    //                     if y_diff == 1 {
    //                         if color.is_none() {
    //                             return true;
    //                         }
    //                     }
    //                 }
    //             } else if to.0 - self.x == 1 && y_diff == 1 {
    //                 if color == Some(PieceColor::Black) {
    //                     return true;
    //                 }
    //             }
    //         } else {
    //             if same_x && path_empty {
    //                 if self.y == 6 {
    //                     if y_diff == -2 {
    //                         if color.is_none() {
    //                             return true;
    //                         }
    //                     }
    //                 } else {
    //                     if y_diff == -1 {
    //                         if color.is_none() {
    //                             return true;
    //                         }
    //                     }
    //                 }
    //             } else if to.0 - self.x == -1 && y_diff == 1 {
    //                 if color == Some(PieceColor::White) {
    //                     return true;
    //                 }
    //             }
    //         }

    //         false
    //     }
    // }
}
