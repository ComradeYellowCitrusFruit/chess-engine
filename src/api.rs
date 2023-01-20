use std::{string::String, option::Option};

/*  Generate next move
*   @param FEN The FEN string to use to generate the next move from
*   @param depth How many moves to think ahead, less means a dumber AI, faster compuation, and less memory used
*/
pub fn generateMove(FEN: &str, depth: i16) -> Option<String>;

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

<<<<<<< HEAD
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
        pub fn generateMove(FEN: &str, depth: i16) -> CxxString;

        /*  Get checkmate status, as a set of bitflags
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
    }
}
=======
// TODO: Add a cxx bridge
>>>>>>> 69082e29b37b4ee0817ce4ca2de694cbee00e4ea
