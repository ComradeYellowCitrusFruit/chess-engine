/*
*   name: src/move.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

use std::{vec::*};

pub use crate::board::*;
pub use crate::fen::*;
pub use crate::piece::*;
pub use crate::position::*;
pub use crate::search::*;

impl Board
{
	// Generate all legal moves (not include checkmate illegality or en passant)
	pub fn generateMoves(&self) -> Vec<Board>;

	// Validate move
	pub fn isMoveValid(&self, start: Position, destination: Position) -> bool;
}