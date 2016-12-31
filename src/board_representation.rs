use bitboard::BitBoard;
use std::collections::HashMap;
use errors::{Result,Error};

use self::Color::*;
use self::Piece::*;
use self::Square::*;


#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub enum Color {
    Black,
    White
}

#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub enum Piece {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Debug)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
	// The square at (0, 0) when the board is laid out with the white side down
    // is A8 and the square at (7, 7) is H1. This is because the ranks run
    // *bottom-to-top*, with the top-most rank being 8.
    pub fn row_col(&self) -> (u8, u8) {
	    match self {	
			&A8 => (0, 0), &B8 => (0, 1), &C8 => (0, 2), &D8 => (0, 3), &E8 => (0, 4), &F8 => (0, 5), &G8 => (0, 6), &H8 => (0, 7), 
			&A7 => (1, 0), &B7 => (1, 1), &C7 => (1, 2), &D7 => (1, 3), &E7 => (1, 4), &F7 => (1, 5), &G7 => (1, 6), &H7 => (1, 7), 
			&A6 => (2, 0), &B6 => (2, 1), &C6 => (2, 2), &D6 => (2, 3), &E6 => (2, 4), &F6 => (2, 5), &G6 => (2, 6), &H6 => (2, 7), 
			&A5 => (3, 0), &B5 => (3, 1), &C5 => (3, 2), &D5 => (3, 3), &E5 => (3, 4), &F5 => (3, 5), &G5 => (3, 6), &H5 => (3, 7), 
			&A4 => (4, 0), &B4 => (4, 1), &C4 => (4, 2), &D4 => (4, 3), &E4 => (4, 4), &F4 => (4, 5), &G4 => (4, 6), &H4 => (4, 7), 
			&A3 => (5, 0), &B3 => (5, 1), &C3 => (5, 2), &D3 => (5, 3), &E3 => (5, 4), &F3 => (5, 5), &G3 => (5, 6), &H3 => (5, 7), 
			&A2 => (6, 0), &B2 => (6, 1), &C2 => (6, 2), &D2 => (6, 3), &E2 => (6, 4), &F2 => (6, 5), &G2 => (6, 6), &H2 => (6, 7), 
			&A1 => (7, 0), &B1 => (7, 1), &C1 => (7, 2), &D1 => (7, 3), &E1 => (7, 4), &F1 => (7, 5), &G1 => (7, 6), &H1 => (7, 7),
        }
    }
    pub fn from_row_col(row: u8, col: u8) -> Result<Square> {
        match (row, col) {
			(0, 0) => Ok(A8), (0, 1) => Ok(B8), (0, 2) => Ok(C8), (0, 3) => Ok(D8), (0, 4) => Ok(E8), (0, 5) => Ok(F8), (0, 6) => Ok(G8), (0, 7) => Ok(H8), 
			(1, 0) => Ok(A7), (1, 1) => Ok(B7), (1, 2) => Ok(C7), (1, 3) => Ok(D7), (1, 4) => Ok(E7), (1, 5) => Ok(F7), (1, 6) => Ok(G7), (1, 7) => Ok(H7), 
			(2, 0) => Ok(A6), (2, 1) => Ok(B6), (2, 2) => Ok(C6), (2, 3) => Ok(D6), (2, 4) => Ok(E6), (2, 5) => Ok(F6), (2, 6) => Ok(G6), (2, 7) => Ok(H6), 
			(3, 0) => Ok(A5), (3, 1) => Ok(B5), (3, 2) => Ok(C5), (3, 3) => Ok(D5), (3, 4) => Ok(E5), (3, 5) => Ok(F5), (3, 6) => Ok(G5), (3, 7) => Ok(H5), 
			(4, 0) => Ok(A4), (4, 1) => Ok(B4), (4, 2) => Ok(C4), (4, 3) => Ok(D4), (4, 4) => Ok(E4), (4, 5) => Ok(F4), (4, 6) => Ok(G4), (4, 7) => Ok(H4), 
			(5, 0) => Ok(A3), (5, 1) => Ok(B3), (5, 2) => Ok(C3), (5, 3) => Ok(D3), (5, 4) => Ok(E3), (5, 5) => Ok(F3), (5, 6) => Ok(G3), (5, 7) => Ok(H3), 
			(6, 0) => Ok(A2), (6, 1) => Ok(B2), (6, 2) => Ok(C2), (6, 3) => Ok(D2), (6, 4) => Ok(E2), (6, 5) => Ok(F2), (6, 6) => Ok(G2), (6, 7) => Ok(H2), 
			(7, 0) => Ok(A1), (7, 1) => Ok(B1), (7, 2) => Ok(C1), (7, 3) => Ok(D1), (7, 4) => Ok(E1), (7, 5) => Ok(F1), (7, 6) => Ok(G1), (7, 7) => Ok(H1),
            _ => Err(Error::OutOfRange),
        }
    }
    pub fn from_str(s: &str) -> Result<Square> {
        let s = &s.to_uppercase();
        match s.as_str() {
			"A1" => Ok(A1), "A2" => Ok(A2), "A3" => Ok(A3), "A4" => Ok(A4), "A5" => Ok(A5), "A6" => Ok(A6), "A7" => Ok(A7), "A8" => Ok(A8), 
			"B1" => Ok(B1), "B2" => Ok(B2), "B3" => Ok(B3), "B4" => Ok(B4), "B5" => Ok(B5), "B6" => Ok(B6), "B7" => Ok(B7), "B8" => Ok(B8), 
			"C1" => Ok(C1), "C2" => Ok(C2), "C3" => Ok(C3), "C4" => Ok(C4), "C5" => Ok(C5), "C6" => Ok(C6), "C7" => Ok(C7), "C8" => Ok(C8), 
			"D1" => Ok(D1), "D2" => Ok(D2), "D3" => Ok(D3), "D4" => Ok(D4), "D5" => Ok(D5), "D6" => Ok(D6), "D7" => Ok(D7), "D8" => Ok(D8), 
			"E1" => Ok(E1), "E2" => Ok(E2), "E3" => Ok(E3), "E4" => Ok(E4), "E5" => Ok(E5), "E6" => Ok(E6), "E7" => Ok(E7), "E8" => Ok(E8), 
			"F1" => Ok(F1), "F2" => Ok(F2), "F3" => Ok(F3), "F4" => Ok(F4), "F5" => Ok(F5), "F6" => Ok(F6), "F7" => Ok(F7), "F8" => Ok(F8), 
			"G1" => Ok(G1), "G2" => Ok(G2), "G3" => Ok(G3), "G4" => Ok(G4), "G5" => Ok(G5), "G6" => Ok(G6), "G7" => Ok(G7), "G8" => Ok(G8), 
			"H1" => Ok(H1), "H2" => Ok(H2), "H3" => Ok(H3), "H4" => Ok(H4), "H5" => Ok(H5), "H6" => Ok(H6), "H7" => Ok(H7), "H8" => Ok(H8),
		    _ => Err(Error::InvalidSquare),
        }
    }
}

// To aid iteration.
pub const SQUARES: &'static [Square] = &[
    Square::A8, Square::B8, Square::C8, Square::D8, Square::E8, Square::F8, Square::G8, Square::H8,
    Square::A7, Square::B7, Square::C7, Square::D7, Square::E7, Square::F7, Square::G7, Square::H7,
    Square::A6, Square::B6, Square::C6, Square::D6, Square::E6, Square::F6, Square::G6, Square::H6,
    Square::A5, Square::B5, Square::C5, Square::D5, Square::E5, Square::F5, Square::G5, Square::H5,
    Square::A4, Square::B4, Square::C4, Square::D4, Square::E4, Square::F4, Square::G4, Square::H4,
    Square::A3, Square::B3, Square::C3, Square::D3, Square::E3, Square::F3, Square::G3, Square::H3,
    Square::A2, Square::B2, Square::C2, Square::D2, Square::E2, Square::F2, Square::G2, Square::H2,
    Square::A1, Square::B1, Square::C1, Square::D1, Square::E1, Square::F1, Square::G1, Square::H1,
];

pub type ColoredPiece = (Color, Piece);

pub const WHITE_ROOK: ColoredPiece = (White, Rook);
pub const WHITE_KNIGHT: ColoredPiece = (White, Knight);
pub const WHITE_BISHOP: ColoredPiece = (White, Bishop);
pub const WHITE_QUEEN: ColoredPiece = (White, Queen);
pub const WHITE_KING: ColoredPiece = (White, King);
pub const WHITE_PAWN: ColoredPiece = (White, Pawn);
pub const BLACK_ROOK: ColoredPiece = (Black, Rook);
pub const BLACK_KNIGHT: ColoredPiece = (Black, Knight);
pub const BLACK_BISHOP: ColoredPiece = (Black, Bishop);
pub const BLACK_QUEEN: ColoredPiece = (Black, Queen);
pub const BLACK_KING: ColoredPiece = (Black, King);
pub const BLACK_PAWN: ColoredPiece = (Black, Pawn);

pub const STARTING_BOARD_POSITION: &'static [Option<ColoredPiece>] = &[
    Some(BLACK_ROOK), Some(BLACK_KNIGHT), Some(BLACK_BISHOP), Some(BLACK_QUEEN), Some(BLACK_KING), Some(BLACK_BISHOP), Some(BLACK_KNIGHT), Some(BLACK_ROOK),
    Some(BLACK_PAWN), Some(BLACK_PAWN),   Some(BLACK_PAWN),   Some(BLACK_PAWN),  Some(BLACK_PAWN), Some(BLACK_PAWN),   Some(BLACK_PAWN),   Some(BLACK_PAWN),
    None,             None,               None,               None,              None,             None,               None,               None,
    None,             None,               None,               None,              None,             None,               None,               None,
    None,             None,               None,               None,              None,             None,               None,               None,
    None,             None,               None,               None,              None,             None,               None,               None,
    Some(WHITE_PAWN), Some(WHITE_PAWN),   Some(WHITE_PAWN),   Some(WHITE_PAWN),  Some(WHITE_PAWN), Some(WHITE_PAWN),   Some(WHITE_PAWN),   Some(WHITE_PAWN),
    Some(WHITE_ROOK), Some(WHITE_KNIGHT), Some(WHITE_BISHOP), Some(WHITE_QUEEN), Some(WHITE_KING), Some(WHITE_BISHOP), Some(WHITE_KNIGHT), Some(WHITE_ROOK),
];

pub struct Board {
    pub piece_bitboards: HashMap<ColoredPiece, BitBoard>,
    // Empty squares.
    pub empty_bitboard: BitBoard,
    pub current_player: Color,
}

impl Board {
    pub fn new_starting() -> Board {
        let mut ret = Board{
            piece_bitboards: HashMap::new(),
            empty_bitboard: BitBoard::empty(),
            current_player: Color::White,
        };
        for color in &[Black, White] {
            for piece in &[Rook, Knight, Bishop, Queen, King, Pawn] {
                ret.piece_bitboards.insert((color.clone(), piece.clone()), BitBoard::empty());
            }
        }
        for (idx, piece) in STARTING_BOARD_POSITION.iter().enumerate() {
            let (row, col) = BitBoard::deconstruct_offset(idx as u8).unwrap();
            if let &Some(ref piece) = piece {
                ret.piece_bitboards
                   .get_mut(piece)
                   .unwrap()
                   .set(row, col)
                   .unwrap();
            } else {
                ret.empty_bitboard.set(row, col).unwrap();
            }
        }
        ret
    }
    pub fn piece_on_row_col(&self, row: u8, col: u8) -> Result<Option<ColoredPiece>> {
        for (piece, board) in &self.piece_bitboards {
            if try!(board.get(row, col)) {
                return Ok(Some(piece.clone()));
            }
        }
        Ok(None)
    }
    pub fn piece_on_square(&self, square: &Square) -> Option<ColoredPiece> {
        let (row, col) = square.row_col();
        self.piece_on_row_col(row, col).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::Square::*;
    use bitboard::BitBoard;

    #[test]
    fn test_init() {
        let board = Board::new_starting();
        assert_eq!(board.piece_bitboards.len(), 12);
        fn expected_piece_on_square(square: &Square) -> Option<ColoredPiece> {
            match square {
				&A8 => Some(BLACK_ROOK), &B8 => Some(BLACK_KNIGHT), &C8 => Some(BLACK_BISHOP), &D8 => Some(BLACK_QUEEN),
				&E8 => Some(BLACK_KING), &F8 => Some(BLACK_BISHOP), &G8 => Some(BLACK_KNIGHT), &H8 => Some(BLACK_ROOK),

				&A7 => Some(BLACK_PAWN), &B7 => Some(BLACK_PAWN),   &C7 => Some(BLACK_PAWN),   &D7 => Some(BLACK_PAWN),
				&E7 => Some(BLACK_PAWN), &F7 => Some(BLACK_PAWN),   &G7 => Some(BLACK_PAWN),   &H7 => Some(BLACK_PAWN),

				&A6 => None, &B6 => None, &C6 => None, &D6 => None, &E6 => None, &F6 => None, &G6 => None, &H6 => None,
				&A5 => None, &B5 => None, &C5 => None, &D5 => None, &E5 => None, &F5 => None, &G5 => None, &H5 => None,
				&A4 => None, &B4 => None, &C4 => None, &D4 => None, &E4 => None, &F4 => None, &G4 => None, &H4 => None,
				&A3 => None, &B3 => None, &C3 => None, &D3 => None, &E3 => None, &F3 => None, &G3 => None, &H3 => None,

				&A2 => Some(WHITE_PAWN), &B2 => Some(WHITE_PAWN),   &C2 => Some(WHITE_PAWN),   &D2 => Some(WHITE_PAWN),
				&E2 => Some(WHITE_PAWN), &F2 => Some(WHITE_PAWN),   &G2 => Some(WHITE_PAWN),   &H2 => Some(WHITE_PAWN),
				&A1 => Some(WHITE_ROOK), &B1 => Some(WHITE_KNIGHT), &C1 => Some(WHITE_BISHOP), &D1 => Some(WHITE_QUEEN),
				&E1 => Some(WHITE_KING), &F1 => Some(WHITE_BISHOP), &G1 => Some(WHITE_KNIGHT), &H1 => Some(WHITE_ROOK)
            }
        }
        for square in SQUARES {
            assert_eq!(board.piece_on_square(square), expected_piece_on_square(square));
        }
    }

    #[test]
    fn test_square_offsets() {
        fn expected_offset(sq: &Square) -> u8 {
            match sq {
				&A8 =>  0, &B8 =>  1, &C8 =>  2, &D8 =>  3, &E8 =>  4, &F8 =>  5, &G8 =>  6, &H8 =>  7, 
				&A7 =>  8, &B7 =>  9, &C7 => 10, &D7 => 11, &E7 => 12, &F7 => 13, &G7 => 14, &H7 => 15, 
				&A6 => 16, &B6 => 17, &C6 => 18, &D6 => 19, &E6 => 20, &F6 => 21, &G6 => 22, &H6 => 23, 
				&A5 => 24, &B5 => 25, &C5 => 26, &D5 => 27, &E5 => 28, &F5 => 29, &G5 => 30, &H5 => 31, 
				&A4 => 32, &B4 => 33, &C4 => 34, &D4 => 35, &E4 => 36, &F4 => 37, &G4 => 38, &H4 => 39, 
				&A3 => 40, &B3 => 41, &C3 => 42, &D3 => 43, &E3 => 44, &F3 => 45, &G3 => 46, &H3 => 47, 
				&A2 => 48, &B2 => 49, &C2 => 50, &D2 => 51, &E2 => 52, &F2 => 53, &G2 => 54, &H2 => 55, 
				&A1 => 56, &B1 => 57, &C1 => 58, &D1 => 59, &E1 => 60, &F1 => 61, &G1 => 62, &H1 => 63,
            }
        }
        for square in SQUARES {
            let (row, col) = square.row_col();
	        let offset = BitBoard::offset(row, col).unwrap();
            assert_eq!(offset, expected_offset(square));
        }        
    }
    
}
