use crate::game::card;
use crate::game::card_effect::*;
use crate::game::die;
use crate::game::die::Die;
use crate::game::die::DieKind;
use crate::game::grid::Grid;
use crate::game::pile::DrawPileCard;
use crate::game::pile::Piles;
use crate::game::pile::draw_pile_card_pos_rot;
use crate::game::tile::TileCoords;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Gameplay.on_enter(spawn_level));
}

#[cfg_attr(feature = "native_dev", hot(rerun_on_hot_patch))]
fn spawn_level(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut rng = thread_rng();

    cmd.spawn((Grid::new(9, 9), Transform::from_xyz(0., 0., 0.)));
    let piles_e = cmd.spawn((Name::new("Piles"), Piles)).id();

    let card_hover_mesh = meshes.add(Rectangle::new(230., 570.));
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
        CardAction::HealSelf(2),
    ]
    .into_iter()
    .enumerate()
    {
        let i = i as i16 - 3;
        let (pos, rot) = draw_pile_card_pos_rot(&mut rng, i);
        cmd.spawn(card::card(
            action,
            pos,
            Rot2::degrees(rot),
            card_hover_mesh.clone(),
        ))
        .insert(DrawPileCard(piles_e));
    }

    cmd.spawn((
        die::die(
            ROSE_300,
            Die {
                kind: DieKind::D6,
                pip_count: 5,
            },
        ),
        Player,
        TileCoords((4, 4).into()),
    ));
}
