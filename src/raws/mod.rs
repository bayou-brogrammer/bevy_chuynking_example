use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use parking_lot::RwLock;

mod block_type;
mod formats;
mod strata;

pub use block_type::*;
pub use formats::*;
pub use strata::*;

lazy_static! {
    pub static ref RAWS: Lazy<RwLock<Raws>> = Lazy::new(|| RwLock::new(Raws::new()));
}

pub struct Raws {
    pub names: Names,
    pub biomes: Biomes,
    pub plants: Plants,
    pub materials: Materials,
}

impl Raws {
    fn new() -> Self {
        Self {
            names: Names::new(),
            plants: Plants::new(),
            biomes: Biomes::new(),
            materials: Materials::new(),
        }
    }

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

pub fn load_raws() {
    RAWS.write().load();
    strata::verify_strata();
}
