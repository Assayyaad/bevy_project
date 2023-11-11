use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

use crate::{board::MAX, board::SIZE, input::Selection};

const ORDER_LAYER: f32 = 5.0;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
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
    // Position
    pub x: u8,
    pub y: u8,
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pieces)
            .add_systems(Update, draw)
            .add_systems(PostUpdate, move_piece);
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

fn move_piece(
    mut commands: Commands,
    mut selection: ResMut<Selection>,
    mut piece_query: Query<(Entity, &mut Piece)>,
) {
    if selection.new == Vec2::NEG_ONE {
        return;
    }

    // TODO: if not move allowed, return

    for (piece_id, mut piece) in piece_query.iter_mut() {
        let pos = Vec2::new(piece.x as f32, piece.y as f32);
        if pos == selection.old {
            piece.x = selection.new.x as u8;
            piece.y = selection.new.y as u8;
        } else if pos == selection.new {
            commands.entity(piece_id).despawn();
        }
    }

    selection.old = Vec2::NEG_ONE;
    selection.new = Vec2::NEG_ONE;
}
