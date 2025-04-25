use crate::{bitboard::{AB_FILE_INV, A_FILE_INV, GH_FILE_INV, H_FILE_INV}, piece::PieceColor};

use super::{Position, Vector};

pub struct AttackTables {
    pub knight_attacks: [u64; 64],
    pub king_attacks: [u64; 64],
    pub pawn_attacks: [[u64; 64]; 2],
    pub ray_masks: [[u64; 64]; 64],
    pub line_masks: [[u64; 64]; 64],
}

impl AttackTables {
    pub fn new() -> Self {
        let mut tables = AttackTables {
            knight_attacks: [0; 64],
            king_attacks: [0; 64],
            pawn_attacks: [[0; 64]; 2],
            ray_masks: [[0; 64]; 64],
            line_masks: [[0; 64]; 64],
        };

        tables.init_knight_attacks();
        tables.init_king_attacks();
        tables.init_pawn_attacks();
        tables.init_dir_masks();

        tables
    }

    fn init_knight_attacks(&mut self) {
        for index in 0..64 {
            let square = 1u64 << index;
            self.knight_attacks[index] = ((square << 17) & A_FILE_INV) |
                ((square << 15) & H_FILE_INV) |
                ((square << 10) & AB_FILE_INV) |
                ((square >> 6) & AB_FILE_INV) |
                ((square >> 15) & A_FILE_INV) |
                ((square >> 17) & H_FILE_INV) |
                ((square << 6) & GH_FILE_INV) |
                ((square >> 10) & GH_FILE_INV);
        }
    }

    fn init_king_attacks(&mut self) {
        for index in 0..64 {
            let square = 1u64 << index;
            self.king_attacks[index] = ((square << 1) & A_FILE_INV) |
                ((square >> 1) & H_FILE_INV) |
                (square << 8) |
                (square >> 8) |
                ((square << 9) & A_FILE_INV) |
                ((square << 7) & H_FILE_INV) |
                ((square >> 7) & A_FILE_INV) |
                ((square >> 9) & H_FILE_INV);
        }
    }

    fn init_pawn_attacks(&mut self) {
        for index in 0..64 {
            let square = 1u64 << index;

            let white = ((square >> 9) & H_FILE_INV) | ((square >> 7) & A_FILE_INV);
            let black = ((square << 9) & A_FILE_INV) | ((square << 7) & H_FILE_INV);

            self.pawn_attacks[PieceColor::White.index()][index] = white;
            self.pawn_attacks[PieceColor::Black.index()][index] = black;
        }
    }

    fn init_dir_masks(&mut self) {
        for index1 in 0..64 {
            for index2 in 0..64 {
                if index1 == index2 { continue; }
                let square1 = 1u64 << index1;
                let square2 = 1u64 << index2;

                if self.is_aligned(square1, square2) {
                    self.ray_masks[index1][index2] = self.cast_ray(square1, square2);
                    self.line_masks[index1][index2] = self.gen_line_mask(square1, square2);
                }
            }
        }
    }

    fn is_aligned(&self, square1: u64, square2: u64) -> bool {
        if square1 == square2 { return true; }

        let pos1 = Position::from_bitboard(square1);
        let pos2 = Position::from_bitboard(square2);

        if pos1.x == pos2.x || pos1.y == pos2.y {
            return true;
        }

        (pos1.x as i32 - pos2.x as i32).abs() == (pos1.y as i32 - pos2.y as i32).abs()
    }

    fn cast_ray(&self, square1: u64, square2: u64) -> u64 {
        let pos1 = Position::from_bitboard(square1);
        let pos2 = Position::from_bitboard(square2);

        let dir = Vector::between(pos1, pos2);
        let mut x = pos1.x as i32 + dir.x;
        let mut y = pos1.y as i32 + dir.y;

        let mut mask = 0;

        while x >= 0 && x <= 7 && y >= 0 && y <= 7 {
            if x == pos2.x as i32 && y == pos2.y as i32 { break; }
            mask |= Position::bitboard(x as usize, y as usize);

            x += dir.x;
            y += dir.y;
        }

        mask
    }

    fn gen_line_mask(&self, square1: u64, square2: u64) -> u64 {
        let pos1 = Position::from_bitboard(square1);
        let pos2 = Position::from_bitboard(square2);

        let dir = Vector::between(pos1, pos2);

        let mut mask = square1 | square2;

        let mut x = pos1.x as i32 + dir.x;
        let mut y = pos1.y as i32 + dir.y;
        while x >= 0 && x <= 7 && y >= 0 && y <= 7 {
            mask |= Position::bitboard(x as usize, y as usize);
            
            x += dir.x;
            y += dir.y;
        }

        let mut x = pos1.x as i32 - dir.x;
        let mut y = pos1.y as i32 - dir.y;
        while x >= 0 && x <= 7 && y >= 0 && y <= 7 {
            mask |= Position::bitboard(x as usize, y as usize);

            x -= dir.x;
            y -= dir.y;
        }

        mask
    }

    pub fn is_between(&self, square: u64, from: u64, to: u64) -> bool {
        if square == 0 || from == 0 || to == 0 { return false; }
        if square == from { return true; }
        self.ray_masks[from.trailing_zeros() as usize][to.trailing_zeros() as usize] & square != 0
    }

    pub fn get_line_mask(&self, square1: u64, square2: u64) -> u64 {
        if square1 == 0 || square2 == 0 { return 0; }
        self.line_masks[square1.trailing_zeros() as usize][square2.trailing_zeros() as usize]
    }

    pub fn get_ray(&self, square1: u64, square2: u64) -> u64 {
        if square1 == 0 || square2 == 0 { return 0; }
        self.ray_masks[square1.trailing_zeros() as usize][square2.trailing_zeros() as usize]
    }
}