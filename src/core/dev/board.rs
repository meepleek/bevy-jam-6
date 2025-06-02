use bevy::ui::UiDebugOptions;

use crate::game::board::Board;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // todo: probly use states...
    app.add_systems(Update, draw_board_grid);
}

#[cfg_attr(feature = "native_dev", hot)]
fn draw_board_grid(options: Res<UiDebugOptions>, mut gizmos: Gizmos, board_q: Query<&Board>) {
    if options.enabled {
        for board in board_q {
            gizmos
                .grid_2d(
                    Isometry2d::IDENTITY,
                    board.size().as_uvec2(),
                    Vec2::ONE * 64.,
                    CYAN_400,
                )
                .outer_edges();
        }
    }
}
