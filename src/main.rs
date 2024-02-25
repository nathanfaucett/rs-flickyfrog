use bevy::prelude::*;

use flicky_frog::app::AppPlugin;

#[bevy_main]
fn main() {
    App::new().add_plugins(AppPlugin).run();
}
