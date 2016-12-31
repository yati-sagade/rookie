use std::fmt;
use errors::{Result,Error};


#[derive(PartialEq,Clone)]
pub struct BitBoard {
    board: u64,
}

impl BitBoard {
    // Returns a bitboard with no bit set.
    pub fn empty() -> BitBoard {
        BitBoard { board: 0u64 }
    }
    // Returns a bitboard with all bits set.
    pub fn full() -> BitBoard {
        BitBoard { board: u64::max_value() }
    }
    pub fn is_empty(&self) -> bool {
        self.board == 0u64
    }
    pub fn is_full(&self) -> bool {
        self.board == u64::max_value()
    }
    pub fn set(&mut self, row: u8, col: u8) -> Result<()> {
        let offset = try!(BitBoard::offset(row, col));
        self.board |= 1 << offset;
        Ok(())
    }
    pub fn get(&self, row: u8, col: u8) -> Result<bool> {
        let offset = try!(BitBoard::offset(row, col));
        Ok(((self.board >> offset) & 1) == 1)
    }
    pub fn raw(&self) -> u64 {
        self.board
    }
    pub fn and(&self, other: &BitBoard) -> BitBoard {
        BitBoard { board: self.board & other.board }
    }
    pub fn or(&self, other: &BitBoard) -> BitBoard {
        BitBoard { board: self.board | other.board }
    }
    pub fn complement(&self) -> BitBoard {
        BitBoard { board: !self.board }
    }
    pub fn xor(&self, other: &BitBoard) -> BitBoard {
        BitBoard { board: self.board ^ other.board }
    }
    pub fn offset(row: u8, col: u8) -> Result<u8> {
        if row > 7 || col > 7 {
            Err(Error::OutOfRange)
        } else {
            Ok((row << 3) | col)
        }
    }
    pub fn deconstruct_offset(offset: u8) -> Result<(u8, u8)> {
        if offset > 0b111111 {
            Err(Error::OutOfRange)
        } else {
            Ok((offset >> 3, offset & 0b111))
        }
    }
}

impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..8 {
            for col in 0..8 {
                let set = self.get(row, col).unwrap();
                try!(write!(f, "{}", if set { "x" } else { "." }));
                try!(write!(f, "{}", if col == 7 { "\n" } else { " " }));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let mut board = BitBoard::empty();
        assert_eq!(board.raw(), 0);

        assert!(board.set(0, 0).is_ok());
        assert_eq!(board.raw(), 1);

        assert!(board.set(1, 6).is_ok());
        assert_eq!(board.raw(), 0x00_00_00_00_00_00_40_01);

        assert!(board.set(7, 7).is_ok());
        assert_eq!(board.raw(), 0x80_00_00_00_00_00_40_01);

        assert!(board.set(8, 6).is_err());
        assert!(board.set(6, 8).is_err());
        assert_eq!(board.raw(), 0x80_00_00_00_00_00_40_01);

    }

    #[test]
    fn test_get() {
        let mut board = BitBoard::empty();
        assert_eq!(board.raw(), 0);

        assert!(board.set(0, 0).is_ok());
        assert_eq!(board.raw(), 1);

        assert!(board.set(1, 6).is_ok());
        assert_eq!(board.raw(), 0x00_00_00_00_00_00_40_01);

        assert!(board.set(7, 7).is_ok());
        assert_eq!(board.raw(), 0x80_00_00_00_00_00_40_01);

        assert!(board.get(0, 0).unwrap());
        assert!(board.get(1, 6).unwrap());
        assert!(board.get(7, 7).unwrap());

        assert!(!board.get(0, 1).unwrap());
        assert!(!board.get(1, 0).unwrap());
        assert!(!board.get(7, 0).unwrap());
        assert!(!board.get(0, 7).unwrap());

        assert!(board.get(8, 0).is_err());
        assert!(board.get(0, 10).is_err());
    }

    #[test]
    fn test_ctors() {
        let board = BitBoard::empty();
        assert!(board.is_empty());
        assert_eq!(board.raw(), 0u64);

        let board = BitBoard::full();
        assert!(board.is_full());
        assert_eq!(board.raw(), 0xffff_ffff_ffff_ffff);
    }

    #[test]
    fn test_set_operations() {
        {
            let b = BitBoard::empty();
            let c = BitBoard::full();
            assert_eq!(b.and(&c), BitBoard::empty());
            assert_eq!(b.or(&c), BitBoard::full());
        }
        {
            let mut a = BitBoard::empty();
            let mut b = BitBoard::empty();
            a.set(4, 5).unwrap();
            b.set(6, 7).unwrap();
            assert_eq!(a.and(&b) , BitBoard::empty());
            assert_eq!(a.or(&b).raw(), 0x0080_0020_0000_0000);
        }
        {
            let mut a = BitBoard::empty();
            a.set(4, 5).unwrap();
            a.set(6, 7).unwrap();
            a.set(0, 0).unwrap();
            let b = a.complement();

            assert_eq!(a.and(&b) , BitBoard::empty());
            assert_eq!(a.or(&b), BitBoard::full());
            assert_eq!(b.raw(), 0xff7f_ffdf_ffff_fffe); // (1 << 37) | (1 << 55) | 1
        }
    }
}
