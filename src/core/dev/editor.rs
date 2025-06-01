use bevy_editor_pls::EditorPlugin;
use bevy_editor_pls::EditorWindowPlacement;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    let mut window = Window {
        title: "bevy_editor_pls".to_string(),
        focused: false,
        ..default()
    };
    window.set_minimized(true);

    let window = app
        .world_mut()
        .spawn((Name::new("EditorWindow"), window, IsEditorWindow))
        .id();

    app.add_plugins(EditorPlugin {
        window: EditorWindowPlacement::Window(window),
    });
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsEditorWindow;
