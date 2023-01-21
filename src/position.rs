/*
*   name: src/position.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

use std::{string::*};

pub use crate::board::*;
pub use crate::fen::*;
pub use crate::moves::*;
pub use crate::piece::*;
pub use crate::search::*;

#[derive(Copy, Clone)]
pub struct Position
{
	pub letter: u8,
	pub number: u8,
}

impl Position
{
	pub fn mkPos(num: u8, letr: u8) -> Position
	{
		Position { letter: letr, number: num }
	}
	
	pub fn toAlgebraic(&self) -> String;
	pub fn fromAlgebraic(s: String) -> Position;
	
	pub fn inBounds(&self) -> bool
	{
		if self.number < 1 || self.number > 8 || self.letter < 1 || self.letter > 8
		{
			return false;
		}

		true
	}
}

impl Board
{
	// Is position attacked
	pub fn positionUnderAttack(&self, pos: Position) -> bool
	{
		// Check for knights first, then pawns, then kings, and then perform a full pass

		// Knight, pawn and king check, this is probably the fastest and most concise way to do it
		if  self.positionUnderAttackBy(pos, PieceType::WKnight) ||
			self.positionUnderAttackBy(pos, PieceType::BKnight) ||
			self.positionUnderAttackBy(pos, PieceType::WPawn) ||
			self.positionUnderAttackBy(pos, PieceType::BPawn) ||
			self.positionUnderAttackBy(pos, PieceType::WKing) ||
			self.positionUnderAttackBy(pos, PieceType::BKing)
		{
			return true;
		}

		// Perform the pass for rooks, bishops and queens
		for i in 1..9
		{
			if	self.getPiece(pos.number + i, pos.letter) == PieceType::WRook ||
				self.getPiece(pos.number - i, pos.letter) == PieceType::WRook ||
				self.getPiece(pos.number, pos.letter + i) == PieceType::WRook ||
				self.getPiece(pos.number, pos.letter - i) == PieceType::WRook ||
				self.getPiece(pos.number + i, pos.letter) == PieceType::BRook ||
				self.getPiece(pos.number - i, pos.letter) == PieceType::BRook ||
				self.getPiece(pos.number, pos.letter + i) == PieceType::BRook ||
				self.getPiece(pos.number, pos.letter - i) == PieceType::BRook ||
				self.getPiece(pos.number + i, pos.letter + i) == PieceType::WBishop ||
				self.getPiece(pos.number - i, pos.letter - i) == PieceType::WBishop ||
				self.getPiece(pos.number - i, pos.letter + i) == PieceType::WBishop ||
				self.getPiece(pos.number + i, pos.letter - i) == PieceType::WBishop ||
				self.getPiece(pos.number + i, pos.letter + i) == PieceType::BBishop ||
				self.getPiece(pos.number - i, pos.letter - i) == PieceType::BBishop ||
				self.getPiece(pos.number - i, pos.letter + i) == PieceType::BBishop ||
				self.getPiece(pos.number + i, pos.letter - i) == PieceType::BBishop ||
				self.getPiece(pos.numberer + i, pos.letter) == PieceType::WQueen ||
				self.getPiece(pos.number - i, pos.letter) == PieceType::WQueen ||
				self.getPiece(pos.number, pos.letter + i) == PieceType::WQueen ||
				self.getPiece(pos.number, pos.letter - i) == PieceType::WQueen ||
				self.getPiece(pos.number + i, pos.letter + i) == PieceType::WQueen ||
				self.getPiece(pos.number - i, pos.letter - i) == PieceType::WQueen ||
				self.getPiece(pos.number - i, pos.letter + i) == PieceType::WQueen ||
				self.getPiece(pos.number + i, pos.letter - i) == PieceType::WQueen ||
				self.getPiece(pos.number + i, pos.letter) == PieceType::BQueen ||
				self.getPiece(pos.number - i, pos.letter) == PieceType::BQueen ||
				self.getPiece(pos.number, pos.letter + i) == PieceType::BQueen ||
				self.getPiece(pos.number, pos.letter - i) == PieceType::BQueen ||
				self.getPiece(pos.number + i, pos.letter + i) == PieceType::BQueen ||
				self.getPiece(pos.number - i, pos.letter - i) == PieceType::BQueen ||
				self.getPiece(pos.number - i, pos.letter + i) == PieceType::BQueen ||
				self.getPiece(pos.number + i, pos.letter - i) == PieceType::BQueen
			{
				return true;
			}
		}
		
		false
	}

	// Is position attacked by a piece of type piece
	pub fn positionUnderAttackBy(&self, pos: Position, piece: PieceType) -> bool
	{
		match piece
		{
			PieceType::WPawn =>
			{
				if	self.getPiece(pos.number - 1, pos.letter - 1) == piece ||
					self.getPiece(pos.number - 1, pos.letter + 1) == piece
				{
					return true;
				}
			},
			PieceType::BPawn =>
			{
				if	self.getPiece(pos.number + 1, pos.letter + 1) == piece || 
					self.getPiece(pos.number + 1, pos.letter - 1) == piece
				{
					return true;
				}
			},
			PieceType::WRook =>
			{
				for i in 1..9
				{
					if	self.getPiece(pos.number + i, pos.letter) == PieceType::WRook ||
						self.getPiece(pos.number, pos.letter + i) == PieceType::WRook
					{
						return true;
					}
				}
			},
			PieceType::BRook =>
			{
				for i in 1..9
				{
					if	self.getPiece(pos.number + i, pos.letter) == PieceType::BRook ||
						self.getPiece(pos.number, pos.letter + i) == PieceType::BRook
					{
						return true;
					}
				}
			},
			PieceType::WBishop =>
			{
				for i in 1..9
				{
					if	self.getPiece(pos.number + i, pos.letter + i) == PieceType::WBishop ||
						self.getPiece(pos.number - i, pos.letter + i) == PieceType::WBishop ||
						self.getPiece(pos.number + i, pos.letter - i) == PieceType::WBishop || 
						self.getPiece(pos.number - i, pos.letter - i) == PieceType::WBishop
					{
						return true;
					}
				}
			},
			PieceType::BBishop =>
			{
				for i in 1..9
				{
					if	self.getPiece(pos.number + i, pos.letter + i) == PieceType::BBishop ||
						self.getPiece(pos.number - i, pos.letter + i) == PieceType::BBishop ||
						self.getPiece(pos.number + i, pos.letter - i) == PieceType::BBishop ||
						self.getPiece(pos.number - i, pos.letter - i) == PieceType::BBishop
					{
						return true;
					}
				}
			},
			PieceType::WQueen =>
			{
				for i in 1..9
				{
					if	self.getPiece(pos.number + i, pos.letter + i) == PieceType::WQueen ||
						self.getPiece(pos.number - i, pos.letter + i) == PieceType::WQueen ||
						self.getPiece(pos.number + i, pos.letter - i) == PieceType::WQueen || 
						self.getPiece(pos.number - i, pos.letter - i) == PieceType::WQueen ||
						self.getPiece(pos.number + i, pos.letter) == PieceType::WQueen ||
						self.getPiece(pos.number - i, pos.letter) == PieceType::WQueen ||
						self.getPiece(pos.number, pos.letter + i) == PieceType::WQueen ||
						self.getPiece(pos.number, pos.letter - i) == PieceType::WQueen
					{
						return true;
					}
				}
			},
			PieceType::BQueen =>
			{
				for i in 1..9
				{
					if	self.getPiece(pos.number + i, pos.letter + i) == PieceType::BQueen || 
						self.getPiece(pos.number - i, pos.letter + i) == PieceType::BQueen || 
						self.getPiece(pos.number + i, pos.letter - i) == PieceType::BQueen || 
						self.getPiece(pos.number - i, pos.letter - i) == PieceType::BQueen || 
						self.getPiece(pos.number + i, pos.letter) == PieceType::BQueen ||
						self.getPiece(pos.number - i, pos.letter) == PieceType::BQueen ||
						self.getPiece(pos.number, pos.letter + i) == PieceType::BQueen ||
						self.getPiece(pos.number, pos.letter - i) == PieceType::BQueen
					{
						return true;
					}
				}
			},
			PieceType::WKing =>
			{
				if	self.getPiece(pos.number + 1, pos.letter + 1) == PieceType::WKing || 
					self.getPiece(pos.number + 1, pos.letter) == PieceType::WKing || 
					self.getPiece(pos.number + 1, pos.letter - 1) == PieceType::WKing || 
					self.getPiece(pos.number, pos.letter + 1) == PieceType::WKing || 
					self.getPiece(pos.number - 1, pos.letter + 1) == PieceType::WKing || 
					self.getPiece(pos.number, pos.letter) == PieceType::WKing || 
					self.getPiece(pos.number - 1, pos.letter - 1) == PieceType::WKing
				{
					return true;
				}
			},
			PieceType::BKing =>
			{
				if	self.getPiece(pos.number + 1, pos.letter + 1) == PieceType::BKing ||
					self.getPiece(pos.number + 1, pos.letter) == PieceType::BKing ||
					self.getPiece(pos.number + 1, pos.letter - 1) == PieceType::BKing ||
					self.getPiece(pos.number, pos.letter + 1) == PieceType::BKing || 
					self.getPiece(pos.number - 1, pos.letter + 1) == PieceType::BKing || 
					self.getPiece(pos.number, pos.letter) == PieceType::BKing || 
					self.getPiece(pos.number - 1, pos.letter - 1) == PieceType::BKing
				{
					return true;
				}
			},
			PieceType::WKnight =>
			{
				if	self.getPiece(pos.number + 3, pos.letter + 2) == PieceType::WKnight ||
					self.getPiece(pos.number + 3, pos.letter - 2) == PieceType::WKnight ||
					self.getPiece(pos.number - 3, pos.letter + 2) == PieceType::WKnight ||
					self.getPiece(pos.number - 3, pos.letter - 2) == PieceType::WKnight ||
					self.getPiece(pos.number + 2, pos.letter + 3) == PieceType::WKnight ||
					self.getPiece(pos.number + 2, pos.letter - 3) == PieceType::WKnight ||
					self.getPiece(pos.number - 2, pos.letter + 3) == PieceType::WKnight ||
					self.getPiece(pos.number - 2, pos.letter - 3) == PieceType::WKnight
				{
					return true;
				}
			},
			PieceType::BKnight =>
			{
				if	self.getPiece(pos.number + 3, pos.letter + 2) == PieceType::BKnight ||
					self.getPiece(pos.number + 3, pos.letter - 2) == PieceType::BKnight ||
					self.getPiece(pos.number - 3, pos.letter + 2) == PieceType::BKnight ||
					self.getPiece(pos.number - 3, pos.letter - 2) == PieceType::BKnight ||
					self.getPiece(pos.number + 2, pos.letter + 3) == PieceType::BKnight ||
					self.getPiece(pos.number + 2, pos.letter - 3) == PieceType::BKnight ||
					self.getPiece(pos.number - 2, pos.letter + 3) == PieceType::BKnight ||
					self.getPiece(pos.number - 2, pos.letter - 3) == PieceType::BKnight
				{
					return true;
				}
			},
			PieceType::Empty =>
			{
				return false;
			}
		}
		false
	}
}