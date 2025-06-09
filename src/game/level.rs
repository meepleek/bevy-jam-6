use bevy::color::palettes::css::CRIMSON;

use crate::game::card;
use crate::game::card_effect::*;
use crate::game::die;
use crate::game::die::Die;
use crate::game::die::DieKind;
use crate::game::grid::Grid;
use crate::game::pile::DrawPileCard;
use crate::game::pile::Piles;
use crate::game::pile::draw_pile_card_pos_rot;
use crate::game::tile::TileEntityKind;
use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Gameplay.on_enter(spawn_level));
}

#[cfg_attr(feature = "native_dev", hot(rerun_on_hot_patch))]
fn spawn_level(mut cmd: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    use CardAction::*;
    use TileCardAction::*;

    let mut rng = thread_rng();
    let piles_e = cmd.spawn((Name::new("Piles"), Piles)).id();
    let card_hover_mesh = meshes.add(Rectangle::new(230., 570.));
    for (i, action) in [
        CardActionTrigger::TileSelection(Move {
            reach: EffectReach::Exact(1),
            direction: EffectDirection::Orthogonal,
            pip_cost: 1,
        }),
        CardActionTrigger::TileSelection(Move {
            reach: EffectReach::Exact(1),
            direction: EffectDirection::Orthogonal,
            pip_cost: 1,
        }),
        CardActionTrigger::TileSelection(Move {
            reach: EffectReach::Exact(1),
            direction: EffectDirection::Orthogonal,
            pip_cost: 1,
        }),
        CardActionTrigger::TileSelection(Move {
            reach: EffectReach::Exact(1),
            direction: EffectDirection::Orthogonal,
            pip_cost: 1,
        }),
        CardActionTrigger::TileSelection(Move {
            reach: EffectReach::Exact(2),
            direction: EffectDirection::Diagonal,
            pip_cost: 1,
        }),
        CardActionTrigger::TileSelection(Attack {
            reach: EffectReach::Range(2),
            direction: EffectDirection::Orthogonal,
            pip_cost: 2,
            attack: 2,
            poison: false,
        }),
        CardActionTrigger::CardSelection(RerollSelf),
        CardActionTrigger::TileSelection(Attack {
            reach: EffectReach::Range(1),
            direction: EffectDirection::Orthogonal,
            pip_cost: 2,
            attack: 3,
            poison: false,
        }),
        CardActionTrigger::TileSelection(Move {
            reach: EffectReach::Exact(2),
            direction: EffectDirection::Orthogonal,
            pip_cost: 1,
        }),
        CardActionTrigger::CardSelection(HealSelf(2)),
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

    let grid = Grid::new(9, 7);
    cmd.spawn((
        die::die(
            BLUE_400,
            Die {
                kind: DieKind::D6,
                pip_count: 5,
            },
            TileEntityKind::Player,
        ),
        Player,
    ));

    for (x, y) in [(6, 1), (5, 3), (1, 1)] {
        cmd.spawn((
            die::die(
                CRIMSON,
                Die {
                    kind: DieKind::D6,
                    pip_count: rng.gen_range(1..=3),
                },
                TileEntityKind::Enemy,
            ),
            Transform::from_translation(grid.tile_to_world(Coords::new(x, y)).unwrap().extend(0.)),
        ));
    }

    cmd.spawn((
        Name::new("grid"),
        Transform::from_translation(Vec3::NEG_Z),
        Visibility::default(),
    ))
    .with_children(|b| {
        let size = grid.grid_size();
        for tile in
            (0..size.y).flat_map(|y| (0..size.x).map(move |x| Coords::new(x as i16, y as i16)))
        {
            b.spawn((
                Name::new("grid_tile"),
                Transform::from_translation(grid.tile_to_world(tile).unwrap().extend(0.)),
                Sprite::from_color(GRAY_300, Vec2::splat(TILE_SIZE as f32 - 6.)),
            ));
        }
    });

    cmd.spawn(grid);
}
