use std::{cmp::*};

// Perform alpha-beta pruning search for the best move, fail hard version
fn alphabeta(node: Board, depth: i16, alpha: i64, beta: i64, isBlack: bool) -> i64
{
    if depth == 0
    {
        if isBlack
        {
            return if node.isWhiteInCheckmate()
            {
                node.blackCentipawns() - node.whiteCentipawns() + PieceType::wKing.centipawns()
            }
            else
            {
                node.blackCentipawns() - node.whiteCentipawns()
            };
        }
        else
        {
            return if node.isWhiteInCheckmate()
            {
                node.whiteCentipawns() - node.blackCentipawns() + PieceType::bKing.centipawns()
            }
            else
            {
                node.whiteCentipawns() - node.blackCentipawns()
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