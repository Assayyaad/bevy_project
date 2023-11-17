use super::*;

//FIX: replace its by Selected Pieces 
#[derive(Default, Resource)]
pub struct Selection {
    pub from: Vec2,
    pub to: Vec2,
}

#[derive(Resource, Default)]
pub struct CursorWorld {
    pub pos: Vec2,
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Selection {
            from: Vec2::NEG_ONE,
            to: Vec2::NEG_ONE,
        })
        .insert_resource(CursorWorld::default())
        .insert_resource(TurnManager::default())
        .add_systems(Startup, spawn_turn_text)
        .add_systems(
            Update,
            (
                selection,
                cursor_window_to_world,
                bevy::window::close_on_esc,
            ))
        .add_systems(PostUpdate, (
            print_name_of_selected_pieces,
            draw_path,
        ))
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
    turn: Res<TurnManager>,
) {
    if !turn.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:?} player turn", turn.0);
        text.sections[0].style.font_size = windows.single().resolution.width() * 0.032;
    }
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

fn selection(
    mut commands: Commands,
    mut selected_pieces: ResMut<SelectedPieces>,
    from_query: Query<&From>,
    to_query: Query<&To>,
    //FIX: we dont need this anymore
    mut selection: ResMut<Selection>,
    turn: Res<TurnManager>,

    query: Query<(&Piece, Entity)>,
    cursor_world: Res<CursorWorld>,
    mouse_button_input: Res<Input<MouseButton>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        selection.from = Vec2::NEG_ONE;
        selection.to = Vec2::NEG_ONE;
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

    let no_selection = selection.from.x < 0. || selection.from.y < 0.;
    let pos = square_center(point.x, point.y) / SIZE;

    for (piece, id) in query.iter() {
        let min = Vec2::new(
            (piece.x as f32 * SIZE) - HALF,
            (piece.y as f32 * SIZE) - HALF,
        );
        let max = Vec2::new(
            (piece.x as f32 * SIZE) + HALF,
            (piece.y as f32 * SIZE) + HALF,
        );
        let inside_piece_boarder =
            point.x > min.x && point.y > min.y && point.x < max.x && point.y < max.y;

        if no_selection {
            if inside_piece_boarder && turn.same_color(piece.color) {
                selected_pieces.from(id, &mut commands, &from_query);
                //FIX: delete it no need to handle the logic of pos here  
                selection.from = pos;
                break;
            }
        } else if pos != selection.from {
            if inside_piece_boarder && turn.same_color(piece.color) {
                selected_pieces.from(id, &mut commands, &from_query);
                //FIX: delete it no need to handle the logic of pos here  
                selection.from = pos;
                selection.to = Vec2::NEG_ONE;
                break;
            } 
            else if inside_piece_boarder && !turn.same_color(piece.color) {
                selected_pieces.to(id, &mut commands, &to_query);
                //FIX: delete it no need to handle the logic of pos here  
                selection.to = pos;
            } 
            else if !inside_piece_boarder {
                selected_pieces.remove_from(&mut commands, &from_query);
                selected_pieces.remove_to(&mut commands, &to_query);
                //FIX: delete it no need to handle the logic of pos here  
                selection.to = pos;
            }
        }
    }
}


fn print_name_of_selected_pieces(
    res : Res<SelectedPieces>,
    from: Query<&Piece, (With<From>, Without<pieces::To>)>,
    to: Query<&Piece, (With<To>, Without<pieces::From>)>,
){
    if res.is_changed() {
        let Ok(from_piece) = from.get_single() else {return};
        let Ok(to_piece) = to.get_single() else {return};

        println!(" from : {:?}", from_piece.my_type);
        println!(" to : {:?}", to_piece.my_type);
    }
}


fn draw_path(
    query: Query<&Piece, With<pieces::From>>,
    mut painter: ShapePainter,
){
    let Ok(selected_piece) = query.get_single() else { return };

    let direction = if selected_piece.color == PieceColor::Black { -1 } else { 1 };

    match selected_piece.my_type{
            PieceType::Pawn => {
                for i in 0..3 {
                    let pos = Vec3::new(
                        (selected_piece.x) as f32 * SIZE  ,
                        (selected_piece.y + i * direction) as f32 * SIZE  ,
                        1.,
                    );
                    painter.set_translation(pos);

                    painter.color = Color::LIME_GREEN;
                    painter.circle(SIZE * 0.4);
                }
            }
            _ =>{}
    }
}

fn move_to_path(
    query: Query<&mut Piece, With<pieces::From>>,
){

}