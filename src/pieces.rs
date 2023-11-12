use bevy::{prelude::*, transform};
use bevy_vector_shapes::prelude::*;

use crate::{board::MAX, board::SIZE, input::Selection};

const ORDER_LAYER: f32 = 5.0;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceColor {
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
    pub x: u8,
    pub y: u8,
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        // app.add_state::<Game>()
        app.add_systems(Startup, spawn_pieces)
            .add_systems(PostStartup, add_sprite)
            // .add_systems(Update, draw)
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
                x: pos_arr[i].0,
                y: pos_arr[i].1,
            });
            commands.spawn(Piece {
                my_type: my_type,
                color: PieceColor::Black,
                x: pos_arr[i].0,
                y: MAX - 1 - pos_arr[i].1,
            });
        }
    }
}

const SCALER: f32 = 2.;

fn add_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Piece)>,
) {
    for (id, piece) in query.iter() {
        commands.entity(id).insert(SpriteBundle {
            texture: {
                let image_path =
                    format!("{:?}_{:?}.png", piece.color, piece.my_type).to_lowercase();
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

// Todo : removing this system we dont need this it anymore , we already replaced by (assign_position, move transform)
fn draw(mut painter: ShapePainter, query: Query<&Piece>) {
    for piece in query.iter() {
        if piece.color == PieceColor::White {
            painter.color = Color::CYAN;
        } else {
            painter.color = Color::PINK;
        }

        let pos = Vec3::new(piece.x as f32 * SIZE, piece.y as f32 * SIZE, ORDER_LAYER);
        painter.translate(pos);
        painter.circle(SIZE * 0.25);
        painter.translate(-pos);
    }
}

// TODO: if move not allowed, return
// TODO: else if not empty and ally piece on it, return
// TODO: else if path is blocked, return

fn move_pieces(
    mut commands: Commands,
    mut selection: ResMut<Selection>,
    mut piece_query: Query<(Entity, &mut Piece, &mut Transform)>,
) {
    if selection.new == Vec2::NEG_ONE {
        return;
    }
    for (piece_id, mut piece, mut transform) in piece_query.iter_mut() {
        let pos = Vec2::new(piece.x as f32, piece.y as f32);
        if pos == selection.old {
            piece.x = selection.new.x as u8;
            piece.y = selection.new.y as u8;

            transform.translation =
                Vec3::new(selection.new.x * SIZE, selection.new.y * SIZE, ORDER_LAYER);
        } else if pos == selection.new {
            commands.entity(piece_id).despawn();
        }
    }

    selection.old = Vec2::NEG_ONE;
    selection.new = Vec2::NEG_ONE;
}
