use bevy::ui::UiDebugOptions;

use crate::game::grid::Grid;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // todo: probly use states...
    app.add_systems(Update, draw_board_grid);
}

#[cfg_attr(feature = "native_dev", hot)]
fn draw_board_grid(options: Res<UiDebugOptions>, mut gizmos: Gizmos, grid_q: Query<&Grid>) {
    if options.enabled {
        for grid in grid_q {
            gizmos
                .grid_2d(
                    Isometry2d::from_translation(grid.world_center()),
                    grid.grid_size().as_uvec2(),
                    Vec2::ONE * grid.tile_size() as f32,
                    CYAN_400,
                )
                .outer_edges();
        }
    }
}
