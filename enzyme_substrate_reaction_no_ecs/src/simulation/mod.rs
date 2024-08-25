use rand::Rng;

pub struct Concentration(pub f32);

pub struct Enzyme {
    pub concentration: Concentration,
    pub reaction_rate: f32,
    pub active_site: bool, // free = true, bound = false
    pub michaelis_constant: f32,
}

pub struct Molecule {
    pub concentration: Concentration,
}

impl Enzyme {
    pub fn new(concentration: f32, reaction_rate: f32, michaelis_constant: f32) -> Self {
        Enzyme {
            concentration: Concentration(concentration),
            reaction_rate,
            active_site: true,
            michaelis_constant,
        }
    }

    fn bind(&mut self, substrate: &Molecule) -> bool {
        if !self.active_site && substrate.concentration.0 > 0.1 * self.concentration.0 {
            self.active_site = true;
            true
        } else {
            false
        }
    }

    fn react(&mut self, substrate: &mut Molecule) -> f32 {
        if !self.active_site || substrate.concentration.0 <= 0.0 {
            return 0.0;
        }

        let rate = self.reaction_rate * substrate.concentration.0
            / (self.michaelis_constant + substrate.concentration.0);
        substrate.concentration.0 -= rate;
        rate
    }

    fn release(&mut self) {
        if self.active_site {
            self.active_site = false;
        }
    }
}

impl Molecule {
    pub fn new(concentration: f32) -> Self {
        Molecule {
            concentration: Concentration(concentration),
        }
    }
}

pub fn simulate(
    iterations: usize,
    enzymes: &mut Vec<Enzyme>,
    substrates: &mut Vec<Molecule>,
    product: &mut Molecule,
    should_log: bool,
) {
    for _ in 0..iterations {
        // Binding phase
        for enzyme in enzymes.iter_mut() {
            for substrate in substrates.iter() {
                if enzyme.bind(substrate) {
                    if should_log {
                        println!(
                            "Enzyme bound to substrate with concentration {}",
                            substrate.concentration.0
                        );
                    }
                    break;
                }
            }
        }

        // Reaction phase
        for enzyme in enzymes.iter_mut() {
            for substrate in substrates.iter_mut() {
                let rate = enzyme.react(substrate);
                if rate > 0.0 {
                    if should_log {
                        println!("Substrate reacted: {} amount", rate);
                    }
                    product.concentration.0 += rate;
                    if should_log {
                        println!("Product concentration: {}", product.concentration.0);
                    }
                }
            }
        }

        // Release phase
        for enzyme in enzymes.iter_mut() {
            enzyme.release();
            if should_log {
                println!("Enzyme released");
            }
        }
    }
}

pub fn generate_random_molecules(n: usize) -> Vec<Molecule> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| Molecule::new(rng.gen_range(0.1..0.5)))
        .collect()
}

pub fn generate_random_enzymes(n: usize) -> Vec<Enzyme> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| {
            Enzyme::new(
                rng.gen_range(0.1..0.5),
                rng.gen_range(0.1..0.5),
                rng.gen_range(0.1..0.5),
            )
        })
        .collect()
}
