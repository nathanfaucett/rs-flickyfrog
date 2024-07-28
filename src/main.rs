use bevy::prelude::*;

use flickyfrog::app::AppPlugin;

#[bevy_main]
fn main() {
    App::new().add_plugins(AppPlugin).run();
}
