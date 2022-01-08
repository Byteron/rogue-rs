use bevy::math::IVec2;

pub struct Room {
    pub start: IVec2,
    pub end: IVec2,
}

impl Room {
    pub fn new(start: IVec2, end: IVec2) -> Self {
        return Room { start, end };
    }

    pub fn is_wall(&self, coords: IVec2) -> bool {
        return self.start.x == coords.x
            || self.end.x - 1 == coords.x
            || self.start.y == coords.y
            || self.end.y - 1 == coords.y;
    }
}
