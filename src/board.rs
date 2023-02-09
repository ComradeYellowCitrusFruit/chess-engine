/*
*   name: src/board.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

use std::{ option::* };

pub use crate::{ fen::*, moves::*, position::Position, piece::PieceType };

#[derive(Copy, Clone)]
pub struct Board
{
	pub wqCastleAvalible: bool,
	pub wkCastleAvalible: bool,
	pub bqCastleAvalible: bool,
	pub bkCastleAvalible: bool,
	pub bToMove: bool,
	pub enPassant: Option<Position>,
	pub pieces: [[PieceType; 8]; 8],
}

impl Board
{
	pub fn isBlackInCheckmate(&self) -> bool
	{
		for i in 1..9
		{
			for j in 1..9
			{					
				if self.getPiece(i, j).is_some() && *self.getPiece(i, j).unwrap() == PieceType::BKing && self.positionUnderAttackByWhite(Position::mkPos(i, j))
				{
					let v = self.generateMoves();
					let c: usize;
    				for k in v
    				{
    					c += if k.isBlackInCheck() { 1 } else { 0 };
					}

					if c == v.len()
					{
						return true;
					}
					else
					{
						return false;
					}
    			}
			}
		}
		false
	}

	pub fn isWhiteInCheckmate(&self) -> bool
	{
		for i in 1..9
		{
			for j in 1..9
			{					
				if self.getPiece(i, j).is_some() && *self.getPiece(i, j).unwrap() == PieceType::WKing && self.positionUnderAttackByBlack(Position::mkPos(i, j))
				{
					let v = self.generateMoves();
					let c: usize;
    				for k in v
    				{
    					c += if k.isWhiteInCheck() { 1 } else { 0 };
					}

					if c == v.len()
					{
						return true;
					}
					else
					{
						return false;
					}
    			}
			}
		}
		false
	}

	pub fn isBlackInCheck(&self) -> bool
	{
		for i in 1..9
		{
			for j in 1..9
			{					
				if self.getPiece(i, j).is_some() && *self.getPiece(i, j).unwrap() == PieceType::BKing
				{
    				return if self.positionUnderAttackByWhite(Position::mkPos(i, j)) 
					{ 
						true 
					}
					else
					{
						false
					};
    			}
			}
		}

		return false;
	}
	pub fn isWhiteInCheck(&self) -> bool
	{
		for i in 1..9
		{
			for j in 1..9
			{					
				if self.getPiece(i, j).is_some() && *self.getPiece(i, j).unwrap() == PieceType::WKing
				{
    				return if self.positionUnderAttackByBlack(Position::mkPos(i, j)) 
					{ 
						true 
					}
					else
					{
						false
					};
    			}
			}
		}

		return false;
	}
}