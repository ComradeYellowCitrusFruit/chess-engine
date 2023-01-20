use std::{string::String, option::Option, ffi::*, vec::Vec};

/*  Generate next move
*   @param FEN The FEN string to use to generate the next move from
*   @param depth How many moves to think ahead, less means a dumber AI, faster compuation, and less memory used
*/
pub fn generateMove(FEN: &str, depth: i16) -> Option<String>
{
	let b = Board::fromFEN(FEN);
	let nodes = b.generateMoves();
	let nodeScores: Vec<i64> = Vec::with_capacity(nodes.len());
	let br: Board;
	let sr: i64 = 0;

	for i in nodes.iter()
	{
		nodeScores.push(alphabeta(i, depth - 1, i64::MIN, i64::MAX, b.bToMove));
	}

	for i in 0..nodes.len()
	{
		if(nodeScores[i] > sr)
		{
			sr = nodeScores[i];
			br = nodes[i];
		}
	}

	br.toFEN()
}

/*  Get checkmate status, as bitflags
*   bit 0 - Set if white is in checkmate
*   bit 1 - Set if black is in checkmate
*   bit 2 - Set if position is illegal
*/
pub fn getCheckmateStatus(FEN: &str) -> u8;

/*  Is a move legal?
*   @param FEN The FEN string to use to check legality
*   @param AN The algebraic notation form of the move
*/
pub fn isLegal(FEN: &str, AN: &str) -> bool;

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
		pub fn generateMove(FEN: &CxxString, depth: i16) -> CxxString
		{
			if FEN.as_str().is_ok()
			{
				let_cxx_string!(ret = generateMove(FEN.as_str().ok().unwrap(), depth));
				return ret;
			}
		}

		/*  Get checkmate status, as a set of bitflags
		*   bit 0 - Set if white is in checkmate
		*   bit 1 - Set if black is in checkmate
		*   bit 2 - Set if position is illegal
		*/
		pub fn getCheckmateStatus(FEN: &CxxString) -> u8
		{
			if FEN.as_str().is_ok()
			{
				return getCheckmateStatus(FEN.as_str().ok().unwrap());
			}
		}

		/*  Is a move legal?
		*   @param FEN The FEN string to use to check legality
		*   @param AN The algebraic notation form of the move
		*/
		pub fn isLegal(FEN: &CxxString, AN: &CxxString) -> bool
		{
			if FEN.as_str().is_ok() && AN.as_str().is_ok()
			{
				return isLegal(FEN.as_str().ok().unwrap(), AN.as_str().ok().unwrap());
			}
		}
	}
}
