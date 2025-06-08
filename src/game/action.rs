use crate::game::die::Pips;
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
    mut pip_q: Query<&mut Pips>,
) {
    let pos = or_return!(grid.tile_to_world(trig.target_tile));
    or_return!(cmd.get_entity(trig.agent_e)).insert((
        tween::get_relative_translation_anim(pos, 300, Some(EaseFunction::BackIn)),
        TileCoords(trig.target_tile),
    ));
    let mut pips = or_return_quiet!(pip_q.get_mut(trig.agent_e));
    pips.0 = pips.saturating_sub(trig.pip_cost);
}

fn die(pip_q: Query<(Entity, &Pips), Changed<Pips>>) {
    for (e, pips) in pip_q.iter().filter(|(_, pips)| pips.0 == 0) {
        warn!("this die should die...");
    }
}
