use crate::{grid::{Vec2i, Vec3i}, rooms::{self, Rooms, Tiles}};

pub fn generate(rooms: &mut Rooms, tiles: &mut Tiles) -> Vec3i {
    let room_size = Vec2i::new(19, 11);
    let level_size = Vec2i::new(10, 10);
    
    for level in 0..50 {
        for y in -level_size.y..=level_size.y {
            for x in -level_size.x..=level_size.x {
                let room_coords = Vec2i::new(x, y);

                let room = rooms::create_room(tiles, level, room_coords * room_size, room_size);
                rooms.insert(room_coords.extend(level), room);
            }
        }
    }

    (level_size / Vec2i::new(2, 2)).extend(0)
}