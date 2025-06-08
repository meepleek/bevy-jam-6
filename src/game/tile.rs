use crate::prelude::tween::DespawnOnTweenCompleted;
use crate::prelude::tween::get_relative_scale_anim;
use crate::prelude::tween::get_relative_sprite_color_anim;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(hide_tile_highlighs_on_card_deselected);
}

#[derive(Debug, Clone, PartialEq)]
pub struct TileEntity {
    pub entity: Entity,
    pub kind: TileEntityKind,
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum TileEntityKind {
    Player,
    Enemy,
    #[allow(dead_code)]
    Wall,
}

#[derive(Component)]
pub struct TileInteraction;

fn hide_tile_highlighs_on_card_deselected(
    _trig: Trigger<OnRemove, SelectedTileTriggerCard>,
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
