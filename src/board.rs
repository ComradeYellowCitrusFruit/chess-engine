use std::{string::*, char::*, option::*};

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

#[derive(Copy, Clone)]
struct Board
{
	wqCastleAvalible: mut bool;
	wkcastleAvalible: mut bool;
	bqCastleAvalible: mut bool;
	bkcastleAvalible: mut bool;
	bToMove: mut bool;
	enPassant: mut Option<Position>;
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
	fn startPos() -> Board
	{
		Board::fromFEN("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
	}
	
	fn fromFEN(fen: str) -> Board
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
		let ret = Board { wqCastleAvalible = false, wkCastleAvalible = false, bqCastleAvalible = false, bkCastleAvalible = false, bToMove = false, enPassant = false, pieces = [[PieceType::empty; 8]; 8] };
		for c in str.chars()
		{
			if num != 1 && letter != 0
			{
				if c == '/'
				{
					num += 1;
				}
				else if c.is_digit(10)
				{
					letter -= c.to_digit(10);
				}
				else
				{
					ret.getPiece(num, letter) = match c
					{
						'r' => PieceType::bRook,
						'n' => PieceType::bKnight,
						'b' => PieceType::bBishop,
						'q' => PieceType::bQueen,
						'k' => PieceType::bKing,
						'p' => PieceType::bPawn,
						'R' => PieceType::wRook,
						'N' => PieceType::wKnight,
						'B' => PieceType::wBishop,
						'Q' => PieceType::wQueen,
						'K' => PieceType::wKing,
						'P' => PieceType::wPawn,
						_ => PieceType::empty,
					};
					letter -= 1;
				}
			}
			else
			{
				if c == ' '
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
							if c != '-'
							{
								let s = String::new();
								s.push(c);
								s.push(c.next());
								ret.enPassant = Some(Position::fromAlgebraic(s));
							}
							else
							{
								ret.enPassant = None;
							}
						},
						_ => {},
					}
				}
			}
		}
		ret
	}
	
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
				ret += if j.isBlack() | j == PieceType::wKing
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
				ret += if j.isWhite() || j == PieceType::bKing
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

	fn toFEN(&self) -> String
	{
		let s: String::new();
		for i in (1..9).rev()
		{
			let emptyCount = 0;
			for j in (1..9).rev()
			{
				if self.getPiece(i, j) == PieceType::empty
				{
					emptyCount += 1;
				}
				else
				{
					if emptyCount != 0
					{
						s.push(char::from_digit(emptyCount, 10));
					}
					s.push(self.getPiece(i, j).fenChar());
				}
			}
			if emptyCount != 0
			{
				s.push(char::from_digit(emptyCount, 10));
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
