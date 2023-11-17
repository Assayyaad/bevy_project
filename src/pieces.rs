use super::*;

const ORDER_LAYER: f32 = 5.0;
const SCALER: f32 = 2.;

#[derive(Component)]
pub struct From;

#[derive(Component)]
pub struct To; 

#[derive(Resource, Default)]
pub struct SelectedPieces{
    pub from: Option<Entity>,
    pub to: Option<Entity>,
}

impl SelectedPieces{

    pub fn from(&mut self, entity: Entity, commands: &mut Commands, from_query: &Query<&From>){
        self.remove_from(commands, from_query);
        self.assign_from(entity, commands);
    }

    pub fn remove_from(&mut self, commands: &mut Commands, from_query: &Query<&From>){
        if let Some(from) = self.from{
            if let Ok(_) = from_query.get_component::<From>(from){
                commands.entity(from).remove::<From>();
            }
        }
    }

    pub fn assign_from(&mut self, entity: Entity, commands: &mut Commands ){
        self.from = Some(entity);
        commands.entity(entity).insert(From);
    }


    pub fn to(&mut self, entity: Entity, commands: &mut Commands, to_query: &Query<&To>){
        self.remove_to(commands, to_query);
        self.assign_to(entity, commands);
    }

    pub fn remove_to(&mut self, commands: &mut Commands, to_query: &Query<&To>){
        if let Some(to) = self.to{
            if let Ok(_) = to_query.get_component::<From>(to){
                commands.entity(to).remove::<To>();
            }
        }
    }

    pub fn assign_to(&mut self, entity: Entity, commands: &mut Commands ){
        self.to = Some(entity);
        commands.entity(entity).insert(To);
    }
    
}


#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PieceColor {
    #[default]
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Component, Clone, Copy)]
pub struct Piece {
    pub my_type: PieceType,
    pub color: PieceColor,
    // NOTE: Position
    pub x: i8,
    pub y: i8,
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        // app.add_state::<Game>()
        app
            .insert_resource(SelectedPieces::default())
            .add_systems(Startup, spawn_pieces)
            .add_systems(PostStartup, load_sprites)
            .add_systems(Update, move_pieces);
    }
}

fn spawn_pieces(mut commands: Commands) {
    let chess_pieces: [(PieceType, Vec<(u8, u8)>); 6] = [
        (PieceType::King, vec![(4, 0)]),
        (PieceType::Queen, vec![(3, 0)]),
        (PieceType::Rook, vec![(0, 0), (7, 0)]),
        (PieceType::Bishop, vec![(1, 0), (6, 0)]),
        (PieceType::Knight, vec![(2, 0), (5, 0)]),
        (PieceType::Pawn, (0..MAX).map(|x| (x, 1)).collect()),
    ];

    for (my_type, pos_arr) in chess_pieces.into_iter() {
        for i in 0..pos_arr.len() {
            commands.spawn(Piece {
                my_type: my_type,
                color: PieceColor::White,
                x: pos_arr[i].0 as i8,
                y: pos_arr[i].1 as i8,
            });
            commands.spawn(Piece {
                my_type: my_type,
                color: PieceColor::Black,
                x: pos_arr[i].0 as i8,
                y: ( MAX - 1 - pos_arr[i].1 ) as i8,
            });
        }
    }
}

fn load_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Piece)>,
) {
    for (id, piece) in query.iter() {
        commands.entity(id).insert(SpriteBundle {
            texture: {
                let image_path = format!(
                    "ARABIAN CHESS/sprites/pieces/{:?}_{:?}.png",
                    piece.color, piece.my_type
                )
                .to_lowercase();
                asset_server.load(image_path)
            },
            transform: {
                let pos = Vec3::new(piece.x as f32 * SIZE, piece.y as f32 * SIZE, ORDER_LAYER);
                let scale = Vec3::new(SCALER, SCALER, 1.);

                Transform::from_translation(pos).with_scale(scale)
            },
            ..Default::default()
        });
    }
}

// TODO: if move not allowed, return
// TODO: else if not empty and ally piece on it, return
// TODO: else if path is blocked, return

// FIX: not ECS code , it handles multiple functionalty : moving pieces, despawn objects, changing the turn 
fn move_pieces(
    mut commands: Commands,
    mut selection: ResMut<Selection>,
    mut query: Query<(Entity, &mut Piece, &mut Transform)>,
    mut manager: ResMut<TurnManager>,
) {
    if selection.to == Vec2::NEG_ONE {
        return;
    }

    let mut check1 = false;
    let mut check2 = false;

    for (id, mut piece, mut transform) in query.iter_mut() {
        let pos = Vec2::new(piece.x as f32, piece.y as f32);
        if pos == selection.from {
            piece.x = selection.to.x as i8;
            piece.y = selection.to.y as i8;

            transform.translation =
                Vec3::new(selection.to.x * SIZE, selection.to.y * SIZE, ORDER_LAYER);
            check1 = true;
        } else if pos == selection.to {
            commands.entity(id).despawn();
            check2 = true;
        }

        if check1 && check2 {
            break;
        }
    }

    if check1 || check2 {
        manager.next_turn();
    }

    selection.from = Vec2::NEG_ONE;
    selection.to = Vec2::NEG_ONE;
}
