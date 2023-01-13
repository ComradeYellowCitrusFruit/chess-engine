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
	fn centipawns(&self) -> u32
	{
		match self
		{
			pieceType::empty => return 0;
			pieceType::bPawn | pieceType::wPawn => return 100;
			pieceType::bKnight | pieceType::wKnight | pieceType::bBishop | pieceType::wBishop => return 300;
			pieceType::bRook | pieceType::wRook => return 500;
			pieceType::bQueen | pieceType::wQueen => return 900;
			pieceType::bKing | pieceType::wKing => return 1_000_000_000;
		}
	}
}

struct board
{
	bCastled: mut bool;
	wCastled: mut bool;
	toMove: bool;
	pieces: [mut [mut piece; 8]; 8];
}
