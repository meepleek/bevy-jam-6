use crate::game::board::Grid;
use crate::game::card::CardEffect;
use crate::game::card::card;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Gameplay.on_enter(spawn_gameplay_screen));

    app.configure::<GameplayAction>();
}

fn spawn_gameplay_screen(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    cmd.spawn((Grid::new(9, 9), Transform::from_xyz(300., 0., 0.)));

    let card_hover_mesh = meshes.add(Rectangle::new(190., 570.));
    for i in -2..=2 {
        cmd.spawn(card(
            CardEffect::Move(1),
            Vec3::new(
                i as f32 * 150.,
                -290. - (i as f32).abs() * 25.,
                i as f32 / 5. + 1.,
            ),
            Rot2::degrees(i as f32 * -10.),
            card_hover_mesh.clone(),
        ));
    }
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameplayAction {
    Pause,
    CloseMenu,
}

impl Configure for GameplayAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::Pause, GamepadButton::Start)
                .with(Self::Pause, KeyCode::Escape)
                .with(Self::Pause, KeyCode::KeyP)
                .with(Self::CloseMenu, KeyCode::KeyP),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            Screen::Gameplay.on_update((
                (spawn_pause_overlay, Menu::Pause.enter())
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_disabled.and(action_just_pressed(Self::Pause))),
                Menu::clear
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_enabled.and(action_just_pressed(Self::CloseMenu))),
            )),
        );
    }
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        widget::blocking_overlay(1),
        ThemeColor::Overlay.set::<BackgroundColor>(),
        DespawnOnExitState::<Screen>::default(),
        DespawnOnDisableState::<Menu>::default(),
    ));
}
