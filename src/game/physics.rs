use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;
use wrapped2d::dynamics::world::BodyHandle;

mod noodle_cat;
use noodle_cat::NoodleCat;

use super::state;


type B2World = b2::World<NoUserData>;

pub struct World {
    world: B2World,
    ground: BodyHandle,
    cat: NoodleCat
}


impl World {
    pub fn new(state: &state::State) -> World {
        let mut world = B2World::new(&b2::Vec2 { x: 0.0, y: -10.0 });

        let ground = world.create_body(&b2::BodyDef::new());

        let cat = NoodleCat::new(&mut world, &state.cat);

        World {
            world,
            ground,
            cat
        }
    }


    pub fn step(&mut self, state: &mut state::State, delta_time: f32) {
        let ground = &mut state.ground;
        let cat = &mut state.cat;

        if ground.dirty.contains(state::DirtyFlags::PHYSICS) {
            self.world.destroy_body(self.ground);
            self.ground = self.world.create_body(&b2::BodyDef::new());
            let mut body = self.world.body_mut(self.ground);
            for p in ground.boxes.iter() {
                let square = b2::PolygonShape::new_oriented_box(0.5, 0.5, &b2::Vec2 { x: p.x, y: p.y }, 0.0);
                body.create_fast_fixture(&square, 1.0);
            }
            ground.dirty -= state::DirtyFlags::PHYSICS;
        }

        self.cat.control(&mut self.world, cat);

        self.world.step(delta_time, 5, 5);

        self.cat.update(cat, &self.world);
    }
}
