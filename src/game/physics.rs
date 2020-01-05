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

struct DebugDraw<'a> {
    info: &'a mut state::DebugInfo
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


    pub fn update_debug(&mut self, info: &mut state::DebugInfo) {
        self.world.draw_debug_data(&mut DebugDraw { info }, b2::DrawFlags::all());
    }
}


impl From<&b2::Color> for state::DebugColor {
    fn from(color: &b2::Color) -> state::DebugColor {
        state::DebugColor(
            (color.r * 255.0).round() as _,
            (color.g * 255.0).round() as _,
            (color.b * 255.0).round() as _,
            (color.a * 255.0).round() as _
        )
    }
}


impl b2::Draw for DebugDraw<'_> {
    fn draw_polygon(&mut self, vertices: &[b2::Vec2], color: &b2::Color) {
        for (p1, p2) in vertices.iter().zip(vertices.iter().skip(1).chain(std::iter::once(&vertices[0]))) {
            self.info.shapes.push_back((
                state::DebugShape::Line(
                    p1.x, p1.y, p2.x, p2.y
                ), color.into()
            ));
        }
    }


    fn draw_solid_polygon(&mut self, vertices: &[b2::Vec2], color: &b2::Color) {
        for (p1, p2) in vertices.iter().zip(vertices.iter().skip(1).chain(std::iter::once(&vertices[0]))) {
            self.info.shapes.push_back((
                state::DebugShape::Line(
                    p1.x, p1.y, p2.x, p2.y
                ), color.into()
            ));
        }
    }


    fn draw_circle(&mut self, center: &b2::Vec2, radius: f32, color: &b2::Color) {
        self.info.shapes.push_back((
            state::DebugShape::Circle(
                center.x, center.y, radius
            ), color.into()
        ));
    }


    fn draw_solid_circle(&mut self, center: &b2::Vec2, radius: f32, axis: &b2::Vec2, color: &b2::Color) {
        self.info.shapes.push_back((
            state::DebugShape::Circle(
                center.x, center.y, radius
            ), color.into()
        ));
        self.info.shapes.push_back((
            state::DebugShape::Line(
                center.x, center.y,
                center.x + axis.x * radius,
                center.y + axis.y * radius
            ), color.into()
        ));
    }


    fn draw_segment(&mut self, p1: &b2::Vec2, p2: &b2::Vec2, color: &b2::Color) {
        self.info.shapes.push_back((
            state::DebugShape::Line(
                p1.x, p1.y, p2.x, p2.y
            ), color.into()
        ));
    }


    fn draw_transform(&mut self, xf: &b2::Transform) {
        self.info.shapes.push_back((
            state::DebugShape::Line(
                xf.pos.x, xf.pos.y,
                xf.pos.x + xf.rot.x_axis().x,
                xf.pos.y + xf.rot.x_axis().y,
            ), state::DebugColor(255, 0, 0, 255)
        ));
        self.info.shapes.push_back((
            state::DebugShape::Line(
                xf.pos.x, xf.pos.y,
                xf.pos.x + xf.rot.y_axis().x,
                xf.pos.y + xf.rot.y_axis().y,
            ), state::DebugColor(0, 255, 0, 255)
        ));
    }
}
