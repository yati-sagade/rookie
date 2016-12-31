extern crate rookie;
use std::env;
use rookie::board_representation::*;
use rookie::board_representation::Square::*;
use rookie::bitboard::BitBoard;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    match args.get(1).map(|s| s.as_str()) {
        Some("offset") => {
            let x = args.get(2).expect("Expected a square name");
            if let Ok(square) = Square::from_str(x) {
                let (row, col) = square.row_col();
                let offset = BitBoard::offset(row, col).unwrap();
                println!("{}", offset);
            } else {
                println!("Invalid square '{}'", x);
            }
        },
        _ => {
            let b = Board::new_starting();
            println!("{:?}", b.piece_bitboards.get(&WHITE_ROOK).unwrap());
            println!("{:?}", b.piece_bitboards.get(&BLACK_PAWN).unwrap());
            println!("{:?}", b.empty_bitboard);
            let squares = vec![A1, B1, C1, D1, E1, F1, G1, H1, A8, B8, C8, D8, E8, F8, H8];
            for square in &squares {
                println!("{:?} = {}", square, {
                    let (row, col) = square.row_col();
                    BitBoard::offset(row, col).unwrap()
                });
            }
        }
    }
}
