/*
*   name: src/board.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

use std::{string::*, char::*, option::*};

pub use crate::{ fen::*, moves::*, position::Position, piece::PieceType };

#[derive(Copy, Clone)]
pub struct Board
{
	pub wqCastleAvalible: bool,
	pub wkcastleAvalible: bool,
	pub bqCastleAvalible: bool,
	pub bkcastleAvalible: bool,
	pub bToMove: bool,
	pub enPassant: Option<Position>,
	pub pieces: [[PieceType; 8]; 8],
}

impl Board
{
	pub fn isBlackInCheckmate(&self) -> bool;
	pub fn isWhiteInCheckmate(&self) -> bool;
	pub fn isBlackInCheck(&self) -> bool;
	pub fn isWhiteInCheck(&self) -> bool;
}