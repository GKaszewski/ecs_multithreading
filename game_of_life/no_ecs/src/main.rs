mod game_of_life {
    use std::io::Write;

    use rand::Rng;

    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Cell {
        Dead = 0,
        Alive = 1,
    }

    #[derive(Default)]
    struct Universe {
        width: u32,
        height: u32,
        cells: Vec<Cell>,
        durations: Vec<std::time::Duration>,
    }

    fn initialize_cells(width: u32, height: u32) -> Vec<Cell> {
        // let start = std::time::Instant::now();
        let size = (width * height) as usize;
        let mut cells = Vec::with_capacity(size);
        for _ in 0..size {
            cells.push(Cell::Dead);
        }

        // let duration = start.elapsed();
        // println!("Time elapsed in initializing the cells is: {:?}", duration);

        cells
    }

    fn randomize(cells: &mut Vec<Cell>) {
        let mut rng = rand::thread_rng();
        for cell in cells.iter_mut() {
            *cell = match rng.gen_range(0..2) {
                0 => Cell::Dead,
                _ => Cell::Alive,
            };
        }
    }

    fn get_cell_by_position(cells: &Vec<Cell>, width: u32, x: u32, y: u32) -> &Cell {
        &cells[(y * width + x) as usize]
    }

    fn set_cell_by_position(cells: &mut Vec<Cell>, width: u32, x: u32, y: u32, cell: Cell) {
        cells[(y * width + x) as usize] = cell;
    }

    fn get_alive_neighbours_count(
        cells: &Vec<Cell>,
        width: u32,
        height: u32,
        x: u32,
        y: u32,
    ) -> u32 {
        // let start = std::time::Instant::now();
        let mut count = 0;
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let new_x = x as i32 + i;
                let new_y = y as i32 + j;
                if new_x < 0 || new_x >= width as i32 || new_y < 0 || new_y >= height as i32 {
                    continue;
                }
                if *get_cell_by_position(cells, width, new_x as u32, new_y as u32) == Cell::Alive {
                    count += 1;
                }
            }
        }

        // let duration = start.elapsed();
        // println!(
        //     "Time elapsed in getting the alive neighbours count is: {:?}",
        //     duration
        // );

        count
    }

    fn run_iteration(universe: &mut Universe) {
        let mut new_cells = universe.cells.clone();
        for y in 0..universe.height {
            for x in 0..universe.width {
                let cell = get_cell_by_position(&universe.cells, universe.width, x, y);
                let alive_neighbours = get_alive_neighbours_count(
                    &universe.cells,
                    universe.width,
                    universe.height,
                    x,
                    y,
                );
                let new_cell = match (cell, alive_neighbours) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };
                set_cell_by_position(&mut new_cells, universe.width, x, y, new_cell);
            }
        }
        universe.cells = new_cells;
    }

    fn print_cells(cells: &Vec<Cell>, width: u32, height: u32) {
        for y in 0..height {
            for x in 0..width {
                let cell = get_cell_by_position(cells, width, x, y);
                print!(
                    "{}",
                    match cell {
                        Cell::Alive => "■",
                        Cell::Dead => "□",
                    }
                );
            }
            println!();
        }
    }

    fn save_cells_to_file(cells: &Vec<Cell>, width: u32, height: u32, filename: &str) {
        let mut file = std::fs::File::create(filename).expect("Unable to create file");
        for y in 0..height {
            for x in 0..width {
                let cell = get_cell_by_position(cells, width, x, y);
                let cell_str = match cell {
                    Cell::Alive => "1",
                    Cell::Dead => "0",
                };
                file.write_all(cell_str.as_bytes())
                    .expect("Unable to write data");
            }
            file.write_all(b"\n").expect("Unable to write data");
        }
    }

    fn save_durations_to_file(durations: &Vec<std::time::Duration>, filename: &str) {
        let mut file = std::fs::File::create(filename).expect("Unable to create file");
        for duration in durations.iter() {
            let duration_str = format!("{:?}", duration);
            file.write_all(duration_str.as_bytes())
                .expect("Unable to write data");
            file.write_all(b"\n").expect("Unable to write data");
        }
    }

    pub fn run_simulation(width: u32, height: u32, iterations: u32, should_print_cells: bool) {
        let mut universe = Universe {
            width,
            height,
            cells: initialize_cells(width, height),
            ..Default::default()
        };
        randomize(&mut universe.cells);
        for i in 0..iterations {
            let start = std::time::Instant::now();
            if should_print_cells {
                println!("Iteration {}", i);
                print_cells(&universe.cells, width, height);
                save_cells_to_file(&universe.cells, width, height, "cells.txt");
            }
            run_iteration(&mut universe);
            let duration = start.elapsed();
            universe.durations.push(duration);
            //println!("Time elapsed in running the iteration is: {:?}", duration);
        }

        //save_durations_to_file(&universe.durations, "durations.txt");
    }

    mod tests {
        #[test]
        fn test_initialize_cells() {
            let width = 3;
            let height = 3;
            let cells = super::initialize_cells(width, height);
            assert_eq!(cells.len(), (width * height) as usize);
            for cell in cells.iter() {
                assert_eq!(*cell, super::Cell::Dead);
            }
        }

        #[test]
        fn test_block_pattern() {
            let width = 4;
            let height = 4;
            let cells = super::initialize_cells(width, height);
            let mut universe = super::Universe {
                width,
                height,
                cells,
                ..Default::default()
            };
            super::set_cell_by_position(&mut universe.cells, width, 1, 1, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 1, 2, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 2, 1, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 2, 2, super::Cell::Alive);

            let expected_cells = vec![
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Alive,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Alive,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
            ];

            for i in 0..(width * height) as usize {
                assert_eq!(universe.cells[i], expected_cells[i]);
            }

            super::run_iteration(&mut universe);

            for i in 0..(width * height) as usize {
                assert_eq!(universe.cells[i], expected_cells[i]);
            }
        }

        #[test]
        fn test_beehive_pattern() {
            let width = 6;
            let height = 5;
            let cells = super::initialize_cells(width, height);
            let mut universe = super::Universe {
                width,
                height,
                cells,
                ..Default::default()
            };

            super::set_cell_by_position(&mut universe.cells, width, 1, 2, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 2, 1, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 2, 3, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 3, 1, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 3, 3, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 4, 2, super::Cell::Alive);

            let expected_cells = vec![
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Alive,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Alive,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
            ];

            for i in 0..(width * height) as usize {
                assert_eq!(universe.cells[i], expected_cells[i]);
            }

            super::run_iteration(&mut universe);

            for i in 0..(width * height) as usize {
                assert_eq!(universe.cells[i], expected_cells[i]);
            }
        }

        #[test]
        fn test_blinker_pattern() {
            let width = 5;
            let height = 5;
            let cells = super::initialize_cells(width, height);
            let mut universe = super::Universe {
                width,
                height,
                cells,
                ..Default::default()
            };

            super::set_cell_by_position(&mut universe.cells, width, 2, 1, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 2, 2, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 2, 3, super::Cell::Alive);

            let expected_cells = vec![
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                // first row
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Dead,
                super::Cell::Dead,
                // second row
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Dead,
                super::Cell::Dead,
                // third row
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Dead,
                super::Cell::Dead,
                // fourth row
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                // fifth row
            ];

            for i in 0..(width * height) as usize {
                assert_eq!(universe.cells[i], expected_cells[i]);
            }

            super::run_iteration(&mut universe);

            let expected_cells = vec![
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                // first row
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                // second row
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Alive,
                super::Cell::Alive,
                super::Cell::Dead,
                // third row
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                // fourth row
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                // fifth row
            ];

            for i in 0..(width * height) as usize {
                assert_eq!(universe.cells[i], expected_cells[i]);
            }
        }

        #[test]
        fn test_toad_pattern() {
            let width = 4;
            let height = 4;
            let cells = super::initialize_cells(width, height);
            let mut universe = super::Universe {
                width,
                height,
                cells,
                ..Default::default()
            };

            super::set_cell_by_position(&mut universe.cells, width, 1, 0, super::Cell::Alive);

            super::set_cell_by_position(&mut universe.cells, width, 1, 1, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 2, 1, super::Cell::Alive);

            super::set_cell_by_position(&mut universe.cells, width, 1, 2, super::Cell::Alive);
            super::set_cell_by_position(&mut universe.cells, width, 2, 2, super::Cell::Alive);

            super::set_cell_by_position(&mut universe.cells, width, 2, 3, super::Cell::Alive);

            let expected_cells = vec![
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Dead,
                super::Cell::Dead,
                // first row
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Alive,
                super::Cell::Dead,
                // second row
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Alive,
                super::Cell::Dead,
                // third row
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Dead,
                // fourth row
            ];

            for i in 0..(width * height) as usize {
                assert_eq!(universe.cells[i], expected_cells[i]);
            }

            super::run_iteration(&mut universe);

            let expected_cells = vec![
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Alive,
                super::Cell::Dead,
                // first row
                super::Cell::Alive,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                // second row
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Dead,
                super::Cell::Alive,
                // third row
                super::Cell::Dead,
                super::Cell::Alive,
                super::Cell::Alive,
                super::Cell::Dead,
                // fourth row
            ];

            for i in 0..(width * height) as usize {
                assert_eq!(universe.cells[i], expected_cells[i]);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        println!("Usage: {} <width> <height> <iterations>", args[0]);
        std::process::exit(1);
    }

    let width: u32 = args[1].parse().expect("Invalid width");
    let height: u32 = args[2].parse().expect("Invalid height");
    let iterations: u32 = args[3].parse().expect("Invalid iterations");

    let start = std::time::Instant::now();
    game_of_life::run_simulation(width, height, iterations, false);
    let duration = start.elapsed();
    println!(
        "Time elapsed in running the simulation ({} iterations, {} cells) is: {:?}",
        iterations,
        width * height,
        duration
    );
}
