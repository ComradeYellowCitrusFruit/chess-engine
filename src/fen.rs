/*
*   name: src/fen.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

use std::{ string::* };

pub use crate::{ board::Board, moves::*, position::Position, piece::PieceType };

impl PieceType
{
	fn fenChar(&self) -> char
	{
		match self
		{
			wRook => return 'R',
			wKnight => return 'N',
			wBishop => return 'B',
			wQueen => return 'Q',
			wKing => return 'K',
			wPawn => return 'P',
			bRook => return 'r',
			bKnight => return 'n',
			bBishop => return 'b',
			bQueen => return 'q',
			bKing => return 'k',
			bPawn => return 'p',
		}
	}
}

impl Board
{
	pub fn defaultPos() -> Board
	{
		Board::fromFEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
	}
	
	pub fn fromFEN(fen: &str) -> Board
	{
		// 0 - positions
		// 1 - To Move
		// 2 - Castling metadata
		// 3 - En Passant
		// 4 - Halfmove clock, currently unused
		// 5 - Fullmove clock, currently unused
		let sectionOfFen = 0;
		let num = 8;
		let letter = 8;
		let ret = Board {
			wqCastleAvalible: false,
			wkCastleAvalible: false,
			bqCastleAvalible: false,
			bkCastleAvalible: false,
			bToMove: false,
			enPassant: None,
			pieces: [[PieceType::Empty; 8]; 8],
		};
		for c in fen.chars()
		{
			if num != 1 && letter != 0
			{
				if c == '/'
				{
					num += 1;
				}
				else if c.is_digit(10)
				{
					letter -= c.to_digit(10).unwrap();
				}
				else
				{
					*ret.getPiece(num as i8, letter as i8).unwrap() = match c
					{
						'r' => PieceType::BRook,
						'n' => PieceType::BKnight,
						'b' => PieceType::BBishop,
						'q' => PieceType::BQueen,
						'k' => PieceType::BKing,
						'p' => PieceType::BPawn,
						'R' => PieceType::WRook,
						'N' => PieceType::WKnight,
						'B' => PieceType::WBishop,
						'Q' => PieceType::WQueen,
						'K' => PieceType::WKing,
						'P' => PieceType::WPawn,
						_ => PieceType::Empty,
					};
					letter -= 1;
				}
			}
			else if c == ' '
   			{
   				sectionOfFen += 1;
   			}
   			else
   			{
   				match sectionOfFen
				{
   					1 => {
   						if c == 'b'
   						{
   							ret.bToMove = true;
   						}
					},
   					2 => {
   						if c == 'K'
   						{
   							ret.wkCastleAvalible = true;
   						}
   						if c == 'Q'
   						{
							ret.wqCastleAvalible = true;
   						}
   						if c == 'k'
   						{
   							ret.bkCastleAvalible = true;
   						}
   						if c == 'q'
						{
   							ret.bqCastleAvalible = true;
   						}
   					},
   					3 => {
   						// TODO: En passant
   					},
					_ => {},
   				}
   			}
		}
		ret
	}

	pub fn toFEN(&self) -> String
	{
		let mut s = String::new();
		for i in (1..9).rev()
		{
			let emptyCount = 0;
			for j in (1..9).rev()
			{
				if self.getPiece(i, j).is_some() && *self.getPiece(i, j).unwrap() == PieceType::Empty
				{
					emptyCount += 1;
				}
				else
				{
					if emptyCount != 0
					{
						s.push(char::from_digit(emptyCount, 10).unwrap());
					}
					s.push(self.getPiece(i, j).unwrap().fenChar());
				}
			}
			if emptyCount != 0
			{
				s.push(char::from_digit(emptyCount, 10).unwrap());
			}
			s.push('/');
		}
		s.push(' ');
		
		if self.bToMove
		{
			s.push('b');
		}
		else
		{
			s.push('w');
		}
		
		s.push(' ');
		if self.wqCastleAvalible || self.wkCastleAvalible || self.bqCastleAvalible || self.bkCastleAvalible
		{
			if self.wkCastleAvalible
			{
				s.push('K');
			}
			if self.wqCastleAvalible
			{
				s.push('Q');
			}
			if self.bkCastleAvalible
			{
				s.push('k');
			}
			if self.bqCastleAvalible
			{
				s.push('q');
			}
		}
		else
		{
			s.push('-');
		}
		s.push(' ');

		if self.enPassant.is_some()
		{
			s.push_str(self.enPassant.unwrap().toAlgebraic().as_str());
		}
		else
		{
			s.push('-');
		}
		s.push(' ');

		s
	}
}