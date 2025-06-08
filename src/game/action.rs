use crate::game::die::Die;
use crate::game::tile::TileCoords;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(move_action).add_systems(Update, die);
}

#[derive(Event, Debug)]
pub struct MoveAction {
    pub agent_e: Entity,
    pub target_tile: Coords,
    pub pip_cost: u8,
}

fn move_action(
    trig: Trigger<MoveAction>,
    mut cmd: Commands,
    grid: Single<&Grid>,
    mut die_q: Query<&mut Die>,
) {
    let pos = or_return!(grid.tile_to_world(trig.target_tile));
    or_return!(cmd.get_entity(trig.agent_e)).insert((
        tween::get_relative_translation_anim(pos, 300, Some(EaseFunction::BackIn)),
        TileCoords(trig.target_tile),
    ));
    let mut die = or_return_quiet!(die_q.get_mut(trig.agent_e));
    die.pip_count = die.pip_count.saturating_sub(trig.pip_cost);
}

fn die(die_q: Query<(Entity, &Die), Changed<Die>>) {
    for (e, die) in die_q.iter().filter(|(_, pips)| pips.pip_count == 0) {
        warn!(?die, ?e, "this die should die...");
    }
}
