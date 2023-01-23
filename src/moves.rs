/*
*   name: src/move.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

use std::{vec::Vec};

pub use crate::board::*;
pub use crate::fen::*;
pub use crate::piece::*;
pub use crate::position::*;
pub use crate::search::*;

impl Board
{
	// Generate all legal moves (en passant and checkmate not included)
	pub fn generateMoves(&self) -> Vec<Board>
	{
		// TODO: Add en passant and castling
		let vec: Vec<Board> = Vec::new();
		for i in 1..9
		{
			for j in 1..9
			{
				let b = *self;
				let p = if b.getPiece(i, j).is_some()
				{
					b.getPiece(i, j).unwrap()
				}
				else
				{
					continue
				};

				// TODO: Add code to handle knights, kings, and pawns
				match *p
				{
					PieceType::BBishop | PieceType::WBishop =>
					{
						for k in 1..9
						{
							if b.getPiece(i + k, j + k).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i + k, j + k).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i - k, j - k).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i - k, j - k).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
						}
					},
					PieceType::BRook | PieceType::WRook =>
					{
						for k in 1..9
						{
							if b.getPiece(i + k, j).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i + k, j).unwrap() = *p;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i - k, j).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i - k, j).unwrap() = *p;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i, j + k).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i, j + k).unwrap() = *p;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i, j - k).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i, j - k).unwrap() = *p;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
						}
					},
					PieceType::BQueen | PieceType::WQueen =>
					{
						for k in 1..9
						{
							if b.getPiece(i + k, j + k).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i + k, j + k).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i - k, j - k).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i - k, j - k).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i + k, j).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i + k, j).unwrap() = *p;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i - k, j).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i - k, j).unwrap() = *p;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i, j + k).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i, j + k).unwrap() = *p;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i, j - k).is_some()
							{
								let tmpb = b;
								let p = *tmpb.getPiece(i + k, j + k).unwrap();
								*tmpb.getPiece(i, j - k).unwrap() = *p;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
								if p != PieceType::Empty
								{
									break;
								}
							}
						}
					},
				}
			}
		}
	}

	// Validate move
	pub fn isMoveValid(&self, start: Position, destination: Position) -> bool;
}