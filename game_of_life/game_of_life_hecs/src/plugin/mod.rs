use hecs::*;
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

// is alive or dead
#[derive(Clone, PartialEq, Eq)]
struct State(bool);

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.0 {
            write!(f, "Alive")
        } else {
            write!(f, "Dead")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Neighbors(u8);

fn batch_spawn_cells(world: &mut World, n: usize) {
    let mut rng = thread_rng();
    let cells_to_spawn_count = n * n; // square grid
    let to_spawn = (0..cells_to_spawn_count).map(|i| {
        let x = i % n;
        let y = i / n;
        let position = Position {
            x: x as i32,
            y: y as i32,
        };
        let state = State(rng.gen_bool(0.5)); // 50% chance of being alive
        let neighbors = Neighbors(0);

        (position, state, neighbors)
    });

    world.spawn_batch(to_spawn);
}

fn update_neighbors_system(world: &mut World) {
    let neighbors_count: Vec<(Entity, usize)> = world
        .query::<&Position>()
        .iter()
        .map(|(entity, position)| {
            let mut count = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 {
                        continue; // skip the cell itself
                    }

                    let neighbor_position = Position {
                        x: position.x + x,
                        y: position.y + y,
                    };

                    let mut entities_checked: Vec<Entity> = vec![];
                    // check if neighbor is alive, if so, increment count
                    if let Some((e, (_, _))) = world
                        .query::<(&State, &Position)>()
                        .iter()
                        .find(|(_, (state, pos))| *pos == &neighbor_position && state.0)
                    {
                        if entities_checked.contains(&e) {
                            continue;
                        }

                        count += 1;
                        entities_checked.push(e);
                    }
                }
            }
            (entity, count)
        })
        .collect();

    for (entity, count) in neighbors_count {
        if let Ok(mut neighbors) = world.get::<&mut Neighbors>(entity) {
            neighbors.0 = count as u8;
        }
    }
}

fn update_cells_system(world: &mut World) {
    let entites_to_update: Vec<(Entity, bool)> = world
        .query::<(&Neighbors, &State)>()
        .iter()
        .map(|(entity, (neighbors, state))| {
            // apply the rules of the game of life
            // if cell is alive and has 2 or 3 neighbors, it stays alive
            // if cell is dead and has 3 neighbors, it becomes alive
            let new_state = match (state.0, neighbors.0) {
                (true, 2) | (_, 3) => true,
                _ => false,
            };

            (entity, new_state)
        })
        .collect();

    for (entity, new_state) in entites_to_update {
        if let Ok(mut state) = world.get::<&mut State>(entity) {
            state.0 = new_state;
        }
    }
}

// pub fn run_simulation() {
//     let mut world = World::new();
//     batch_spawn_cells(&mut world, 10);

//     loop {
//         let start_loop = std::time::Instant::now();
//         update_neighbors_system(&mut world);
//         update_cells_system(&mut world);
//         update_neighbors_system(&mut world);
//         println!("Loop took {:?}", start_loop.elapsed());

//         std::thread::sleep(std::time::Duration::from_secs(1));
//     }
// }

pub fn run_simulation_n_times(n: usize, size: usize) {
    let mut world = World::new();
    batch_spawn_cells(&mut world, size);
    let start_sim = std::time::Instant::now();
    for _ in 0..n {
        // let start_loop = std::time::Instant::now();
        update_neighbors_system(&mut world);
        update_cells_system(&mut world);
        update_neighbors_system(&mut world);
        // println!("Loop took {:?}", start_loop.elapsed());

        // std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!(
        "Simulation took ({} iterations, {} cells) {:?}",
        n,
        size * size,
        start_sim.elapsed()
    );
}

#[cfg(test)]
mod tests {
    pub use super::*;

    fn spawn_block_pattern(world: &mut World) {
        let to_spawn = vec![
            (Position { x: 0, y: 0 }, State(true), Neighbors(0)),
            (Position { x: 0, y: 1 }, State(true), Neighbors(0)),
            (Position { x: 1, y: 0 }, State(true), Neighbors(0)),
            (Position { x: 1, y: 1 }, State(true), Neighbors(0)),
        ];

        world.spawn_batch(to_spawn);
    }

    fn spawn_blinker_pattern(world: &mut World) {
        let to_spawn = vec![
            (Position { x: 0, y: 0 }, State(false), Neighbors(0)),
            (Position { x: 1, y: 0 }, State(true), Neighbors(0)),
            (Position { x: 2, y: 0 }, State(false), Neighbors(0)),
            (Position { x: 0, y: 1 }, State(false), Neighbors(0)),
            (Position { x: 1, y: 1 }, State(true), Neighbors(0)),
            (Position { x: 2, y: 1 }, State(false), Neighbors(0)),
            (Position { x: 0, y: 2 }, State(false), Neighbors(0)),
            (Position { x: 1, y: 2 }, State(true), Neighbors(0)),
            (Position { x: 2, y: 2 }, State(false), Neighbors(0)),
        ];

        world.spawn_batch(to_spawn);
    }

    fn spawn_toad_pattern(world: &mut World) {
        let to_spawn = vec![
            (Position { x: 0, y: 0 }, State(true), Neighbors(0)),
            (Position { x: 0, y: 1 }, State(true), Neighbors(0)),
            (Position { x: 0, y: 2 }, State(true), Neighbors(0)),
            (Position { x: 1, y: 1 }, State(true), Neighbors(0)),
            (Position { x: 1, y: 2 }, State(true), Neighbors(0)),
            (Position { x: 1, y: 3 }, State(true), Neighbors(0)),
        ];

        world.spawn_batch(to_spawn);
    }

    #[test]
    fn test_block_pattern() {
        let mut world = World::new();
        spawn_block_pattern(&mut world);

        update_neighbors_system(&mut world);
        update_cells_system(&mut world);

        let expected = vec![
            (Position { x: 0, y: 0 }, State(true), Neighbors(3)),
            (Position { x: 0, y: 1 }, State(true), Neighbors(3)),
            (Position { x: 1, y: 0 }, State(true), Neighbors(3)),
            (Position { x: 1, y: 1 }, State(true), Neighbors(3)),
        ];

        let actual: Vec<(Position, State, Neighbors)> = world
            .query::<(&Position, &State, &Neighbors)>()
            .iter()
            .map(|(_, (p, s, n))| (p.clone(), s.clone(), n.clone()))
            .collect();

        assert_eq!(expected, actual);

        update_neighbors_system(&mut world);
        update_cells_system(&mut world);

        let actual = world
            .query::<(&Position, &State, &Neighbors)>()
            .iter()
            .map(|(_, (p, s, n))| (p.clone(), s.clone(), n.clone()))
            .collect::<Vec<(Position, State, Neighbors)>>();

        assert_eq!(expected, actual);
    }

    fn print_world_state(world: &World) {
        let state: Vec<(Position, State, Neighbors)> = world
            .query::<(&Position, &State, &Neighbors)>()
            .iter()
            .map(|(_, (p, s, n))| (p.clone(), s.clone(), n.clone()))
            .collect();

        println!("{:?}", state);
    }

    #[test]
    fn test_toad_pattern() {
        let mut world = World::new();
        spawn_toad_pattern(&mut world);

        let expected = vec![
            (Position { x: 0, y: 0 }, State(true), Neighbors(0)),
            (Position { x: 0, y: 1 }, State(true), Neighbors(0)),
            (Position { x: 0, y: 2 }, State(true), Neighbors(0)),
            (Position { x: 1, y: 1 }, State(true), Neighbors(0)),
            (Position { x: 1, y: 2 }, State(true), Neighbors(0)),
            (Position { x: 1, y: 3 }, State(true), Neighbors(0)),
        ];

        let actual: Vec<(Position, State, Neighbors)> = world
            .query::<(&Position, &State, &Neighbors)>()
            .iter()
            .map(|(_, (p, s, n))| (p.clone(), s.clone(), n.clone()))
            .collect();

        assert_eq!(expected, actual);

        println!("Initial state:");
        print_world_state(&world);

        update_neighbors_system(&mut world);

        println!("After updating neighbors:");
        print_world_state(&world);

        let expected = vec![
            (Position { x: 0, y: 0 }, State(true), Neighbors(1)),
            (Position { x: 0, y: 1 }, State(true), Neighbors(3)),
            (Position { x: 0, y: 2 }, State(true), Neighbors(2)),
            (Position { x: 1, y: 1 }, State(true), Neighbors(2)),
            (Position { x: 1, y: 2 }, State(true), Neighbors(3)),
            (Position { x: 1, y: 3 }, State(true), Neighbors(1)),
        ];

        let actual: Vec<(Position, State, Neighbors)> = world
            .query::<(&Position, &State, &Neighbors)>()
            .iter()
            .map(|(_, (p, s, n))| (p.clone(), s.clone(), n.clone()))
            .collect();

        assert_eq!(expected, actual);

        update_cells_system(&mut world);
    }

    #[test]
    fn test_blinker_pattern() {
        let mut world = World::new();
        spawn_blinker_pattern(&mut world);

        let expected = vec![
            (Position { x: 0, y: 0 }, State(false), Neighbors(0)),
            (Position { x: 1, y: 0 }, State(true), Neighbors(0)),
            (Position { x: 2, y: 0 }, State(false), Neighbors(0)),
            (Position { x: 0, y: 1 }, State(false), Neighbors(0)),
            (Position { x: 1, y: 1 }, State(true), Neighbors(0)),
            (Position { x: 2, y: 1 }, State(false), Neighbors(0)),
            (Position { x: 0, y: 2 }, State(false), Neighbors(0)),
            (Position { x: 1, y: 2 }, State(true), Neighbors(0)),
            (Position { x: 2, y: 2 }, State(false), Neighbors(0)),
        ];

        let actual: Vec<(Position, State, Neighbors)> = world
            .query::<(&Position, &State, &Neighbors)>()
            .iter()
            .map(|(_, (p, s, n))| (p.clone(), s.clone(), n.clone()))
            .collect();

        assert_eq!(expected, actual);

        update_neighbors_system(&mut world);
        update_cells_system(&mut world);
        update_neighbors_system(&mut world);

        let expected = vec![
            (Position { x: 0, y: 0 }, State(false), Neighbors(2)),
            (Position { x: 1, y: 0 }, State(false), Neighbors(3)),
            (Position { x: 2, y: 0 }, State(false), Neighbors(2)),
            (Position { x: 0, y: 1 }, State(true), Neighbors(1)),
            (Position { x: 1, y: 1 }, State(true), Neighbors(2)),
            (Position { x: 2, y: 1 }, State(true), Neighbors(1)),
            (Position { x: 0, y: 2 }, State(false), Neighbors(2)),
            (Position { x: 1, y: 2 }, State(false), Neighbors(3)),
            (Position { x: 2, y: 2 }, State(false), Neighbors(2)),
        ];

        let actual: Vec<(Position, State, Neighbors)> = world
            .query::<(&Position, &State, &Neighbors)>()
            .iter()
            .map(|(_, (p, s, n))| (p.clone(), s.clone(), n.clone()))
            .collect();

        assert_eq!(expected, actual);
    }
}
