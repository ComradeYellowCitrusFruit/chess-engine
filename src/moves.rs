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
		// TODO: Redo all of this
		let vec: Vec<Board> = Vec::new();
		for i in 1..9
		{
			'next_piece: for j in 1..9
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
						if p.isBlack() != b.bToMove
						{
							continue 'next_piece;
						}
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
						if p.isBlack() != b.bToMove
						{
							continue 'next_piece;
						}

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
						if p.isBlack() != b.bToMove
						{
							continue 'next_piece;
						}

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
					PieceType::BKnight | PieceType::WKnight =>
					{
						if b.getPiece(i + 3, j + 2).is_some()
						{
							let tmpb = b;
							*tmpb.getPiece(i + 3, j + 2).unwrap() = *p;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							vec.push(tmpb);
						}
						if b.getPiece(i + 3, j - 2).is_some()
						{
							let tmpb = b;
							*tmpb.getPiece(i + 3, j - 2).unwrap() = *p;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							vec.push(tmpb);
						}
						if b.getPiece(i - 3, j + 2).is_some()
						{
							let tmpb = b;
							*tmpb.getPiece(i - 3, j + 2).unwrap() = *p;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							vec.push(tmpb);
						}
						if b.getPiece(i - 3, j - 2).is_some()
						{
							let tmpb = b;
							*tmpb.getPiece(i - 3, j - 2).unwrap() = *p;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							vec.push(tmpb);
						}
						if b.getPiece(i + 2, j + 3).is_some()
						{
							let tmpb = b;
							*tmpb.getPiece(i + 2, j + 3).unwrap() = *p;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							vec.push(tmpb);
						}
						if b.getPiece(i + 2, j - 3).is_some()
						{
							let tmpb = b;
							*tmpb.getPiece(i + 2, j - 3).unwrap() = *p;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							vec.push(tmpb);
						}
						if b.getPiece(i - 2, j + 3).is_some()
						{
							let tmpb = b;
							*tmpb.getPiece(i - 2, j + 3).unwrap() = *p;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							vec.push(tmpb);
						}
						if b.getPiece(i - 2, j - 3).is_some()
						{
							let tmpb = b;
							*tmpb.getPiece(i - 2, j - 3).unwrap() = *p;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							vec.push(tmpb);
						}
					},
					PieceType::BPawn =>
					{
						if !b.bToMove
						{
							continue 'next_piece;
						}
						if i == 7 && *b.getPiece(i - 2, j).unwrap() == PieceType::Empty
						{
							let tmpb = b;
							*tmpb.getPiece(i - 2, j).unwrap() = PieceType::BPawn;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							vec.push(tmpb);
						}
						if b.getPiece(i - 1, j).is_some() && b.getPiece(i - 1, j) == PieceType::Empty
						{
							let tmpb = b;
							*tmpb.getPiece(i - 1, j).unwrap() = PieceType::BPawn;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							if i == 2
							{
								*tmpb.getPiece(i - 1, j).unwrap() = PieceType::BBishop;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j).unwrap() = PieceType::BRook;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j).unwrap() = PieceType::BKnight;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j).unwrap() = PieceType::BQueen;
								vec.push(tmpb);
							}
							else
							{
								vec.push(tmpb);
							}
						}
						if b.getPiece(i - 1, j + 1).is_some() && b.getPiece(i - 1, j + 1) != PieceType::Empty
						{
							let tmpb = b;
							*tmpb.getPiece(i - 1, j + 1).unwrap() = PieceType::BPawn;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							if i == 2
							{
								*tmpb.getPiece(i - 1, j + 1).unwrap() = PieceType::BBishop;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j + 1).unwrap() = PieceType::BRook;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j + 1).unwrap() = PieceType::BKnight;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j + 1).unwrap() = PieceType::BQueen;
								vec.push(tmpb);
							}
							else
							{
								vec.push(tmpb);
							}
						}
						if b.getPiece(i - 1, j - 1).is_some() && b.getPiece(i - 1, j - 1) != PieceType::Empty
						{
							let tmpb = b;
							*tmpb.getPiece(i - 1, j - 1).unwrap() = PieceType::BPawn;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							tmpb.bToMove = !tmpb.bToMove;
							if i == 2
							{
								*tmpb.getPiece(i - 1, j - 1).unwrap() = PieceType::BBishop;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j - 1).unwrap() = PieceType::BRook;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j - 1).unwrap() = PieceType::BKnight;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j - 1).unwrap() = PieceType::BQueen;
								vec.push(tmpb);
							}
							else
							{
								vec.push(tmpb);
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