use bevy::math::IVec2;

pub struct Room {
    pub position: IVec2,
    pub size: IVec2,
}

impl Room {
    pub fn contains(&self, coords: IVec2) -> bool {
        coords.x >= self.position.x
            && coords.y >= self.position.y
            && coords.x < self.end().x
            && coords.y < self.end().y
    }

    pub fn end(&self) -> IVec2 {
        self.position + self.size
    }

    pub fn last(&self) -> IVec2 {
        self.end() - IVec2::new(1, 1)
    }

    pub fn center(&self) -> IVec2 {
        self.position + self.size / IVec2::new(2, 2)
    }

    pub fn coords(&self) -> Vec<IVec2> {
        let mut coords: Vec<IVec2> = Vec::default();
        for y in self.position.y..self.end().y {
            for x in self.position.x..self.end().x {
                coords.push(IVec2::new(x, y))
            }
        }

        coords
    }

    pub fn is_door(&self, coords: IVec2) -> bool {
        coords.x == self.center().x && (coords.y == self.position.y || coords.y == self.last().y)
            || coords.y == self.center().y
                && (coords.x == self.position.x || coords.x == self.last().x)
    }

    pub fn is_exit(&self, coords: IVec2) -> bool {
        coords.x == self.center().x && (coords.y == self.position.y - 1 || coords.y == self.end().y)
            || coords.y == self.center().y
                && (coords.x == self.position.x - 1 || coords.x == self.end().x)
    }

    pub fn is_center(&self, coords: IVec2) -> bool {
        self.center() == coords
    }

    pub fn is_entrance(&self, coords: IVec2) -> bool {
        coords.x == self.center().x
            && (coords.y == self.position.y + 1 || coords.y == self.last().y - 1)
            || coords.y == self.center().y
                && (coords.x == self.position.x + 1 || coords.x == self.last().x - 1)
    }

    pub fn is_border(&self, coords: IVec2) -> bool {
        coords.x == self.position.x
            || coords.y == self.position.y
            || coords.x == self.last().x
            || coords.y == self.last().y
    }
}
