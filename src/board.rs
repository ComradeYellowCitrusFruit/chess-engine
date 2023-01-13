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
	pieces: [mut [mut pieceType; 8]; 8];
}

impl board
{
	fn isBlackInCheckmate(&self) -> bool;
	fn isWhiteInCheckmate(&self) -> bool;

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
}