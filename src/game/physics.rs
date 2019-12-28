use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;
use wrapped2d::dynamics::world::BodyHandle;

use lib::math::vec2;

use super::state;


pub struct World {
    world: b2::World<NoUserData>,
    ground: BodyHandle,
    cat_links: Vec<BodyHandle>,
    tail_links: Vec<BodyHandle>
}


impl World {
    pub fn new(state: &state::State) -> World {
        let mut world = b2::World::<NoUserData>::new(&b2::Vec2 { x: 0.0, y: -10.0 });

        let ground = world.create_body(&b2::BodyDef::new());

        let path = &state.cat.path;
        let mut cat_links: Vec<BodyHandle> = Vec::with_capacity(path.len());
        let mut link = world.create_body(
            &b2::BodyDef {
                body_type: b2::BodyType::Dynamic,
                position: b2::Vec2 { x: path[0].x, y: path[0].y },
                linear_damping: 2.0,
                angular_damping: 1.0,
                .. b2::BodyDef::new()
            }
        );
        let butt = link;
        let circle = b2::CircleShape::new_with(b2::Vec2 { x: 0.0, y: 0.0 }, 0.5);
        let mut fixture = b2::FixtureDef::new();
        fixture.density = 1.0;
        fixture.restitution = 0.0;
        fixture.friction = 0.0;
        fixture.filter.group_index = -1;
        world.body_mut(link).create_fixture(&circle, &mut fixture);
        cat_links.push(link);
        for (p, p2) in path.iter().copied().zip(path.iter().copied().skip(1)) {
            let next = world.create_body(
                &b2::BodyDef {
                    body_type: b2::BodyType::Dynamic,
                    position: b2::Vec2 { x: p.x, y: p.y },
                    linear_damping: 2.0,
                    angular_damping: 1.0,
                    .. b2::BodyDef::new()
                }
            );
            world.body_mut(next).create_fixture(&circle, &mut fixture);
            let d = p - p2;
            world.create_joint(
                &b2::RevoluteJointDef {
                    local_anchor_b: b2::Vec2 { x: d.x, y: d.y },
                    lower_angle: -std::f32::consts::PI * 0.06,
                    upper_angle: std::f32::consts::PI * 0.06,
                    enable_limit: true,
                    ..b2::RevoluteJointDef::new(link, next)
                }
            );
            world.create_joint(
                &b2::MotorJointDef {
                    max_force: 0.0,
                    max_torque: 10.0,
                    correction_factor: 1.0,
                    ..b2::MotorJointDef::new(link, next)
                }
            );
            link = next;
            cat_links.push(link);
        }

        let tail = &state.cat.tail;
        let mut tail_links: Vec<BodyHandle> = Vec::with_capacity(tail.len());
        let mut link = world.create_body(
            &b2::BodyDef {
                body_type: b2::BodyType::Dynamic,
                position: b2::Vec2 { x: tail[0].x, y: tail[0].y },
                linear_damping: 2.0,
                angular_damping: 1.0,
                .. b2::BodyDef::new()
            }
        );
        let d = path[0] - tail[0];
        world.create_joint(
            &b2::RevoluteJointDef {
                local_anchor_b: b2::Vec2 { x: d.x, y: d.y },
                lower_angle: -std::f32::consts::PI * 0.06,
                upper_angle: std::f32::consts::PI * 0.06,
                enable_limit: true,
                ..b2::RevoluteJointDef::new(butt, link)
            }
        );
        world.create_joint(
            &b2::MotorJointDef {
                max_force: 0.0,
                max_torque: 1.6,
                correction_factor: 0.5,
                ..b2::MotorJointDef::new(butt, link)
            }
        );
        let circle = b2::CircleShape::new_with(b2::Vec2 { x: 0.0, y: 0.0 }, 0.2);
        let mut fixture = b2::FixtureDef::new();
        fixture.density = 1.0;
        fixture.restitution = 0.0;
        fixture.friction = 0.0;
        fixture.filter.group_index = -1;
        world.body_mut(link).create_fixture(&circle, &mut fixture);
        tail_links.push(link);
        for (p, p2) in tail.iter().copied().zip(tail.iter().copied().skip(1)) {
            let next = world.create_body(
                &b2::BodyDef {
                    body_type: b2::BodyType::Dynamic,
                    position: b2::Vec2 { x: p.x, y: p.y },
                    linear_damping: 2.0,
                    angular_damping: 1.0,
                    .. b2::BodyDef::new()
                }
            );
            world.body_mut(next).create_fixture(&circle, &mut fixture);
            let d = p - p2;
            world.create_joint(
                &b2::RevoluteJointDef {
                    local_anchor_b: b2::Vec2 { x: d.x, y: d.y },
                    lower_angle: -std::f32::consts::PI * 0.06,
                    upper_angle: std::f32::consts::PI * 0.06,
                    enable_limit: true,
                    ..b2::RevoluteJointDef::new(link, next)
                }
            );
            world.create_joint(
                &b2::MotorJointDef {
                    max_force: 0.0,
                    max_torque: 1.6,
                    correction_factor: 1.0,
                    ..b2::MotorJointDef::new(link, next)
                }
            );
            link = next;
            tail_links.push(link);
        }

        World {
            world,
            ground,
            cat_links,
            tail_links
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

        if cat.direction != vec2(0.0, 0.0) {
            let mut body = self.world.body_mut(*self.cat_links.last().unwrap());
            let d = cat.direction * 5.0;
            body.set_linear_velocity(&b2::Vec2 { x: d.x, y: d.y });
        }

        self.world.step(delta_time, 5, 5);

        for (p, link) in cat.path.iter_mut().zip(self.cat_links.iter().copied()) {
            let body = self.world.body(link);
            p.x = body.position().x;
            p.y = body.position().y;
        }
        for (p, link) in cat.tail.iter_mut().zip(self.tail_links.iter().copied()) {
            let body = self.world.body(link);
            p.x = body.position().x;
            p.y = body.position().y;
        }
    }
}
