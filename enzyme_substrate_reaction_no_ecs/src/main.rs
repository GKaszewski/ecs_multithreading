use std::time::Instant;

mod simulation;

fn main() {
    let mut enzymes = simulation::generate_random_enzymes(20_000);
    let mut substrates = simulation::generate_random_molecules(20_000);
    let mut product = simulation::Molecule::new(0.0);
    let steps = 1;
    let start = Instant::now();
    simulation::simulate(steps, &mut enzymes, &mut substrates, &mut product, false);
    let duration = start.elapsed();
    println!(
        "Time elapsed in simulation ({} steps) is: {:?}",
        steps, duration
    );
}