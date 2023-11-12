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
        .add_systems(Update, (click_input, update_turn_text));
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

fn spawn_turn_text(mut commands: Commands, manager: Res<TurnManager>) {
    commands.spawn((
        TextBundle::from_section(
            format!("{:?} player turn !", *manager),
            TextStyle {
                font_size: 50.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            right: Val::Px(400.0),
            ..default()
        }),
        RoleText,
    ));
}
fn update_turn_text(mut query: Query<&mut Text, With<TurnText>>, manager: Res<TurnManager>) {
    if manager.is_changed() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{:?} player turn !", *manager);
        }
    }
}

fn click_input(
    manager: Res<TurnManager>,
    mut selection: ResMut<Selection>,
    mouse_button_input: Res<Input<MouseButton>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    query: Query<&Piece>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        selection.old = Vec2::NEG_ONE;
        selection.new = Vec2::NEG_ONE;
        return;
    }

    const HALF: f32 = SIZE as f32 * 0.5;
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_pos) = windows.single().cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera_query.single();
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };

    if !inside_board(point.x, point.y) {
        return;
    }

    let no_selection = selection.old.x < 0. || selection.old.y < 0.;
    let pos = square_center(point.x, point.y) / SIZE;

    if no_selection {
        for piece in query.iter() {
            let min = Vec2::new(
                (piece.x as f32 * SIZE) - HALF,
                (piece.y as f32 * SIZE) - HALF,
            );
            let max = Vec2::new(
                (piece.x as f32 * SIZE) + HALF,
                (piece.y as f32 * SIZE) + HALF,
            );

            if point.x > min.x
                && point.x < max.x
                && point.y > min.y
                && point.y < max.y
                && manager.same_color(piece.color)
            {
                selection.old = pos;
                break;
            }
        }
    } else {
        if pos != selection.old {
            selection.new = pos;
        }
    }
}
