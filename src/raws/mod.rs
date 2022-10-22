use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

mod block_type;
mod formats;

pub use block_type::*;
pub use formats::*;

lazy_static! {
    pub static ref RAWS: Lazy<RwLock<Raws>> = Lazy::new(|| RwLock::new(Raws::new()));
}

pub struct Raws {
    pub biomes: Biomes,
    pub names: Names,
}

impl Raws {
    fn new() -> Self { Self { biomes: Biomes::new(), names: Names::new() } }

    fn load_index(&self) -> Vec<String> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open("raws/index.txt").unwrap();
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|l| l.unwrap())
            .filter(|l| !l.is_empty() && !l.starts_with("# "))
            .collect()
    }

    fn load(&mut self) {
        self.names = load_names();

        let bundles = self.load_index();
        bundles.iter().for_each(|bf| {
            let bundle = RawBundle::load(bf);
            bundle.merge(self);
        });
    }
}

pub fn load_raws() { RAWS.write().load(); }
