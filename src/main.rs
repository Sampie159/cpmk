use clap::Parser;
use cpmk::Cpmk;

fn main() {
    let cpmk = Cpmk::parse();

    cpmk.setup_project();
}
