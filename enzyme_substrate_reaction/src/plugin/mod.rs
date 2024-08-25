use std::{ops::Sub, time::Instant};

use bevy::prelude::*;
use rand::Rng;

pub struct EnzymeSubstrateReactionPlugin;

impl Plugin for EnzymeSubstrateReactionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationLogFlag>()
            // .add_systems(Startup, setup_system)
            .add_systems(Startup, huge_scene_setup)
            .add_systems(
                Update,
                (
                    binding_system,
                    reaction_system,
                    release_system,
                    check_if_substrates_are_consumed,
                ),
            );
    }
}

#[derive(Resource)]
struct SimulationLogFlag(pub bool);

impl Default for SimulationLogFlag {
    fn default() -> Self {
        SimulationLogFlag(false)
    }
}

// Component representing the concentration of a molecule
#[derive(Component)]
struct Concentration(f32);

// Component representing the reaction rate of an enzyme
#[derive(Component)]
struct ReactionRate(f32);

// Component indicating whether the enzyme's active site is free or occupied (free = true, occupied = false)
#[derive(Component)]
struct ActiveSite(bool);

#[derive(Component)]
struct MichaelisConstant(f32); // Represents Km

// Component representing the product formed after the reaction
#[derive(Component)]
struct Product;

#[derive(Component)]
struct Substrate;

#[derive(Component)]
struct Enzyme;

fn setup_system(mut commands: Commands) {
    commands.spawn((
        Concentration(1.0),
        ReactionRate(0.1),
        ActiveSite(true),
        MichaelisConstant(0.5),
        Enzyme,
    )); // Enzyme
    commands.spawn((Concentration(5.0), Substrate)); // Substrate
    commands.spawn((Concentration(0.0), Product)); // Product
}

fn huge_scene_setup(mut commands: Commands) {
    let num_enzymes = 200_000;
    let num_substrates = 200_000;

    let mut rng = rand::thread_rng();

    for _ in 0..num_enzymes {
        commands.spawn((
            // ReactionRate(rng.gen_range(0.1..0.5)),
            ReactionRate(0.5),
            Concentration(5.0),
            ActiveSite(true),
            MichaelisConstant(0.5),
            Enzyme,
        ));
    }

    for _ in 0..num_substrates {
        commands.spawn((Concentration(5.0), Substrate));
    }

    commands.spawn((Concentration(0.0), Product));

    println!(
        "Start: {:?}",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_millis()
    );
}

fn binding_system(
    mut enzyme_query: Query<(&mut ActiveSite, &Concentration, &Enzyme)>,
    substrate_query: Query<(&Concentration, &Substrate)>,
    log_flag: Res<SimulationLogFlag>,
) {
    for (mut active_site, enzyme_concentration, _) in enzyme_query.iter_mut() {
        if active_site.0 {
            for (substrate_concentration, _) in substrate_query.iter() {
                if substrate_concentration.0 > 0.1 * enzyme_concentration.0 {
                    active_site.0 = false; // Bind the substrate to the enzyme
                    if log_flag.0 {
                        println!("Enzyme active site occupied!");
                    }
                    break;
                }
            }
        }
    }
}

fn reaction_system(
    enzyme_query: Query<(&ActiveSite, &ReactionRate, &MichaelisConstant), With<Enzyme>>,
    mut substrate_query: Query<(Entity, &mut Concentration), (Without<Product>, With<Substrate>)>,
    mut product_query: Query<(Entity, &mut Concentration), With<Product>>,
    log_flag: Res<SimulationLogFlag>,
) {
    for (active_site, reaction_rate, km) in enzyme_query.iter() {
        if !active_site.0 {
            continue; // Only proceed if the active site is occupied
        }

        for (substrate_entity, mut substrate_concentration) in substrate_query.iter_mut() {
            if substrate_concentration.0 > 0.0 {
                let rate = reaction_rate.0 * substrate_concentration.0
                    / (km.0 + substrate_concentration.0); // Michaelis-Menten kinetics equation v = Vmax * [S] / (Km + [S])
                substrate_concentration.0 -= rate; // Consume the substrate
                if log_flag.0 {
                    println!(
                        "Substrate {} concentration: {}",
                        substrate_entity, substrate_concentration.0
                    );
                }

                for (product_entity, mut product_concentration) in product_query.iter_mut() {
                    product_concentration.0 += rate; // Produce the product
                    if log_flag.0 {
                        println!(
                            "Product {} concentration: {}",
                            product_entity, product_concentration.0
                        );
                    }
                }
            }
        }
    }
}

fn release_system(mut enzyme_query: Query<&mut ActiveSite>, log_flag: Res<SimulationLogFlag>) {
    for mut active_site in enzyme_query.iter_mut() {
        if !active_site.0 {
            active_site.0 = true; // Release the product from the enzyme (i.e. free the active site)
            if log_flag.0 {
                println!("Enzyme active site released!");
            }
        }
    }
}

fn check_if_substrates_are_consumed(substrate_query: Query<&Concentration, With<Substrate>>) {
    let mut substrates_concentration_sum = 0.0;
    for substrate_concentration in substrate_query.iter() {
        substrates_concentration_sum += substrate_concentration.0;
    }

    if substrates_concentration_sum <= 0.0 {
        // println!("All substrates are consumed!");
        println!(
            "End: {:?}",
            std::time::UNIX_EPOCH.elapsed().unwrap().as_millis()
        );
        std::process::exit(0);
    }
}

mod tests {
    use super::*;

    fn setup_test_app() -> App {
        let mut app = App::new();
        app.insert_resource(SimulationLogFlag(false))
            .add_plugins(MinimalPlugins);
        app
    }

    #[test]
    fn test_binding_system() {
        let mut app = setup_test_app();
        app.add_systems(Update, binding_system);

        app.world_mut().spawn((
            ReactionRate(0.1),
            ActiveSite(true),
            MichaelisConstant(0.5),
            Concentration(5.0),
            Enzyme,
        ));
        app.world_mut().spawn((Concentration(5.0), Substrate));

        app.update();

        let mut query = app.world_mut().query::<&ActiveSite>();
        for active_site in query.iter(&app.world()) {
            assert_eq!(active_site.0, false);
        }

        app.world_mut().clear_entities();

        app.world_mut().spawn((
            ReactionRate(0.1),
            ActiveSite(true),
            MichaelisConstant(0.5),
            Concentration(25.0),
            Enzyme,
        ));
        app.world_mut().spawn((Concentration(1.0), Substrate));

        app.update();

        let mut query = app.world_mut().query::<&ActiveSite>();
        for active_site in query.iter(&app.world()) {
            assert_eq!(active_site.0, true);
        }
    }

    #[test]
    fn test_reaction_system() {
        let mut app = setup_test_app();
        app.add_systems(Update, reaction_system);

        app.world_mut().spawn((
            ReactionRate(0.1),
            ActiveSite(true),
            MichaelisConstant(0.5),
            Concentration(5.0),
            Enzyme,
        ));

        let substrate_entity = app.world_mut().spawn((Concentration(5.0), Substrate)).id();

        let product_entity = app.world_mut().spawn((Concentration(0.0), Product)).id();

        app.update();

        let substrate_concentration = app.world().get::<Concentration>(substrate_entity).unwrap();
        assert_eq!(substrate_concentration.0, 4.909091);

        let product_concentration = app.world().get::<Concentration>(product_entity).unwrap();
        assert_eq!(product_concentration.0, 0.09090909);
    }

    #[test]
    fn test_release_system() {
        let mut app = setup_test_app();
        app.add_systems(Update, release_system);

        let enzyme_entity = app
            .world_mut()
            .spawn((ReactionRate(0.1), ActiveSite(false)))
            .id();

        app.update();

        let active_site = app.world().get::<ActiveSite>(enzyme_entity).unwrap();
        assert_eq!(active_site.0, true);
    }
}
