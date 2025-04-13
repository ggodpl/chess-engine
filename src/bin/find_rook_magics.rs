use mchess::moves::magic::Magic;
use rand::Rng;

const MAX_TRIES: usize = 1000000;

fn find_magics() {
    let magic = Magic::new();
    for square in 0..64 {
        let mask = magic.rook_masks[square];
        let shift = magic.rook_shifts[square];

        let size = 1 << mask.count_ones();
        let occupancy = Magic::get_blockers(mask);
        let attacks = magic.rook_attacks[square].clone();

        'ext: for _ in 0..MAX_TRIES {
            let candidate = random_magic();
            let mut used = vec![0u64; size];
            let mut attack_table = vec![0u64; size];

            for i in 0..size {
                let index = ((occupancy[i].wrapping_mul(candidate)) >> shift) as usize;
                if used[index] == 0 {
                    used[index] = 1;
                    attack_table[index] = attacks[i];
                } else if attack_table[index] != attacks[i] {
                    continue 'ext; // collision
                }
            }

            println!("Square {}: 0x{:016X}, shift: {}", square, candidate, 64 - shift);
            break;
        }
    }
}

fn random_magic() -> u64 {
    let mut rng = rand::rng();
    rng.random::<u64>() & rng.random::<u64>() & rng.random::<u64>()
}

fn main() {
    find_magics();
}