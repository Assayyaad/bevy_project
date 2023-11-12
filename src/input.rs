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
        .insert_resource(CursorWorld::default())
        .insert_resource(TurnManager::default())
        .add_systems(Startup, spawn_turn_text)
        .add_systems(
            Update,
            (
                select_area,
                cursor_window_to_world,
                bevy::window::close_on_esc,
            ),
        )
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

                bottom: Val::Percent(80.0),
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

// mouse pos hundler

#[derive(Resource, Default)]
pub struct CursorWorld {
    pub pos: Vec2,
}

fn cursor_window_to_world(
    mut cursor_world: ResMut<CursorWorld>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
) {
    let Some(cursor_pos) = windows.single().cursor_position() else {
        return;
    };
    let (camera, camera_transform) = camera_query.single();
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else {
        return;
    };
    cursor_world.pos = point;
}

fn select_area(
    mut selection: ResMut<Selection>,
    turn_manager: Res<TurnManager>,
    query: Query<&Piece>,
    cursor_world: Res<CursorWorld>,
    mouse_button_input: Res<Input<MouseButton>>,
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

    let point = cursor_world.pos;
    if !inside_board(point.x, point.y) {
        return;
    }

    let no_selection = selection.old.x < 0. || selection.old.y < 0.;
    let pos = square_center(point.x, point.y) / SIZE;

    for piece in query.iter() {
        let min = Vec2::new(
            (piece.x as f32 * SIZE) - HALF,
            (piece.y as f32 * SIZE) - HALF,
        );
        let max = Vec2::new(
            (piece.x as f32 * SIZE) + HALF,
            (piece.y as f32 * SIZE) + HALF,
        );
        let piece_boarder =
            point.x > min.x && point.y > min.y && point.x < max.x && point.y < max.y;

        if no_selection {
            if piece_boarder && turn_manager.same_color(piece.color) {
                selection.old = pos;
                break;
            }
        } else if pos != selection.old {
            if piece_boarder && turn_manager.same_color(piece.color) {
                selection.new = Vec2::NEG_ONE;
                break;
            } else {
                selection.new = pos;
            }
        }
    }
}
