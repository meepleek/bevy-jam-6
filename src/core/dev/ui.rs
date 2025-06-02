use bevy::ui::UiDebugOptions;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
    app.insert_resource(UiDebugOptions {
        enabled: true,
        line_width: 2.,
        // show_hidden: true,
        show_clipped: true,
        ..default()
    });
}

const TOGGLE_KEY: KeyCode = KeyCode::F1;

#[cfg_attr(feature = "native_dev", hot)]
fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
