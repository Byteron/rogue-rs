use bevy::{prelude::*, utils::HashMap};

use crate::core::Coordinates;

#[derive(Clone)]
pub struct Tile {
    pub material: Handle<ColorMaterial>,
}

pub struct TileSprite;

pub struct TileMapCursor;

pub struct TileSet {
    tiles: Vec<Tile>,
}

impl TileSet {
    pub fn create_tile(&mut self, material: Handle<ColorMaterial>) -> usize {
        self.tiles.push(Tile { material });
        self.tiles.len() - 1
    }
}

impl Default for TileSet {
    fn default() -> Self {
        TileSet {
            tiles: Vec::default(),
        }
    }
}

pub struct TileMap {
    pub cell_size: Vec2,
    pub draw_size: Vec2,
    pub tile_set: TileSet,
    pub tiles: HashMap<Coordinates, Tile>,
    sprites: HashMap<Coordinates, Entity>,
}

impl TileMap {
    pub fn map_to_world(&self, coords: Coordinates) -> Vec3 {
        (coords.to_vec() * self.cell_size).extend(0.0)
    }

    pub fn map_to_world_centered(&self, coords: Coordinates) -> Vec3 {
        self.map_to_world(coords) + self.cell_size.extend(0.0) / Vec3::new(2.0, 2.0, 0.0)
    }

    pub fn world_to_map(&self, translation: Vec3) -> Coordinates {
        let vec = (Vec2::new(translation.x, translation.y) / self.cell_size).floor();
        Coordinates::new(vec.x as i32, vec.y as i32)
    }

    pub fn set_cell(&mut self, coords: Coordinates, index: usize) {
        let tile = self.tile_set.tiles.get(index).unwrap().clone();
        self.tiles.insert(coords, tile);
    }
}

impl Default for TileMap {
    fn default() -> Self {
        TileMap {
            cell_size: Vec2::new(64.0, 64.0),
            draw_size: Vec2::new(12.0, 7.0),
            tile_set: TileSet::default(),
            tiles: HashMap::default(),
            sprites: HashMap::default(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(TileMap::default())
            .add_system(draw_tiles.system());
    }
}

fn draw_tiles(
    commands: &mut Commands,
    mut map: ResMut<TileMap>,
    cursors: Query<&Coordinates, With<TileMapCursor>>,
) {
    
    for cursor in cursors.iter() {

        let mut kernel: Vec<Coordinates> = Vec::default();
        
        for y in -map.draw_size.y as i32..=map.draw_size.y as i32 {
            for x in -map.draw_size.x as i32..=map.draw_size.x as i32 {
                kernel.push(Coordinates::new(x, y) + *cursor)
            }
        }

        spawn(commands, &mut map, &kernel);
        despawn(commands, &mut map, &kernel);
    }
}

fn spawn(commands: &mut Commands, map: &mut TileMap, kernel: &Vec<Coordinates>) {
    let mut sprites: HashMap<Coordinates, Entity> = HashMap::default();

    for (coords, tile) in map.tiles.iter() {
        if !kernel.contains(coords) {
            continue;
        }

        if !map.sprites.contains_key(coords) {
            let entity = commands
                .spawn(SpriteBundle {
                    material: tile.material.clone(),
                    transform: Transform::from_translation(
                        map.map_to_world(*coords) + Vec3::new(0.0, 0.0, -0.05),
                    ),
                    sprite: Sprite {
                        size: map.cell_size,
                        resize_mode: SpriteResizeMode::Manual,
                    },
                    ..Default::default()
                })
                .with(TileSprite)
                .current_entity()
                .unwrap();

            sprites.insert(*coords, entity);
        }
    }

    for (coords, entity) in sprites.iter() {
        map.sprites.insert(*coords, *entity);
    }
}

fn despawn(commands: &mut Commands, map: &mut TileMap, kernel: &Vec<Coordinates>) {
    let mut removed: Vec<Coordinates> = Vec::default();

    for (coords, entity) in map.sprites.iter() {
        if !kernel.contains(coords) {
            commands.despawn(*entity);
            removed.push(*coords)
        }
    }

    for coords in removed.iter() {
        map.sprites.remove(coords);
    }
}

