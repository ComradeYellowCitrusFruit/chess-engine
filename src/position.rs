/*
*   name: src/position.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

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
	
	fn toAlgebraic(&self) -> String;
	fn fromAlgebraic(String) -> Position;
	
	fn inBounds(&self) -> bool
	{
		if self.num < 1 || self.num > 8 || self.letter < 1 || self.letter > 8
		{
			return false;
		}

		true
	}
}

impl Board
{
    // Is position attacked
	fn positionUnderAttack(&self, pos: Position) -> bool
	{
		// TODO: Optimize this to use as few passes as possible.
		(
		self.positionUnderAttackBy(pos, PieceType::wPawn) ||
		self.positionUnderAttackBy(pos, PieceType::bPawn) ||
		self.positionUnderAttackBy(pos, PieceType::wRook) ||
		self.positionUnderAttackBy(pos, PieceType::bRook) ||
		self.positionUnderAttackBy(pos, PieceType::wBishop) ||
		self.positionUnderAttackBy(pos, PieceType::bBishop) ||
		self.positionUnderAttackBy(pos, PieceType::wKnight) ||
		self.positionUnderAttackBy(pos, PieceType::bKnight) ||
		self.positionUnderAttackBy(pos, PieceType::wQueen) ||
		self.positionUnderAttackBy(pos, PieceType::bQueen) ||
		self.positionUnderAttackBy(pos, PieceType::wKing) ||
		self.positionUnderAttackBy(pos, PieceType::bKing)
		)
	}

	// Is position attacked by a piece of type piece
	fn positionUnderAttackBy(&self, pos: Position, piece: PieceType) -> bool
	{
		match piece
		{
			PieceType::wPawn =>
			{
				if	self.getPiece(pos.num - 1, pos.letter - 1) == piece ||
					self.getPiece(pos.num - 1, pos.letter + 1) == piece
				{
					return true;
				}
			},
			PieceType::bPawn =>
			{
				if	self.getPiece(pos.num + 1, pos.letter + 1) == piece || 
					self.getPiece(pos.num + 1, pos.letter - 1) == piece
				{
					return true;
				}
			},
			PieceType::wRook =>
			{
				for i in (1..9).rev()
				{
					if	self.getPiece(pos.num + i, pos.letter) == PieceType::wRook ||
						self.getPiece(pos.num, pos.letter + i) == PieceType::wRook
					{
						return true;
					}
				}
			},
			PieceType::bRook =>
			{
				for i in (1..9).rev()
				{
					if	self.getPiece(pos.num + i, pos.letter) == PieceType::bRook ||
						self.getPiece(pos.num, pos.letter + i) == PieceType::bRook
					{
						return true;
					}
				}
			},
			PieceType::wBishop =>
			{
				for i in (1..9).rev()
				{
					if	self.getPiece(pos.num + i, pos.letter + i) == PieceType::wBishop ||
						self.getPiece(pos.num - i, pos.letter + i) == PieceType::wBishop ||
						self.getPiece(pos.num + i, pos.letter - i) == PieceType::wBishop || 
						self.getPiece(pos.num - i, pos.letter - i) == PieceType::wBishop
					{
						return true;
					}
				}
			},
			PieceType::bBishop =>
			{
				for i in (1..9).rev()
				{
					if	self.getPiece(pos.num + i, pos.letter + i) == PieceType::bBishop ||
						self.getPiece(pos.num - i, pos.letter + i) == PieceType::bBishop ||
						self.getPiece(pos.num + i, pos.letter - i) == PieceType::bBishop ||
						self.getPiece(pos.num - i, pos.letter - i) == PieceType::bBishop
					{
						return true;
					}
				}
			},
			PieceType::wQueen =>
			{
				for i in (1..9).rev()
				{
					if	self.getPiece(pos.num + i, pos.letter + i) == PieceType::wQueen ||
						self.getPiece(pos.num - i, pos.letter + i) == PieceType::wQueen ||
						self.getPiece(pos.num + i, pos.letter - i) == PieceType::wQueen || 
						self.getPiece(pos.num - i, pos.letter - i) == PieceType::wQueen ||
						self.getPiece(pos.num + i, pos.letter) == PieceType::wQueen ||
						self.getPiece(pos.num, pos.letter + i) == PieceType::wQueen
					{
						return true;
					}
				}
			},
			PieceType::bQueen =>
			{
				for i in (1..9).rev()
				{
					if	self.getPiece(pos.num + i, pos.letter + i) == PieceType::bQueen || 
						self.getPiece(pos.num - i, pos.letter + i) == PieceType::bQueen || 
						self.getPiece(pos.num + i, pos.letter - i) == PieceType::bQueen || 
						self.getPiece(pos.num - i, pos.letter - i) == PieceType::bQueen || 
						self.getPiece(pos.num + i, pos.letter) == PieceType::bQueen || 
						self.getPiece(pos.num, pos.letter + i) == PieceType::bQueen
					{
						return true;
					}
				}
			},
			PieceType::wKing =>
			{
				if	self.getPiece(pos.num + 1, pos.letter + 1) == PieceType::wKing || 
					self.getPiece(pos.num + 1, pos.letter) == PieceType::wKing || 
					self.getPiece(pos.num + 1, pos.letter - 1) == PieceType::wKing || 
					self.getPiece(pos.num, pos.letter + 1) == PieceType::wKing || 
					self.getPiece(pos.num - 1, pos.letter + 1) == PieceType::wKing || 
					self.getPiece(pos.num, pos.letter) == PieceType::wKing || 
					self.getPiece(pos.num - 1, pos.letter - 1) == PieceType::wKing
				{
					return true;
				}
			},
			PieceType::bKing =>
			{
				if	self.getPiece(pos.num + 1, pos.letter + 1) == PieceType::bKing ||
					self.getPiece(pos.num + 1, pos.letter) == PieceType::bKing ||
					self.getPiece(pos.num + 1, pos.letter - 1) == PieceType::bKing ||
					self.getPiece(pos.num, pos.letter + 1) == PieceType::bKing || 
					self.getPiece(pos.num - 1, pos.letter + 1) == PieceType::bKing || 
					self.getPiece(pos.num, pos.letter) == PieceType::bKing || 
					self.getPiece(pos.num - 1, pos.letter - 1) == PieceType::bKing
				{
					return true;
				}
			},
			PieceType::wKnight =>
			{
				if	self.getPiece(pos.num + 3, pos.letter + 2) == PieceType::wKnight ||
					self.getPiece(pos.num + 3, pos.letter - 2) == PieceType::wKnight ||
					self.getPiece(pos.num - 3, pos.letter + 2) == PieceType::wKnight ||
					self.getPiece(pos.num - 3, pos.letter - 2) == PieceType::wKnight ||
					self.getPiece(pos.num + 2, pos.letter + 3) == PieceType::wKnight ||
					self.getPiece(pos.num + 2, pos.letter - 3) == PieceType::wKnight ||
					self.getPiece(pos.num - 2, pos.letter + 3) == PieceType::wKnight ||
					self.getPiece(pos.num - 2, pos.letter - 3) == PieceType::wKnight
				{
					return true;
				}
			},
			PieceType::bKnight =>
			{
				if	self.getPiece(pos.num + 3, pos.letter + 2) == PieceType::bKnight ||
					self.getPiece(pos.num + 3, pos.letter - 2) == PieceType::bKnight ||
					self.getPiece(pos.num - 3, pos.letter + 2) == PieceType::bKnight ||
					self.getPiece(pos.num - 3, pos.letter - 2) == PieceType::bKnight ||
					self.getPiece(pos.num + 2, pos.letter + 3) == PieceType::bKnight ||
					self.getPiece(pos.num + 2, pos.letter - 3) == PieceType::bKnight ||
					self.getPiece(pos.num - 2, pos.letter + 3) == PieceType::bKnight ||
					self.getPiece(pos.num - 2, pos.letter - 3) == PieceType::bKnight
				{
					return true;
				}
			},
			PieceType::empty =>
			{
				return false;
			}
		}
		false
	}
}