use super::{values::{BISHOP_MAGICS, ROOK_MAGICS}, Position};

pub struct Magic {
    pub rook_masks: [u64; 64],
    pub rook_shifts: [u64; 64],
    pub rook_attacks: [Vec<u64>; 64],

    pub bishop_masks: [u64; 64],
    pub bishop_shifts: [u64; 64],
    pub bishop_attacks: [Vec<u64>; 64]
}

impl Magic {
    pub fn new() -> Self {
        let mut magic =  Magic {
            rook_masks: [0; 64],
            rook_shifts: [0; 64],
            rook_attacks: array_init::array_init(|_| Vec::new()),
            bishop_masks: [0; 64],
            bishop_shifts: [0; 64],
            bishop_attacks: array_init::array_init(|_| Vec::new())
        };

        magic.gen_rook_masks();
        magic.gen_rook_attacks();

        magic.gen_bishop_masks();
        magic.gen_bishop_attacks();

        magic
    }

    fn gen_rook_masks(&mut self) {
        for square in 0..64 {
            let mut mask = 0u64;
            let rank = square / 8;
            let file = square % 8;

            for r in (rank + 1)..7 {
                mask |= Position::bitboard(file, r);
            }

            for r in 1..rank {
                mask |= Position::bitboard(file, rank - r);
            }

            for f in (file + 1)..7 {
                mask |= Position::bitboard(f, rank);
            }

            for f in 1..file {
                mask |= Position::bitboard(file - f, rank);
            }

            self.rook_masks[square] = mask;

            self.rook_shifts[square] = 64 - mask.count_ones() as u64;
        }
    }

    fn gen_bishop_masks(&mut self) {
        for square in 0..64 {
            let mut mask = 0u64;
            let rank = square / 8;
            let file = square % 8;

            let mut r = rank + 1;
            let mut f = file + 1;
            while r < 7 && f < 7 {
                mask |= Position::bitboard(f, r);
                r += 1;
                f += 1;
            }

            if file > 0 {
                let mut r = rank + 1;
                let mut f = file - 1;
                while r < 7 && f > 0 {
                    mask |= Position::bitboard(f, r);
                    r += 1;
                    f -= 1;
                }
            }

            if rank > 0 {
                let mut r = rank - 1;
                let mut f = file + 1;
                while r > 0 && f < 7 {
                    mask |= Position::bitboard(f, r);
                    r -= 1;
                    f += 1;
                }
            }

            if rank > 0 && file > 0 {
                let mut r = rank - 1;
                let mut f = file - 1;
                while r > 0 && f > 0 {
                    mask |= Position::bitboard(f, r);
                    r -= 1;
                    f -= 1;
                }
            }

            self.bishop_masks[square] = mask;
            
            self.bishop_shifts[square] = 64 - mask.count_ones() as u64;
        }
    }

    pub fn get_occupancy(mask: u64) -> Vec<u64> {
        let mut indices = Vec::with_capacity(64);

        for square in 0..64 {
            if mask & (1u64 << square) != 0 {
                indices.push(square);
            }
        }

        let size = indices.len();
        let combinations = 1 << size;
        let mut bitboards = Vec::with_capacity(combinations);

        for i in 0..combinations {
            let mut blocker = 0u64;
            for j in 0..size {
                if i & (1 << j) != 0 {
                    blocker |= 1u64 << indices[j];
                }
            }
            bitboards.push(blocker);
        }

        bitboards
    }

    fn gen_rook_attacks(&mut self) {
        for square in 0..64 {
            let magic = ROOK_MAGICS[square];
            let shift = self.rook_shifts[square];

            let bits = 64 - shift;
            let size = 1 << bits;
            let mut table = vec![0; size];

            let mask = self.rook_masks[square];
            let occupancy = Magic::get_occupancy(mask);

            for block in occupancy {
                let index = ((block.wrapping_mul(magic)) >> shift) as usize;
                let moves = self.get_rook_attacks(square, block);
                table[index] = moves;
            }

            self.rook_attacks[square] = table;
        }
    }

    fn gen_bishop_attacks(&mut self) {
        for square in 0..64 {
            let magic = BISHOP_MAGICS[square];
            let shift = self.bishop_shifts[square];

            let bits = 64 - shift;
            let size = 1 << bits;
            let mut table = vec![0; size];

            let mask = self.bishop_masks[square];
            let occupancy = Magic::get_occupancy(mask);

            for block in occupancy {
                let index = ((block.wrapping_mul(magic)) >> shift) as usize;
                let moves = self.get_bishop_attacks(square, block);
                table[index] = moves;
            }

            self.bishop_attacks[square] = table;
        }
    }

    pub fn get_rook_attacks(&self, square: usize, occupancy: u64) -> u64 {
        let mut attacks = 0u64;
        let rank = square / 8;
        let file = square % 8;

        for r in (rank + 1)..8 {
            let target = Position::bitboard(file, r);
            attacks |= target;
            if occupancy & target != 0 { break; }
        }

        for r in 0..rank {
            let target = Position::bitboard(file, rank - r);
            attacks |= target;
            if occupancy & target != 0 { break; }
        }

        for f in (file + 1)..8 {
            let target = Position::bitboard(f, rank);
            attacks |= target;
            if occupancy & target != 0 { break; }
        }

        for f in 0..file {
            let target = Position::bitboard(file - f, rank);
            attacks |= target;
            if occupancy & target != 0 { break; }
        }

        attacks
    }

    pub fn get_bishop_attacks(&self, square: usize, occupancy: u64) -> u64 {
        let mut attacks = 0u64;
        let rank = square / 8;
        let file = square % 8;

        let mut r = rank + 1;
        let mut f = file + 1;
        while r < 8 && f < 8 {
            let target = Position::bitboard(f, r);
            attacks |= target;
            if occupancy & target != 0 { break; }
            r += 1;
            f += 1;
        }

        if file > 0 {
            let mut r = rank + 1;
            let mut f = file - 1;
            while r < 8 {
                let target = Position::bitboard(f, r);
                attacks |= target;
                if occupancy & target != 0 { break; }
                if f == 0 { break; }
                r += 1;
                f -= 1;
            }
        }

        if rank > 0 {
            let mut r = rank - 1;
            let mut f = file + 1;
            while f < 8 {
                let target = Position::bitboard(f, r);
                attacks |= target;
                if occupancy & target != 0 { break; }
                if r == 0 { break; }
                r -= 1;
                f += 1;
            }
        }

        if rank > 0 && file > 0 {
            let mut r = rank - 1;
            let mut f = file - 1;
            loop {
                let target = Position::bitboard(f, r);
                attacks |= target;
                if occupancy & target != 0 { break; }
                if r == 0 || f == 0 { break; }
                r -= 1;
                f -= 1;
            }
        }

        attacks
    }

    pub fn get_rook_moves(&self, square: usize, occupancy: u64) -> u64 {
        let key = occupancy & self.rook_masks[square];
        let index = ((key.wrapping_mul(ROOK_MAGICS[square])) >> self.rook_shifts[square]) as usize;
        self.rook_attacks[square][index]
    }

    pub fn get_bishop_moves(&self, square: usize, occupancy: u64) -> u64 {
        let key = occupancy & self.bishop_masks[square];
        let index = ((key.wrapping_mul(BISHOP_MAGICS[square])) >> self.bishop_shifts[square]) as usize;
        self.bishop_attacks[square][index]
    }

    pub fn get_queen_moves(&self, square: usize, occupancy: u64) -> u64 {
        self.get_rook_moves(square, occupancy) | self.get_bishop_moves(square, occupancy)
    }
}