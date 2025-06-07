use crate::game::card;
use crate::game::card_effect::*;
use crate::game::die;
use crate::game::grid::Grid;
use crate::game::tile::TileCoords;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Gameplay.on_enter(spawn_level));
}

#[cfg_attr(feature = "native_dev", hot(rerun_on_hot_patch))]
fn spawn_level(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    cmd.spawn((Grid::new(9, 9), Transform::from_xyz(0., 0., 0.)));

    let card_hover_mesh = meshes.add(Rectangle::new(190., 570.));
    for (i, action) in [
        CardAction::Move {
            reach: EffectReach::Exact(1),
            direction: EffectDirection::Orthogonal,
            pip_cost: 1,
        },
        CardAction::Move {
            reach: EffectReach::Exact(1),
            direction: EffectDirection::Diagonal,
            pip_cost: 1,
        },
        CardAction::Move {
            reach: EffectReach::Exact(2),
            direction: EffectDirection::Orthogonal,
            pip_cost: 1,
        },
        CardAction::Move {
            reach: EffectReach::Range(3),
            direction: EffectDirection::Diagonal,
            pip_cost: 1,
        },
        CardAction::Move {
            reach: EffectReach::Range(2),
            direction: EffectDirection::Orthogonal,
            pip_cost: 1,
        },
        CardAction::Move {
            reach: EffectReach::Exact(3),
            direction: EffectDirection::Area,
            pip_cost: 1,
        },
        CardAction::Move {
            reach: EffectReach::Range(3),
            direction: EffectDirection::Area,
            pip_cost: 1,
        },
    ]
    .into_iter()
    .enumerate()
    {
        let i = i as f32 - 3.0;
        cmd.spawn(card::card(
            action,
            Vec3::new(i as f32 * 150., -290. - i.abs() * 25., i as f32 / 5. + 1.),
            Rot2::degrees(i as f32 * -10.),
            card_hover_mesh.clone(),
        ));
    }

    cmd.spawn((die::die(ROSE_300, 5), Player, TileCoords((4, 4).into())));
}
