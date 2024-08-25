use std::time::Instant;

mod simulation;

fn main() {
    let mut enzymes = simulation::generate_random_enzymes(200_000);
    let mut substrates = simulation::generate_random_molecules(200_000);
    let mut product = simulation::Molecule::new(0.0);
    let steps = 1_000_000;
    simulation::simulate(steps, &mut enzymes, &mut substrates, &mut product, false);
    // println!(
    //     "Time elapsed in simulation ({} steps) is: {:?}",
    //     steps, duration
    // );
}
