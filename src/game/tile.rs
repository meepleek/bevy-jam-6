use bevy::prelude::*;

use crate::prelude::*;

pub fn plugin(_app: &mut App) {}

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Player,
}

// #[cfg_attr(feature = "native_dev", hot)]
// fn on_piece_added(
//     mut cmd: Commands,
//     added_piece_q: Query<(Entity, &Tile), Added<Tile>>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let explosion_mat = materials.add(Color::from(AMBER_200));
//     let center_mat = materials.add(Color::from(RED_200));

//     for (e, piece) in added_piece_q {
//         let mut e_cmd = or_continue!(cmd.get_entity(e));
//         e_cmd.insert(Draggable);

//         e_cmd.with_children(|b| {
//             for tile in piece.explosion_tiles() {
//                 let is_center = tile == PieceCoords::ZERO;
//                 b.spawn((
//                     Mesh2d(meshes.add(Rectangle::new(
//                         TILE_SIZE as f32 * 0.9,
//                         TILE_SIZE as f32 * 0.9,
//                     ))),
//                     MeshMaterial2d(if is_center {
//                         center_mat.clone()
//                     } else {
//                         explosion_mat.clone()
//                     }),
//                     Transform::from_xyz(
//                         tile.x as f32 * TILE_SIZE as f32,
//                         tile.y as f32 * TILE_SIZE as f32,
//                         if is_center { 0.2 } else { 0.1 },
//                     ),
//                     TilePieceOf(b.target_entity()),
//                 ));
//             }
//         });
//     }
// }
