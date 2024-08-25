use bevy::prelude::*;
use plugin::EnzymeSubstrateReactionPlugin;

mod plugin;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(EnzymeSubstrateReactionPlugin)
        .run();
}
