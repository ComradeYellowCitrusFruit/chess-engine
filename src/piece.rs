/*
*   name: src/piece.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

pub use crate::{ board::Board, moves::*, position::Position, fen::*};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceType
{
	Empty,
	BPawn,
	BKnight,
	BBishop,
	BRook,
	BKing,
	BQueen,
	WPawn,
	WKnight,
	WBishop,
	WRook,
	WKing,
	WQueen,
}

impl PieceType
{
	pub fn centipawns(&self) -> i32
	{
		match self
		{
			PieceType::Empty => return 0,
			PieceType::BPawn | PieceType::WPawn => return 100,
			PieceType::BKnight | PieceType::WKnight | PieceType::BBishop | PieceType::WBishop => return 300,
			PieceType::BRook | PieceType::WRook => return 500,
			PieceType::BQueen | PieceType::WQueen => return 900,
			PieceType::BKing | PieceType::WKing => return 1_000_000_000,
		}
	}

	pub fn isBlack(&self) -> bool
	{
		match self
		{
			PieceType::BPawn | PieceType::BKnight | PieceType::BRook | PieceType::BQueen | PieceType::BKing => return true,
			_ => return false,
		}
	}

	pub fn isWhite(&self) -> bool
	{
		match self
		{
			PieceType::WPawn | PieceType::WKnight | PieceType::WRook | PieceType::WQueen | PieceType::WKing => return true,
			_ => return false,
		}
	}	
}

impl Board
{
	// @param num Number on the Chess board
	// @param letter Letter on the Chess board, 8 for a, 1 for h
	pub fn getPiece(&self, num: u8, letter: u8) -> Option<&mut PieceType>
	{
		if !Position::mkPos(num, letter).inBounds()
		{
			return false;
		}
		self.pieces[(num - 8).abs()][(letter - 8).abs()]
	}

		// Calculate white's centipawn rating
	pub fn whiteCentipawns(&self) -> i32
	{
		let ret: i32 = 0;

		for i in self.pieces
		{
			for j in i
			{
				ret += if j.isWhite() && j != PieceType::WKing
				{
					j.centipawns()
				}
				else
				{
					0
				};
			}
		}
		ret
	}

	// Calculate black's centipawn rating
	pub fn blackCentipawns(&self) -> i32
	{
		let mut ret: i32 = 0;

		for i in self.pieces
		{
			for j in i
			{
				ret += if j.isBlack() && j != PieceType::BKing
				{
					j.centipawns()
				}
				else
				{
					0
				};
			}
		}
		ret
	}
}