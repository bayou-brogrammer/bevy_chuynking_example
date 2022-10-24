use super::*;
use bevy::utils::HashSet;

struct RainParticle {
    load: i32,
    cycles: u32,
    raining: bool,
    position: usize,
    history: HashSet<usize>,
}

impl RainParticle {
    fn take_water(&mut self, planet: &mut Planet, amount: i32) {
        if amount <= planet.landblocks[self.position].rainfall_mm {
            planet.landblocks[self.position].rainfall_mm -= amount;
            self.load += amount;
        } else {
            self.load += planet.landblocks[self.position].rainfall_mm;
            planet.landblocks[self.position].rainfall_mm = 0;
        }
    }

    fn dump_water(&mut self, planet: &mut Planet, amount: i32) {
        if self.load >= amount {
            self.load -= amount;
            planet.landblocks[self.position].rainfall_mm += amount;
        } else {
            planet.landblocks[self.position].rainfall_mm += self.load;
            self.load = 0;
        }
    }
}

pub fn planet_rainfall(planet: &mut Planet) {
    let lb_copy = planet.landblocks.clone();
    planet.landblocks.iter_mut().for_each(|lb| {
        let mut neighbors: Vec<(Direction, f32)> = lb
            .neighbors
            .iter()
            .map(|n| (n.0, lb_copy[n.1].air_pressure_kpa))
            //.filter(|n| n.1 <= lb.air_pressure_kpa)
            .collect();

        if neighbors.is_empty() {
            lb.prevailing_wind = Direction::None;
        } else {
            neighbors.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            lb.prevailing_wind = neighbors[0].0;
        }
    });

    let mut rain_particles = Vec::with_capacity(WORLD_TILES_COUNT);
    for i in 0..planet.landblocks.len() {
        rain_particles.push(RainParticle {
            position: i,
            cycles: 0,
            load: 0,
            history: HashSet::new(),
            raining: false,
        })
    }

    while !rain_particles.is_empty() {
        rain_particles.iter_mut().for_each(|p| {
            p.cycles += 1;

            if planet.landblocks[p.position].btype == BiomeType::Water {
                p.take_water(planet, 20);
            } else if p.raining {
                p.dump_water(planet, 5);
            } else {
                p.take_water(planet, 200);
            }

            if p.load < 1 {
                p.raining = false;
            }
            if p.load > 0
                && (planet.landblocks[p.position].btype == BiomeType::Mountains
                    || planet.landblocks[p.position].btype == BiomeType::Highlands)
            {
                p.raining = true;
            }

            let wind = planet.landblocks[p.position].prevailing_wind;
            if wind != Direction::None {
                let destination = match wind {
                    Direction::North => planet.landblocks[p.position].neighbors[0].1,
                    Direction::South => planet.landblocks[p.position].neighbors[1].1,
                    Direction::East => planet.landblocks[p.position].neighbors[2].1,
                    Direction::West => planet.landblocks[p.position].neighbors[3].1,
                    Direction::None => 0,
                };

                if !p.history.contains(&destination) {
                    p.history.insert(p.position);
                    p.position = destination;
                } else {
                    p.cycles += 500;
                }
            } else {
                p.cycles += 500;
            }
        });

        rain_particles.retain(|p| p.cycles < WORLD_WIDTH as u32 * 2);
        let percent =
            ((1.0 - (rain_particles.len() as f32 / WORLD_TILES_COUNT as f32)) * 100.0) as u8;

        update_status(PlanetBuilderStatus::Rainfall { amount: percent });
    }
}
