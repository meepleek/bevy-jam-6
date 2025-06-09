use bevy::ui::UiDebugOptions;

use crate::game::grid::Grid;
use crate::game::grid::TILE_SIZE;
use crate::prelude::*;

pub(super) fn plugin(_app: &mut App) {
    // todo: probly use states...
    // app.add_systems(Update, draw_board_grid);
}

#[cfg_attr(feature = "native_dev", hot)]
fn draw_board_grid(options: Res<UiDebugOptions>, mut gizmos: Gizmos, board_q: Query<&Grid>) {
    if options.enabled {
        for board in board_q {
            gizmos
                .grid_2d(
                    Isometry2d::from_translation(board.world_center()),
                    board.grid_size().as_uvec2(),
                    Vec2::ONE * TILE_SIZE as f32,
                    CYAN_400,
                )
                .outer_edges();
        }
    }
}
