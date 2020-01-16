use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;
use wrapped2d::dynamics::world::BodyHandle;
use wrapped2d::dynamics::contacts::Contact;

use lib::math::{Vec2, vec2};

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


fn to_vec2(vector: b2::Vec2) -> Vec2 {
    vec2(vector.x, vector.y)
}


fn to_bvec(vector: Vec2) -> b2::Vec2 {
    b2::Vec2 { x: vector.x, y: vector.y }
}


fn b2_get_local_point(body: &b2::Body, world_point: &b2::Vec2) -> b2::Vec2 {
    let t = body.transform();
    let px = world_point.x - t.pos.x;
    let py = world_point.y - t.pos.y;
    b2::Vec2 {
        x: t.rot.cos * px + t.rot.sin * py,
        y: -t.rot.sin * px + t.rot.cos * py
    }
}


fn evaluate_contact(world: &B2World, contact: &Contact) -> (i32, b2::WorldManifold) {
    let body = world.body(contact.fixture_a().0);
    let transform_a = body.transform();
    let radius_a = match &*body.fixture(contact.fixture_a().1).shape() {
        b2::UnknownShape::Circle(circle) => circle.radius(),
        _ => b2::POLYGON_RADIUS
    };
    let body = world.body(contact.fixture_b().0);
    let transform_b = body.transform();
    let radius_b = match &*body.fixture(contact.fixture_b().1).shape() {
        b2::UnknownShape::Circle(circle) => circle.radius(),
        _ => b2::POLYGON_RADIUS
    };
    let manifold = unsafe {
        use wrapped2d::dynamics::contacts::ffi;
        use wrapped2d::wrap::Wrapped;
        let mut m = std::mem::MaybeUninit::uninit();
        // Converting from *const to *mut here is not UB.
        // Evaluating a contact doesn't mutate it, but Box2D incorrectly takes a non-const pointer,
        // and wrapper2d does not correct this.
        ffi::Contact_evaluate_virtual(contact.ptr() as *mut _, m.as_mut_ptr(), transform_a, transform_b);
        m.assume_init()
    };
    (manifold.count, manifold.world_manifold(transform_a, radius_a, transform_b, radius_b))
}


impl World {
    pub fn new(state: &state::State) -> World {
        let mut world = B2World::new(&b2::Vec2 { x: 0.0, y: -10.0 });

        let ground = world.create_body(&b2::BodyDef::new());

        let cat = NoodleCat::new(&mut world, &state.cat);

        let test = world.create_body(&b2::BodyDef {
            body_type: b2::BodyType::Dynamic,
            position: b2::Vec2 { x: 26.5, y: -9.0 },
            ..b2::BodyDef::new()
        });
        let mut body = world.body_mut(test);
        let circle = b2::CircleShape::new_with(b2::Vec2 { x: 0.0, y: 0.0 }, 0.5);
        body.create_fast_fixture(&circle, 1.0);
        drop(body);

        let test = world.create_body(&b2::BodyDef {
            body_type: b2::BodyType::Dynamic,
            position: b2::Vec2 { x: 26.5, y: -10.0 },
            ..b2::BodyDef::new()
        });
        let mut body = world.body_mut(test);
        let square = b2::PolygonShape::new_box(0.5, 0.5);
        body.create_fast_fixture(&square, 1.0);
        drop(body);

        let test = world.create_body(&b2::BodyDef {
            body_type: b2::BodyType::Kinematic,
            position: b2::Vec2 { x: 25.5, y: -6.0 },
            angular_velocity: std::f32::consts::PI * 0.25,
            ..b2::BodyDef::new()
        });
        let mut body = world.body_mut(test);
        let rectangle = b2::PolygonShape::new_box(2.0, 0.5);
        body.create_fast_fixture(&rectangle, 1.0);
        drop(body);

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
            for p in ground.boxes.iter().copied() {
                let square = b2::PolygonShape::new_oriented_box(0.5, 0.5, &to_bvec(p), 0.0);
                body.create_fast_fixture(&square, 1.0);
            }
            ground.dirty -= state::DirtyFlags::PHYSICS;
        }

        self.cat.control(&mut self.world, cat);

        self.world.step(delta_time, 5, 5);

        self.cat.update(cat, &self.world);
    }


    pub fn debug(&mut self, info: &mut state::DebugInfo) {
        self.world.draw_debug_data(&mut DebugDraw { info }, b2::DRAW_SHAPE | b2::DRAW_JOINT);
        self.debug_contacts(info);
    }


    fn debug_contacts(&mut self, info: &mut state::DebugInfo) {
        for contact in self.world.contacts() {
            if !contact.is_touching() {
                continue;
            }
            let (points, manifold) = evaluate_contact(&self.world, &*contact);
            let mut p1 = to_vec2(manifold.points[0]);
            if points == 1 {
                info.shapes.push_back((
                    state::DebugShape::Circle(
                        manifold.points[0].x, manifold.points[0].y, 0.05
                    ), state::DebugColor(255, 0, 0, 255)
                ));
            }
            else {
                info.shapes.push_back((
                    state::DebugShape::Line(
                        manifold.points[0].x, manifold.points[0].y, manifold.points[1].x, manifold.points[1].y
                    ), state::DebugColor(255, 0, 0, 255)
                ));
                p1 += to_vec2(manifold.points[1]);
                p1 *= 0.5;
            }
            let p2 = p1 + to_vec2(manifold.normal);
            info.shapes.push_back((
                state::DebugShape::Line(
                    p1.x, p1.y, p2.x, p2.y
                ), state::DebugColor(0, 255, 0, 255)
            ));
        }
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
