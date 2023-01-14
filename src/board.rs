use std::string::String;

enum pieceType
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

impl pieceType
{
	fn centipawns(&self) -> i32
	{
		match self
		{
			pieceType::empty => return 0,
			pieceType::bPawn | pieceType::wPawn => return 100,
			pieceType::bKnight | pieceType::wKnight | pieceType::bBishop | pieceType::wBishop => return 300,
			pieceType::bRook | pieceType::wRook => return 500,
			pieceType::bQueen | pieceType::wQueen => return 900,
			pieceType::bKing | pieceType::wKing => return 1_000_000_000,
		}
	}

	fn isBlack(&self) -> bool
	{
		match self
		{
			pieceType::bPawn | pieceType::bKnight | pieceType::bRook | pieceType::bQueen | pieceType::bKing => return true,
			_ => return false,
		}
	}

	fn isWhite(&self) -> bool
	{
		match self
		{
			pieceType::wPawn | pieceType::wKnight | pieceType::wRook | pieceType::wQueen | pieceType::wKing => return true,
			_ => return false,
		}
	}	
}

struct board
{
	bCastled: mut bool;
	wCastled: mut bool;
	toMove: bool;
	enPassant: bool;
	pieces: [mut [mut pieceType; 8]; 8];
}

struct Position
{
	letter: u8;
	number: u8;
}

impl Position
{
	fn inBounds(&self) -> bool
	{
		if self.letter < 8 | 8 > self.number
		{
			return true;
		}
		else
		{
			return false;
		}
	}
}

impl board
{
	fn isBlackInCheckmate(&self) -> bool;
	fn isWhiteInCheckmate(&self) -> bool;
	fn isBlackInCheck(&self) -> bool;
	fn isWhiteInCheck(&self) -> bool;

	// Calculate white's centipawn rating
	fn whiteCentipawns(&self) -> i32
	{
		let ret: i32 = 0;

		for i in pieces
		{
			for j in i
			{
				ret += if j.isBlack() || j == pieceType::wKing
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
		let ret: i32 = 0;

		for i in pieces
		{
			for j in i
			{
				ret += if j.isWhite() || j == pieceType::bKing
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

	// Validate move
	fn isMoveValid(&self, start: Position, destination: Position) -> bool;

	// Is position attacked
	fn positionUnderAttack(&self, pos: Position) -> bool;

	// Is position attacked by a piece of type piece
	fn positionUnderAttackBy(&self, pos: Position, piece: pieceType) -> bool;

	fn toFEN(&self) -> String;
}