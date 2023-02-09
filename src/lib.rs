/*
*   name: src/lib.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/
extern crate cxx;

use std::{string::*, option::*, vec::*};
use cxx::*;

mod board;
mod fen;
mod moves;
mod piece;
mod position;
mod search;

pub use crate::{ board::Board, fen::*, moves::*, position::Position, search::alphabeta, piece::PieceType };

/*  Generate next move
*   @param FEN The FEN string to use to generate the next move from
*   @param depth How many moves to think ahead, less means a dumber AI, faster compuation, and less memory used
*/
pub fn generate_move(FEN: &str, depth: i16) -> Option<std::string::String>
{
	let b = Board::fromFEN(FEN);
	let nodes = b.generateMoves();
	let node_scores: Vec<i64> = Vec::with_capacity(nodes.len());
	let br: Board;
	let sr: i64 = 0;

	for i in nodes.iter()
	{
		node_scores.push(alphabeta(*i, depth - 1, i64::MIN, i64::MAX, b.bToMove));
	}

	for i in 0..nodes.len()
	{
		if node_scores[i] > sr
		{
			sr = node_scores[i];
			br = nodes[i];
		}
	}

	Some(br.toFEN())
}

/*  Get checkmate status, as bitflags
*   bit 0 - Set if white is in checkmate
*   bit 1 - Set if black is in checkmate
*   bit 2 - Set if position is illegal
*/
pub fn get_checkmate_status(fen: &str) -> u8
{
	let b = Board::fromFEN(fen);
	0 | 
	if b.isWhiteInCheckmate() { 1 << 0 } else { 0 } | 
	if b.isBlackInCheckmate() { 1 << 1} else { 0 } // TODO: legality
}

/*  Is a move legal?
*   @param FEN The FEN string to use to check legality
*   @param AN The algebraic notation form of the move
*/
pub fn is_legal(fen: &str, an: &str) -> bool
{
	let b = Board::fromFEN(fen);
	// TODO: Actual stuff.
	false // Just to get rust-analyzer to stfu
}

#[cxx::bridge]
mod ffi
{
	#[namespace = "chess"]
	extern "Rust"
	{
		/*  Generate next move
		*   @param FEN The FEN string to use to generate the next move from
		*   @param depth How many moves to think ahead, less means a dumber AI, faster compuation, and less memory used
		*/
		pub unsafe fn generateMove(FEN: *const u8, depth: i16) -> *const u8;

		/*  Get checkmate status, as a set of bitflags
		*   bit 0 - Set if white is in checkmate
		*   bit 1 - Set if black is in checkmate
		*   bit 2 - Set if position is illegal
		*/
		pub unsafe fn getCheckmateStatus(FEN: &CxxString) -> u8;

		/*  Is a move legal?
		*   @param FEN The FEN string to use to check legality
		*   @param AN The algebraic notation form of the move
		*/
		pub unsafe fn isLegal(FEN: &CxxString, AN: &CxxString) -> bool;
	}
}
