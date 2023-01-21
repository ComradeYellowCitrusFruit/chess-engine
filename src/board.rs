/*
*   name: src/board.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

use std::{core::*, string::*, char::*, option::*};

struct Board
{
	wqCastleAvalible: mut bool,
	wkcastleAvalible: mut bool,
	bqCastleAvalible: mut bool,
	bkcastleAvalible: mut bool,
	bToMove: mut bool,
	enPassant: mut Option<Position>,
	pieces: [mut [mut PieceType; 8]; 8],
}

impl Board
{
	fn isBlackInCheckmate(&self) -> bool;
	fn isWhiteInCheckmate(&self) -> bool;
	fn isBlackInCheck(&self) -> bool;
	fn isWhiteInCheck(&self) -> bool;
}