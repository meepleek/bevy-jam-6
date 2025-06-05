use bevy::prelude::*;

use super::Coords;
use super::board::Grid;
use crate::prelude::*;

pub const PIECE_TILE_SIZE: u16 = 64;

pub fn plugin(app: &mut App) {
    app.add_observer(on_drag).add_observer(on_piece_drag_end);
}

#[derive(Component, Debug, Clone)]
pub struct Draggable;

pub trait SnapTarget: Default + PartialEq + Send + Sync + 'static {}

#[derive(Clone, PartialEq, Default)]
pub struct SnapHover;
impl SnapTarget for SnapHover {}

#[derive(Component, Debug, Clone)]
#[relationship(relationship_target = Snappables<T>)]
pub struct SnappedTo<T: SnapTarget> {
    #[relationship]
    pub snapped_to: Entity,
    pub coords: Coords,
    _t: PhantomData<T>,
}
impl<T: SnapTarget> SnappedTo<T> {
    pub fn new(snapped_to: Entity, coords: Coords) -> Self {
        Self {
            snapped_to,
            coords,
            _t: PhantomData::default(),
        }
    }
}

#[derive(Component, Default)]
#[relationship_target(relationship = SnappedTo<S>, linked_spawn)]
pub struct Snappables<S: SnapTarget> {
    #[relationship]
    entities: Vec<Entity>,
    _s: PhantomData<S>,
}

#[cfg_attr(feature = "native_dev", hot)]
fn on_drag(
    drag: Trigger<Pointer<Drag>>,
    mut cmd: Commands,
    mut draggable_q: Query<(Entity, &mut Transform, Has<SnappedTo<SnapHover>>), With<Draggable>>,
    board_q: Query<(Entity, &Grid)>,
) {
    let (e, mut draggable_t, has_snap_hover) = or_return_quiet!(draggable_q.get_mut(drag.target()));
    let delta = drag.delta * Vec2::new(1., -1.);
    draggable_t.translation += delta.extend(0.);
    let piece_pos = draggable_t.translation.xy();

    match (
        board_q.iter().find_map(|(board_e, board)| {
            let Some(tile) = board.world_to_tile(piece_pos) else {
                return None;
            };
            let tile_world = board
                .tile_to_world(tile)
                .expect("Tile should map back to world position");
            // t snap when the piece is too close to either axis
            let max_dist = PIECE_TILE_SIZE as f32 * 0.4;
            if (tile_world - piece_pos).abs().max_element() < max_dist {
                return Some((board_e, tile));
            }

            None
        }),
        has_snap_hover,
    ) {
        (Some((board_e, snap_pos)), _) => {
            cmd.entity(e)
                .try_insert(SnappedTo::<SnapHover>::new(board_e, snap_pos));
        },
        (None, true) => {
            cmd.entity(e).try_remove::<SnappedTo<SnapHover>>();
        },
        (None, false) => {},
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn on_piece_drag_end(
    drag: Trigger<Pointer<DragEnd>>,
    mut draggable_q: Query<(&mut Transform, Option<&SnappedTo<SnapHover>>), With<Draggable>>,
    board_q: Query<&Grid>,
) {
    let (mut t, snap_hover) = or_return_quiet!(draggable_q.get_mut(drag.target()));
    let snap_hover = or_return_quiet!(snap_hover);
    let board = or_return!(board_q.get(snap_hover.snapped_to));
    let snapped_pos = or_return!(board.tile_to_world(snap_hover.coords));
    t.translation = snapped_pos.extend(t.translation.z);
}
