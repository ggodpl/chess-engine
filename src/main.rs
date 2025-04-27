use std::io;

use mchess::uci::Uci;

fn main() -> io::Result<()> {
    let mut uci = Uci::new();
    uci.run()
}