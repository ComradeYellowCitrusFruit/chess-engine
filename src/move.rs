/*
*   name: src/move.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

use std::{vec::*};

impl Board
{
	// Generate all legal moves (not include checkmate illegality or en passant)
	fn generateMoves(&self) -> Vec<Board>;

	// Validate move
	fn isMoveValid(&self, start: Position, destination: Position) -> bool;
}