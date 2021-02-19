use crate::core::math::Vec2i;

pub struct Room {
    pub position: Vec2i,
    pub size: Vec2i,
}

impl Room {
    pub fn contains(&self, coords: Vec2i) -> bool {
        coords.x >= self.position.x
            && coords.y >= self.position.y
            && coords.x < self.end().x
            && coords.y < self.end().y
    }

    pub fn end(&self) -> Vec2i {
        self.position + self.size
    }

    pub fn last(&self) -> Vec2i {
        self.end() - Vec2i::new(1, 1)
    }

    pub fn center(&self) -> Vec2i {
        self.position + self.size / Vec2i::new(2, 2)
    }

    pub fn coords(&self) -> Vec<Vec2i> {
        let mut coords: Vec<Vec2i> = Vec::default();
        for y in self.position.y..self.end().y {
            for x in self.position.x..self.end().x {
                coords.push(Vec2i::new(x, y))
            }
        }

        coords
    }

    pub fn is_door(&self, coords: Vec2i) -> bool {
        coords.x == self.center().x && (coords.y == self.position.y || coords.y == self.last().y)
            || coords.y == self.center().y
                && (coords.x == self.position.x || coords.x == self.last().x)
    }

    pub fn is_exit(&self, coords: Vec2i) -> bool {
        coords.x == self.center().x && (coords.y == self.position.y - 1 || coords.y == self.end().y)
            || coords.y == self.center().y
                && (coords.x == self.position.x - 1 || coords.x == self.end().x)
    }

    pub fn is_center(&self, coords: Vec2i) -> bool {
        self.center() == coords
    }

    pub fn is_entrance(&self, coords: Vec2i) -> bool {
        coords.x == self.center().x
            && (coords.y == self.position.y + 1 || coords.y == self.last().y - 1)
            || coords.y == self.center().y
                && (coords.x == self.position.x + 1 || coords.x == self.last().x - 1)
    }

    pub fn is_border(&self, coords: Vec2i) -> bool {
        coords.x == self.position.x
            || coords.y == self.position.y
            || coords.x == self.last().x
            || coords.y == self.last().y
    }
}
