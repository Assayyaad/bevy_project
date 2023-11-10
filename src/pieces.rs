use bevy::prelude::*;

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

#[derive(Clone, Copy)]
pub struct Piece {
    pub my_type: PieceType,
    pub color: PieceColor,
    // Position
    pub x: i8,
    pub y: i8,
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pieces);
    }
}

// fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
//     for (velocity, mut transform) in query.iter_mut() {
//         transform.translation += velocity.value * time.delta_seconds();
//     }
// }

fn spawn_pieces(mut commands: Commands) {
    let chess_pieces: [(PieceType, [(i8, i8)]); 6] = [
        (PieceType::King, [(4, 0)]),
        (PieceType::Queen, [(3, 0)]),
        (PieceType::Rook, [(0, 0), (7, 0)]),
        (PieceType::Bishop, [(1, 0), (6, 0)]),
        (PieceType::Knight, [(2, 0), (5, 0)]),
        (PieceType::Pawn, (0..8).map(|x| (x, 1)).collect()),
    ];

    for &(my_type, pos_arr) in chess_pieces.iter() {
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
                y: 8 - pos_arr[i].1,
            });
        }
    }
}

//
//
//

impl Piece {
    pub fn is_move_valid(&self, new_pos: (i8, i8), pieces: Vec<Piece>) -> bool {
        // If there's a piece of the same color in the same square, it can't move
        if color_of_square(new_pos, &pieces) == Some(self.color) {
            return false;
        }

        let x_diff = (self.x - new_pos.0).abs();
        let y_diff = (self.y - new_pos.1).abs();
        let same_x = self.x == new_pos.0;
        let same_y = self.y == new_pos.1;
        let path_empty = is_path_empty((self.x, self.y), new_pos, &pieces);

        match self.my_type {
            PieceType::King => {
                path_empty
                    && ((x_diff == 1 && same_y)
                        || (y_diff == 1 && same_x)
                        || (x_diff == 1 && y_diff == 1))
            }
            PieceType::Queen => {
                path_empty && (x_diff == y_diff || ((same_x && !same_y) || (same_y && !same_x)))
            }
            PieceType::Bishop => path_empty && x_diff == y_diff,
            PieceType::Knight => (x_diff == 2 && y_diff == 1) || (x_diff == 1 && y_diff == 2),
            PieceType::Rook => path_empty && ((same_x && !same_y) || (same_y && !same_x)),
            PieceType::Pawn => {
                let color = color_of_square(new_pos, &pieces);

                if self.color == PieceColor::White {
                    if same_x && path_empty {
                        if self.y == 1 {
                            if y_diff == 2 {
                                if color.is_none() {
                                    true
                                }
                            }
                        } else {
                            if y_diff == 1 {
                                if color.is_none() {
                                    true
                                }
                            }
                        }
                    } else if new_pos.0 - self.x == 1 && y_diff == 1 {
                        if color == Some(PieceColor::Black) {
                            true
                        }
                    }
                } else {
                    if same_x && path_empty {
                        if self.y == 6 {
                            if y_diff == -2 {
                                if color.is_none() {
                                    true
                                }
                            }
                        } else {
                            if y_diff == -1 {
                                if color.is_none() {
                                    true
                                }
                            }
                        }
                    } else if new_pos.0 - self.x == -1 && y_diff == 1 {
                        if color == Some(PieceColor::White) {
                            true
                        }
                    }
                }

                false
            }
        }
    }
}

fn is_path_empty(begin: (i8, i8), end: (i8, i8), pieces: &Vec<Piece>) -> bool {
    if begin.0 == end.0 {
        for piece in pieces {
            if piece.x == begin.0
                && ((piece.y > begin.1 && piece.y < end.1)
                    || (piece.y > end.1 && piece.y < begin.1))
            {
                return false;
            }
        }
    }

    if begin.1 == end.1 {
        for piece in pieces {
            if piece.y == begin.1
                && ((piece.x > begin.0 && piece.x < end.0)
                    || (piece.x > end.0 && piece.x < begin.0))
            {
                return false;
            }
        }
    }

    let x_diff = (begin.0 - end.0).abs();
    let y_diff = (begin.1 - end.1).abs();
    if x_diff == y_diff {
        for i in 1..x_diff {
            let pos = if begin.0 < end.0 && begin.1 < end.1 {
                // left bottom - right top
                (begin.0 + i, begin.1 + i)
            } else if begin.0 < end.0 && begin.1 > end.1 {
                // left top - right bottom
                (begin.0 + i, begin.1 - i)
            } else if begin.0 > end.0 && begin.1 < end.1 {
                // right bottom - left top
                (begin.0 - i, begin.1 + i)
            } else {
                // begin.0 > end.0 && begin.1 > end.1
                // right top - left bottom
                (begin.0 - i, begin.1 - i)
            };

            if color_of_square(pos, pieces).is_some() {
                return false;
            }
        }
    }

    true
}

fn color_of_square(pos: (i8, i8), pieces: &Vec<Piece>) -> Option<PieceColor> {
    for piece in pieces {
        if piece.x == pos.0 && piece.y == pos.1 {
            return Some(piece.color);
        }
    }
    None
}
