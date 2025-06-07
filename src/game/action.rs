use crate::game::tile::TileCoords;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(move_action);
}

#[derive(Event, Debug)]
pub struct MoveAction {
    pub agent_e: Entity,
    pub target_tile: Coords,
    pub pip_cost: u8,
}

fn move_action(trig: Trigger<MoveAction>, mut cmd: Commands, grid: Single<&Grid>) {
    // todo: remove pips
    let pos = or_return!(grid.tile_to_world(trig.target_tile));
    or_return!(cmd.get_entity(trig.agent_e)).insert((
        tween::get_relative_translation_anim(pos, 300, Some(EaseFunction::BackIn)),
        TileCoords(trig.target_tile),
    ));
}
