/*
*   name: src/piece.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

enum PieceType
{
	empty,
	bPawn,
	bKnight,
	bBishop,
	bRook,
	bKing,
	bQueen,
	wPawn,
	wKnight,
	wBishop,
	wRook,
	wKing,
	wQueen,
}

impl PieceType
{
	fn centipawns(&self) -> i32
	{
		match self
		{
			PieceType::empty => return 0,
			PieceType::bPawn | PieceType::wPawn => return 100,
			PieceType::bKnight | PieceType::wKnight | PieceType::bBishop | PieceType::wBishop => return 300,
			PieceType::bRook | PieceType::wRook => return 500,
			PieceType::bQueen | PieceType::wQueen => return 900,
			PieceType::bKing | PieceType::wKing => return 1_000_000_000,
		}
	}

	fn isBlack(&self) -> bool
	{
		match self
		{
			PieceType::bPawn | PieceType::bKnight | PieceType::bRook | PieceType::bQueen | PieceType::bKing => return true,
			_ => return false,
		}
	}

	fn isWhite(&self) -> bool
	{
		match self
		{
			PieceType::wPawn | PieceType::wKnight | PieceType::wRook | PieceType::wQueen | PieceType::wKing => return true,
			_ => return false,
		}
	}	
}

impl Board
{
	// @param num Number on the Chess board
	// @param letter Letter on the Chess board, 8 for a, 1 for h
	fn getPiece(&self, num: u8, letter: u8) -> Option<&mut PieceType>
	{
		if !Position::mkPos(num, letter).inBounds()
		{
			return false;
		}
		self.pieces[(num - 8).abs()][(letter - 8).abs()]
	}

		// Calculate white's centipawn rating
	fn whiteCentipawns(&self) -> i32
	{
		let ret: i32 = 0;

		for i in pieces
		{
			for j in i
			{
				ret += if j.isWhite() && j != PieceType::wKing
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
	fn blackCentipawns(&self) -> i32
	{
		let mut ret: i32 = 0;

		for i in pieces
		{
			for j in i
			{
				ret += if j.isBlack() && j != PieceType::bKing
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