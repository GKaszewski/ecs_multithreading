mod plugin;

fn main() {
    // plugin::run_simulation();
    let args: Vec<String> = std::env::args().collect();
    let iterations = args[1].parse::<usize>().unwrap();
    let size = args[2].parse::<usize>().unwrap();
    plugin::run_simulation_n_times(iterations, size);
}
