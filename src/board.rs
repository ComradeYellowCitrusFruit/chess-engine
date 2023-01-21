/*
*   name: src/board.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

use std::{core::*, string::*, char::*, option::*};

struct Board
{
	wqCastleAvalible: bool,
	wkcastleAvalible: bool,
	bqCastleAvalible: bool,
	bkcastleAvalible: bool,
	bToMove: bool,
	enPassant: Option<Position>,
	pieces: [[PieceType; 8]; 8],
}

impl Board
{
	fn isBlackInCheckmate(&self) -> bool;
	fn isWhiteInCheckmate(&self) -> bool;
	fn isBlackInCheck(&self) -> bool;
	fn isWhiteInCheck(&self) -> bool;
}