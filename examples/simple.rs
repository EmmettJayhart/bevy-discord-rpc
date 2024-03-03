use bevy::prelude::*;
use bevy_discord_presence::{ActivityState, RPCPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // Pass Discord App ID, and set show_time to true.
    app.add_plugins(RPCPlugin::new(965125975941709834, true))
        .add_systems(Update, update_presence);

    app.run();
}

fn update_presence(mut state: ResMut<ActivityState>) {
    state.instance = Some(true);
    state.details = Some("Hello World".to_string());
    state.state = Some("This is state".to_string());
}
