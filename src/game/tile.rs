use crate::game::card_effect::PlaySelectedTileCard;
use crate::prelude::tween::DespawnOnTweenCompleted;
use crate::prelude::tween::get_absolute_scale_anim;
use crate::prelude::tween::get_relative_scale_anim;
use crate::prelude::tween::get_relative_sprite_color_anim;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(highlight_tiles_on_card_selected)
        .add_observer(hide_tile_highlighs_on_card_deselected);
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileEntity {
    Player,
}

#[derive(Component, Deref, DerefMut, Clone, Default)]
pub struct TileCoords(pub Coords);

#[derive(Component)]
pub struct TileInteraction;

fn hide_tile_highlighs_on_card_deselected(
    _trig: Trigger<OnRemove, CardSelected>,
    mut cmd: Commands,
    interaction_tile_q: Query<Entity, With<TileInteraction>>,
) {
    for e in &interaction_tile_q {
        or_continue!(cmd.get_entity(e)).try_insert((
            get_relative_sprite_color_anim(Color::NONE, 150, None),
            get_relative_scale_anim(Vec2::splat(0.1), 150, None),
            DespawnOnTweenCompleted::Itself,
        ));
    }
}

fn highlight_tiles_on_card_selected(
    trig: Trigger<OnAdd, CardSelected>,
    card_q: Query<&Card>,
    mut cmd: Commands,
    grid: Single<&Grid>,
    player_tile: Single<&TileCoords, With<Player>>,
) {
    let card = or_return!(card_q.get(trig.target()));
    let interaction_palette = or_return_quiet!(card.action.tile_interaction_palette());
    let player_tile = player_tile.0;
    let effect_tiles = or_return_quiet!(card.action.effect_tiles());
    for (tile, position) in effect_tiles
        .into_iter()
        .map(|tile| player_tile + tile)
        .filter_map(|tile| grid.tile_to_world(tile).and_then(|pos| Some((tile, pos))))
    {
        cmd.spawn((
            Transform::from_translation(position.extend(0.)),
            Sprite::from_color(Color::NONE, Vec2::splat(60.)),
            get_relative_sprite_color_anim(interaction_palette.highlight, 150, None),
            get_absolute_scale_anim(Vec3::splat(0.5), Vec2::ONE, 180, None),
            TileInteraction,
            TileCoords(tile),
            Pickable {
                should_block_lower: false,
                is_hoverable: true,
            },
        ))
        .observe(tween::tween_sprite_color_on_trigger::<Pointer<Over>, ()>(
            interaction_palette.hover,
        ))
        .observe(tween::tween_sprite_color_on_trigger::<Pointer<Out>, ()>(
            interaction_palette.highlight,
        ))
        .observe(move |_trig: Trigger<Pointer<Click>>, mut cmd: Commands| {
            cmd.trigger(PlaySelectedTileCard(tile));
        });
    }
}
