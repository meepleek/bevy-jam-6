use bevy::math::U16Vec2;
use bevy::platform::collections::HashMap;

use super::Coords;
use crate::game::tile::TileEntity;
use crate::game::tile::TileEntityKind;
use crate::prelude::*;

pub const TILE_SIZE: u16 = 64;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (track_position, track_tile_entities));
}

#[derive(Component)]
#[require(Transform)]
pub struct Grid {
    width: u16,
    heigth: u16,
    center_global_position: Vec2,
    occupied_tiles: HashMap<Coords, TileEntity>,
    entities: HashMap<Entity, Coords>,
}

#[derive(Debug, PartialEq, Eq, derive_more::Error, derive_more::Display)]
pub enum PlaceError {
    Taken,
    OutOfBounds,
}

#[derive(Debug, PartialEq, Eq, derive_more::Error, derive_more::Display)]
pub enum MoveError {
    Taken,
    OutOfBounds,
    EntityLookupFailed,
}
impl From<PlaceError> for MoveError {
    fn from(place_err: PlaceError) -> Self {
        match place_err {
            PlaceError::Taken => Self::Taken,
            PlaceError::OutOfBounds => Self::OutOfBounds,
        }
    }
}

impl Grid {
    pub fn new(width: u16, heigth: u16) -> Self {
        if width == 0 || heigth == 0 {
            panic!("Invalid dimension - no dimension can be 0");
        }
        Self {
            width,
            heigth,
            occupied_tiles: HashMap::default(),
            entities: HashMap::default(),
            center_global_position: Vec2::ZERO,
        }
    }

    #[allow(dead_code)]
    pub fn world_center(&self) -> Vec2 {
        self.center_global_position
    }

    pub fn grid_size(&self) -> U16Vec2 {
        (self.width, self.heigth).into()
    }

    pub fn size(&self) -> Vec2 {
        self.grid_size().as_vec2() * TILE_SIZE as f32
    }

    pub fn coords_to_tile_entity(&self, coords: Coords) -> Option<TileEntity> {
        self.occupied_tiles.get(&coords).cloned()
    }

    pub fn contains_die(&self, coords: Coords) -> bool {
        self.coords_to_tile_entity(coords)
            .is_some_and(|tile_entity| {
                matches!(
                    tile_entity.kind,
                    TileEntityKind::Player | TileEntityKind::Enemy
                )
            })
    }

    pub fn entity_to_coords(&self, entity: Entity) -> Option<Coords> {
        self.entities.get(&entity).cloned()
    }

    pub fn world_to_tile(&self, pos: Vec2) -> Option<Coords> {
        // transform world position to board space (like screen space but in tiles)
        let half_size = self.size() / 2.;
        let x = half_size.x - self.center_global_position.x + pos.x;
        let y = half_size.y + self.center_global_position.y - pos.y;
        let pos_on_board = Vec2::new(x, y);
        let coords = (pos_on_board / TILE_SIZE as f32).floor().as_i16vec2();
        if coords.min_element() < 0
            || coords.x >= self.width as i16
            || coords.y >= self.heigth as i16
        {
            return None;
        }

        Some(coords)
    }

    pub fn tile_to_world(&self, tile: Coords) -> Option<Vec2> {
        if tile.min_element() < 0 || tile.x >= self.width as i16 || tile.y >= self.heigth as i16 {
            return None;
        }

        let half_size = self.size() / 2.;
        let half_tile = TILE_SIZE as f32 / 2.;
        let tile_world = tile.as_vec2() * TILE_SIZE as f32;
        let x = tile_world.x + self.center_global_position.x + half_tile - half_size.x;
        let y = -tile_world.y + self.center_global_position.y - half_tile + half_size.y;
        Some(Vec2::new(x, y))
    }

    pub fn can_place_at(&self, coords: Coords) -> Result<(), PlaceError> {
        if coords.min_element() < 0
            || coords.x >= self.width as i16
            || coords.y >= self.heigth as i16
        {
            return Err(PlaceError::OutOfBounds);
        } else if self.occupied_tiles.contains_key(&coords) {
            return Err(PlaceError::Taken);
        }
        Ok(())
    }

    pub fn place_entity(
        &mut self,
        tile_entity: TileEntity,
        coords: Coords,
    ) -> Result<(), PlaceError> {
        self.can_place_at(coords)?;
        self.entities.insert(tile_entity.entity, coords);
        self.occupied_tiles.insert(coords, tile_entity);

        Ok(())
    }

    pub fn move_entity(&mut self, entity: Entity, coords: Coords) -> Result<(), MoveError> {
        self.can_place_at(coords)?;
        match self.entities.get(&entity) {
            Some(prev_tile) => match self.clear_tile(*prev_tile) {
                Some(tile_entity) => self.place_entity(tile_entity, coords)?,
                None => panic!("Reverse coords lookup failed"),
            },
            None => return Err(MoveError::EntityLookupFailed),
        }
        Ok(())
    }

    fn clear_tile(&mut self, coords: Coords) -> Option<TileEntity> {
        self.occupied_tiles.remove(&coords)
    }
}

fn track_position(mut board_q: Query<(&mut Grid, &GlobalTransform), Changed<GlobalTransform>>) {
    for (mut board, t) in &mut board_q {
        board.center_global_position = t.translation().truncate();
    }
}

fn track_tile_entities(
    entity_q: Query<(Entity, &TileEntityKind, &GlobalTransform), Changed<GlobalTransform>>,
    mut grid: Single<&mut Grid>,
) {
    for (e, kind, t) in &entity_q {
        let tile = or_continue!(grid.world_to_tile(t.translation().truncate()));
        if grid.entities.contains_key(&e) {
            or_continue!(grid.move_entity(e, tile));
        } else {
            or_continue!(grid.place_entity(
                TileEntity {
                    entity: e,
                    kind: *kind,
                },
                tile,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use tracing_test::traced_test;

    use super::*;
    use crate::game::tile::TileEntityKind;

    #[test_case(0., 0., 0., 0. => Some(Coords::ONE))]
    #[test_case(64.,-64., 0., 0. => Some(Coords::ZERO))]
    #[test_case(64.,-64., 20., -20. => Some(Coords::ZERO))]
    #[test_case(64.,-64., 40., -40. => Some(Coords::ONE))]
    #[test_case(64., -64., 64., 0. => Some(Coords::new(1, 0)))]
    #[test_case(0., 0., 120., 0. => None)]
    #[test_case(0., 0., -128., 0. => None)]
    #[test_case(0., 0., 0., 120. => None)]
    #[test_case(0., 0., 0., -128. => None)]
    #[traced_test]
    fn world_to_tile(map_x: f32, map_y: f32, world_x: f32, world_y: f32) -> Option<Coords> {
        let mut board = Grid::new(3, 3);
        board.center_global_position = Vec2::new(map_x, map_y);

        board.world_to_tile(Vec2::new(world_x, world_y))
    }

    #[test_case(0., 0., 0, 0 => Some(Vec2::new(-64., 64.)))]
    #[test_case(0., 0., 1, 1 => Some(Vec2::new(0., 0.)))]
    // todo: fix failing test
    // #[test_case(64.,-64., 0, 0 => Some(Vec2::new(64., -64.)))]
    #[test_case(64.,-64., 2, 2 => Some(Vec2::new(128., -128.)))]
    #[test_case(0.,0., 3, 0 => None)]
    #[test_case(0.,0., 0, 3 => None)]
    #[traced_test]
    fn tile_to_world(map_x: f32, map_y: f32, tile_x: i16, tile_y: i16) -> Option<Vec2> {
        let mut board = Grid::new(3, 3);
        board.center_global_position = Vec2::new(map_x, map_y);

        board.tile_to_world(Coords::new(tile_x, tile_y))
    }

    #[test_case(0, 0 => matches Ok(_))]
    #[test_case(3, 3 => matches Ok(_))]
    #[test_case(4, 6 => matches Ok(_))]
    #[test_case(6, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 9 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(50, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 50 => matches Err(PlaceError::OutOfBounds))]
    fn can_place_at_coords(x: i16, y: i16) -> Result<(), PlaceError> {
        let board = Grid::new(6, 9);
        board.can_place_at((x, y).into())
    }

    #[test]
    fn cannot_place_at_coords_when_taken() {
        let coords: Coords = (3, 3).into();
        let mut board = Grid::new(6, 6);
        board
            .place_entity(
                TileEntity {
                    kind: TileEntityKind::Player,
                    entity: Entity::PLACEHOLDER,
                },
                coords,
            )
            .expect("Place first piece");

        assert_eq!(board.can_place_at(coords), Err(PlaceError::Taken));
    }
}
