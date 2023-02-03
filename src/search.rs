/*
*   name: src/search.rs
*   author: https://github.com/ComradeYellowCitrusFruit
*   license: GPL-3.0-only
*/

use std::{cmp::*};

pub use crate::{ board::Board, fen::*, moves::*, position::Position, piece::PieceType };

// Perform alpha-beta pruning search for the best move, fail hard version
pub fn alphabeta(node: Board, depth: i16, alpha: i64, beta: i64, isBlack: bool) -> i64
{
    if depth == 0
    {
        if isBlack
        {
            return if node.isWhiteInCheckmate()
            {
                (node.blackCentipawns() - node.whiteCentipawns() + PieceType::WKing.centipawns()) as i64
            }
            else
            {
                (node.blackCentipawns() - node.whiteCentipawns()) as i64
            };
        }
        else
        {
            return if node.isWhiteInCheckmate()
            {
                (node.whiteCentipawns() - node.blackCentipawns() + PieceType::BKing.centipawns()) as i64
            }
            else
            {
                (node.whiteCentipawns() - node.blackCentipawns()) as i64
            };
        }
    }

    let value: i64 = i64::MIN;

    for i in node.generateMoves()
    {
        value = max(value, alphabeta(i, depth - 1, alpha, beta, isBlack));
        if value > beta
        {
            break;
        }
        alpha = max(alpha, value);
    }
    return value;
}