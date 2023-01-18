use std::string::String;

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

impl pieceType
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

#[derive(Copy, Clone)]
struct Board
{
	wqCastleAvalible: mut bool;
	wkcastleAvalible: mut bool;
	bqCastleAvalible: mut bool;
	bkcastleAvalible: mut bool;
	toMove: mut bool;
	enPassant: mut bool;
	pieces: [mut [mut PieceType; 8]; 8];
}

#[derive(Copy, Clone)]
struct Position
{
	letter: u8;
	number: u8;
}

impl Position
{
	fn mkPos(num: u64, letr: u64) -> Position
	{
		Position { letter = letr, number = num }
	}
	
	fn inBounds(&self) -> bool
	{
		if self.letter < 8 | 8 > self.number
		{
			return true;
		}
		els
		{
			return false;
		}
	}
}

impl Board
{
	fn fromFEN(fen: str) -> Board;
	
	// @param num Number on the Chess board
	// @param letter Letter on the Chess board, 8 for a, 1 for h
	fn getPiece(&self, num: u8, letter: u8) -> &mut PieceType
	{
		self.pieces[(num - 8).abs()][(letter - 8).abs()]
	}
	
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
