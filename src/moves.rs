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

				// TODO: Add code to handle pawns, and check
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
								let P = *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i + k, j + k).unwrap() = *p;
									*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}
								
								if P != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i - k, j - k).is_some()
							{
								let tmpb = b;
								let P = *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i - k, j - k).unwrap() = *p;
									*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}

								if *p != PieceType::Empty
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
								let P = *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i + k, j).unwrap() = *p;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}

								if P != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i - k, j).is_some()
							{
								let tmpb = b;
								let P= *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i - k, j).unwrap() = *p;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}

								if P != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i, j + k).is_some()
							{
								let tmpb = b;
								let P = *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i, j + k).unwrap() = *p;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}
								
								if P != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i, j - k).is_some()
							{
								let tmpb = b;
								let P = *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i, j - k).unwrap() = *p;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}

								if P != PieceType::Empty
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
								let P = *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i + k, j + k).unwrap() = *p;
									*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}
								
								if P != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i - k, j - k).is_some()
							{
								let tmpb = b;
								let P = *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i - k, j - k).unwrap() = *p;
									*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}

								if P != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i + k, j).is_some()
							{
								let tmpb = b;
								let P = *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i + k, j).unwrap() = *p;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}

								if P != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i - k, j).is_some()
							{
								let tmpb = b;
								let P= *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i - k, j).unwrap() = *p;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}

								if P != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i, j + k).is_some()
							{
								let tmpb = b;
								let P = *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i, j + k).unwrap() = *p;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}
								
								if P != PieceType::Empty
								{
									break;
								}
							}
							if b.getPiece(i, j - k).is_some()
							{
								let tmpb = b;
								let P = *tmpb.getPiece(i + k, j + k).unwrap();
								if P.isBlack() != p.isBlack() || P == PieceType::Empty
								{
									*tmpb.getPiece(i, j - k).unwrap() = *p;
									tmpb.bToMove = !tmpb.bToMove;
									vec.push(tmpb);
								}

								if P != PieceType::Empty
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
							let P = tmpb.getPiece(i + 3, j + 2).unwrap();
							if P.isBlack() != p.isBlack()
							{
								*P = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
							}
						}
						if b.getPiece(i + 3, j - 2).is_some()
						{
							let tmpb = b;
							let P = tmpb.getPiece(i + 3, j - 2).unwrap();
							if P.isBlack() != p.isBlack()
							{
								*P = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
							}
						}
						if b.getPiece(i - 3, j + 2).is_some()
						{
							let tmpb = b;
							let P = tmpb.getPiece(i - 3, j + 2).unwrap();
							if P.isBlack() != p.isBlack()
							{
								*P = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
							}
						}
						if b.getPiece(i - 3, j - 2).is_some()
						{
							let tmpb = b;
							let P = tmpb.getPiece(i - 3, j - 2).unwrap();
							if P.isBlack() != p.isBlack()
							{
								*P = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
							}
						}
						if b.getPiece(i + 2, j + 3).is_some()
						{
							let tmpb = b;
							let P = tmpb.getPiece(i + 2, j + 3).unwrap();
							if P.isBlack() != p.isBlack()
							{
								*P = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
							}
						}
						if b.getPiece(i + 2, j - 3).is_some()
						{
							let tmpb = b;
							let P = tmpb.getPiece(i + 2, j - 3).unwrap();
							if P.isBlack() != p.isBlack()
							{
								*P = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
							}
						}
						if b.getPiece(i - 2, j + 3).is_some()
						{
							let tmpb = b;
							let P = tmpb.getPiece(i - 2, j + 3).unwrap();
							if P.isBlack() != p.isBlack()
							{
								*P = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
							}
						}
						if b.getPiece(i - 2, j - 3).is_some()
						{
							let tmpb = b;
							let P = tmpb.getPiece(i - 2, j - 3).unwrap();
							if P.isBlack() != p.isBlack()
							{
								*P = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								tmpb.bToMove = !tmpb.bToMove;
								vec.push(tmpb);
							}
						}
					},
					PieceType::BKing | PieceType::WKing =>
					{
						if b.getPiece(i + 1, j + 1).is_some()
						{
							let tmpb = b;
							let P = *b.getPiece(i + 1, j + 1).unwrap();
							if 	!b.positionUnderAttack(Position::mkPos(i + 1, j + 1)) &&
								(P.isBlack() != p.isBlack() || P == PieceType::Empty)
							{
								*tmpb.getPiece(i + 1, j + 1).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								vec.push(tmpb);
							}
						}
						if b.getPiece(i - 1, j - 1).is_some()
						{
							let tmpb = b;
							let P = *b.getPiece(i - 1, j -1).unwrap();
							if 	!b.positionUnderAttack(Position::mkPos(i - 1, j - 1)) &&
								(P.isBlack() != p.isBlack() || P == PieceType::Empty)
							{
								*tmpb.getPiece(i - 1, j - 1).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
								vec.push(tmpb);
							}
						}
						if b.getPiece(i + 1, j - 1).is_some()
						{
							let tmpb = b;
							let P = *b.getPiece(i + 1, j - 1).unwrap();
							if 	!b.positionUnderAttack(Position::mkPos(i + 1, j - 1)) &&
								(P.isBlack() != p.isBlack() || P == PieceType::Empty)
							{
								*tmpb.getPiece(i + 1, j - 1).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							}
						}
						if b.getPiece(i - 1, j + 1).is_some()
						{
							let tmpb = b;
							let P = *b.getPiece(i - 1, j + 1).unwrap();
							if 	!b.positionUnderAttack(Position::mkPos(i - 1, j + 1)) &&
								(P.isBlack() != p.isBlack() || P == PieceType::Empty)
							{
								*tmpb.getPiece(i - 1, j + 1).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							}
						}
						if b.getPiece(i + 1, j).is_some()
						{
							let tmpb = b;
							let P = *b.getPiece(i + 1, j + 1).unwrap();
							if 	!b.positionUnderAttack(Position::mkPos(i + 1, j + 1)) &&
								(P.isBlack() != p.isBlack() || P == PieceType::Empty)
							{
								*tmpb.getPiece(i + 1, j + 1).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							}
						}
						if b.getPiece(i, j + 1).is_some()
						{
							let tmpb = b;
							let P = *b.getPiece(i + 1, j + 1).unwrap();
							if 	!b.positionUnderAttack(Position::mkPos(i + 1, j + 1)) &&
								(P.isBlack() != p.isBlack() || P == PieceType::Empty)
							{
								*tmpb.getPiece(i + 1, j + 1).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							}
						}
						if b.getPiece(i - 1, j).is_some()
						{
							let tmpb = b;
							let P = *b.getPiece(i + 1, j + 1).unwrap();
							if 	!b.positionUnderAttack(Position::mkPos(i + 1, j + 1)) &&
								(P.isBlack() != p.isBlack() || P == PieceType::Empty)
							{
								*tmpb.getPiece(i + 1, j + 1).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							}
						}
						if b.getPiece(i, j - 1).is_some()
						{
							let tmpb = b;
							let P = *b.getPiece(i + 1, j + 1).unwrap();
							if 	!b.positionUnderAttack(Position::mkPos(i + 1, j + 1)) &&
								(P.isBlack() != p.isBlack() || P == PieceType::Empty)
							{
								*tmpb.getPiece(i + 1, j + 1).unwrap() = *p;
								*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							}
						}
					}, 
					// TODO: En Passant
					PieceType::BPawn =>
					{
						if !b.bToMove
						{
							continue 'next_piece;
						}
						
						if i == 7 && *b.getPiece(i - 2, j).unwrap() == PieceType::Empty 
						{
							let tmpb = b;
      						*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
      						*tmpb.getPiece(i - 2, j).unwrap() = PieceType::BPawn;
      						tmpb.bToMove = false;
      						vec.push(tmpb);
    					}

						if *b.getPiece(i - 1, j).unwrap() == PieceType::Empty
						{
							let tmpb = b;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							*tmpb.getPiece(i - 1, j).unwrap() = PieceType::BPawn;
							tmpb.bToMove = false;
							if tmpb.getPiece(i - 2, j).is_none()
							{
								*tmpb.getPiece(i - 1, j).unwrap() = PieceType::BKnight;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j).unwrap() = PieceType::BRook;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j).unwrap() = PieceType::BBishop;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j).unwrap() = PieceType::BQueen;
								vec.push(tmpb);
							}
							else
							{
								vec.push(tmpb);
							}
						}

						if b.getPiece(i - 1, j - 1).is_some() && *b.getPiece(i - 1, j - 1).unwrap() != PieceType::Empty && b.getPiece(i - 1, j - 1).unwrap().isWhite()
						{
							let tmpb = b;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							*tmpb.getPiece(i - 1, j - 1).unwrap() = PieceType::BPawn;
							tmpb.bToMove = false;
							if tmpb.getPiece(i - 2, j).is_none()
							{
								*tmpb.getPiece(i - 1, j - 1).unwrap() = PieceType::BKnight;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j - 1).unwrap() = PieceType::BRook;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j - 1).unwrap() = PieceType::BBishop;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j - 1).unwrap() = PieceType::BQueen;
								vec.push(tmpb);
							}
							else
							{
								vec.push(tmpb);
							}
						}

						if b.getPiece(i - 1, j + 1).is_some() && *b.getPiece(i - 1, j + 1).unwrap() != PieceType::Empty && b.getPiece(i - 1, j + 1).unwrap().isWhite()
						{
							let tmpb = b;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							*tmpb.getPiece(i - 1, j + 1).unwrap() = PieceType::BPawn;
							tmpb.bToMove = false;
							if tmpb.getPiece(i - 2, j).is_none()
							{
								*tmpb.getPiece(i - 1, j + 1).unwrap() = PieceType::BKnight;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j + 1).unwrap() = PieceType::BRook;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j + 1).unwrap() = PieceType::BBishop;
								vec.push(tmpb);
								*tmpb.getPiece(i - 1, j + 1).unwrap() = PieceType::BQueen;
								vec.push(tmpb);
							}
							else
							{
								vec.push(tmpb);
							};
						}
					},
					PieceType::WPawn =>
					{
						if b.bToMove
						{
							continue 'next_piece;
						}
						
						if i == 2 && *b.getPiece(i + 2, j).unwrap() == PieceType::Empty 
						{
							let tmpb = b;
      						*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
      						*tmpb.getPiece(i + 2, j).unwrap() = PieceType::BPawn;
      						tmpb.bToMove = false;
      						vec.push(tmpb);
    					}

						if *b.getPiece(i + 1, j).unwrap() == PieceType::Empty
						{
							let tmpb = b;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							*tmpb.getPiece(i + 1, j).unwrap() = PieceType::BPawn;
							tmpb.bToMove = false;
							if tmpb.getPiece(i + 2, j).is_none()
							{
								*tmpb.getPiece(i + 1, j).unwrap() = PieceType::BKnight;
								vec.push(tmpb);
								*tmpb.getPiece(i + 1, j).unwrap() = PieceType::BRook;
								vec.push(tmpb);
								*tmpb.getPiece(i + 1, j).unwrap() = PieceType::BBishop;
								vec.push(tmpb);
								*tmpb.getPiece(i + 1, j).unwrap() = PieceType::BQueen;
								vec.push(tmpb);
							}
							else
							{
								vec.push(tmpb);
							}
						}

						if b.getPiece(i + 1, j - 1).is_some() && *b.getPiece(i - 1, j - 1).unwrap() != PieceType::Empty && b.getPiece(i - 1, j - 1).unwrap().isWhite()
						{
							let tmpb = b;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							*tmpb.getPiece(i + 1, j - 1).unwrap() = PieceType::BPawn;
							tmpb.bToMove = false;

							if tmpb.getPiece(i + 2, j).is_none()
							{
								*tmpb.getPiece(i + 1, j - 1).unwrap() = PieceType::BKnight;
								vec.push(tmpb);
								*tmpb.getPiece(i + 1, j - 1).unwrap() = PieceType::BRook;
								vec.push(tmpb);
								*tmpb.getPiece(i + 1, j - 1).unwrap() = PieceType::BBishop;
								vec.push(tmpb);
								*tmpb.getPiece(i + 1, j - 1).unwrap() = PieceType::BQueen;
								vec.push(tmpb);
							}
							else
							{
								vec.push(tmpb);
							}
						}

						if b.getPiece(i + 1, j + 1).is_some() && *b.getPiece(i - 1, j + 1).unwrap() != PieceType::Empty && b.getPiece(i - 1, j + 1).unwrap().isWhite()
						{
							let tmpb = b;
							*tmpb.getPiece(i, j).unwrap() = PieceType::Empty;
							*tmpb.getPiece(i + 1, j + 1).unwrap() = PieceType::BPawn;
							tmpb.bToMove = false;

							if tmpb.getPiece(i + 2, j).is_none()
							{
								*tmpb.getPiece(i + 1, j + 1).unwrap() = PieceType::BKnight;
								vec.push(tmpb);
								*tmpb.getPiece(i + 1, j + 1).unwrap() = PieceType::BRook;
								vec.push(tmpb);
								*tmpb.getPiece(i + 1, j + 1).unwrap() = PieceType::BBishop;
								vec.push(tmpb);
								*tmpb.getPiece(i + 1, j + 1).unwrap() = PieceType::BQueen;
								vec.push(tmpb);
							}
							else
							{
								vec.push(tmpb);
							}
						}
					},
					PieceType::Empty =>
					{
						continue 'next_piece;
					},
				}
			}
		}
		vec
	}

	// Validate move
	pub fn isMoveValid(&self, start: Position, destination: Position) -> bool;
}