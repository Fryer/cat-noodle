use wrapped2d::b2;
use wrapped2d::user_data::NoUserData;
use wrapped2d::dynamics::world::{BodyHandle, JointHandle};

use lib::math::{Vec2, wrap_angle};

use super::state;


pub struct World {
    world: b2::World<NoUserData>,
    ground: BodyHandle,
    cat_links: Vec<BodyHandle>,
    cat_muscles: Vec<JointHandle>,
    tail_links: Vec<BodyHandle>
}


impl World {
    pub fn new(state: &state::State) -> World {
        let mut world = b2::World::<NoUserData>::new(&b2::Vec2 { x: 0.0, y: -10.0 });

        let ground = world.create_body(&b2::BodyDef::new());

        let path = &state.cat.path;
        let mut cat_links: Vec<_> = Vec::with_capacity(path.len());
        let mut cat_muscles: Vec<_> = Vec::with_capacity(path.len().saturating_sub(1));
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
            let muscle = world.create_joint(
                &b2::MotorJointDef {
                    max_force: 0.0,
                    max_torque: 10.0,
                    correction_factor: 1.0,
                    ..b2::MotorJointDef::new(link, next)
                }
            );
            link = next;
            cat_links.push(link);
            cat_muscles.push(muscle);
        }

        let tail = &state.cat.tail;
        let mut tail_links: Vec<_> = Vec::with_capacity(tail.len());
        let mut link = world.create_body(
            &b2::BodyDef {
                body_type: b2::BodyType::Dynamic,
                position: b2::Vec2 { x: tail[0].x, y: tail[0].y },
                linear_damping: 2.0,
                angular_damping: 1.0,
                gravity_scale: 0.1,
                .. b2::BodyDef::new()
            }
        );
        let d = path[0] - tail[0];
        world.create_joint(
            &b2::RevoluteJointDef {
                local_anchor_b: b2::Vec2 { x: d.x, y: d.y },
                lower_angle: -std::f32::consts::PI * 0.25,
                upper_angle: std::f32::consts::PI * 0.25,
                enable_limit: true,
                ..b2::RevoluteJointDef::new(butt, link)
            }
        );
        world.create_joint(
            &b2::MotorJointDef {
                max_force: 0.0,
                max_torque: 5.0,
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
                    gravity_scale: 0.1,
                    .. b2::BodyDef::new()
                }
            );
            world.body_mut(next).create_fixture(&circle, &mut fixture);
            let d = p - p2;
            world.create_joint(
                &b2::RevoluteJointDef {
                    local_anchor_b: b2::Vec2 { x: d.x, y: d.y },
                    lower_angle: -std::f32::consts::PI * 0.1,
                    upper_angle: std::f32::consts::PI * 0.1,
                    enable_limit: true,
                    ..b2::RevoluteJointDef::new(link, next)
                }
            );
            world.create_joint(
                &b2::MotorJointDef {
                    max_force: 0.0,
                    max_torque: 0.5,
                    correction_factor: 0.5,
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
            cat_muscles,
            tail_links
        }
    }


    pub fn step(&mut self, state: &mut state::State, delta_time: f32) {
        let ground = &mut state.ground;
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

        self.control_cat(state, delta_time);

        self.world.step(delta_time, 5, 5);

        let cat = &mut state.cat;
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


    fn control_cat(&mut self, state: &mut state::State, _delta_time: f32) {
        let input = &mut state.input;
        let cat = &mut state.cat;

        let direction = match cat.direction {
            Some(direction) => direction,
            None => {
                for muscle in self.cat_muscles.iter().copied() {
                    let mut joint = self.world.joint_mut(muscle);
                    let motor = match &mut **joint {
                        b2::UnknownJoint::Motor(motor) => motor,
                        _ => unreachable!()
                    };
                    motor.set_angular_offset(0.0);
                    motor.set_max_torque(10.0);
                }
                return
            }
        };

        let mut turn = 0.0;
        if input.force {
            let mut body = self.world.body_mut(*self.cat_links.last().unwrap());
            let d = Vec2::from_angle(direction) * 5.0;
            body.set_linear_velocity(&b2::Vec2 { x: d.x, y: d.y });
        }
        else {
            let p = cat.path[cat.path.len() - 2];
            let p2 = cat.path.back().copied().unwrap();
            let d = p2 - p;
            if d.length_squared() >= 1000.0 * std::f32::EPSILON * std::f32::EPSILON {
                turn = wrap_angle(direction - d.y.atan2(d.x));
            }
        }

        let p3_iter = cat.path.iter().copied()
            .zip(cat.path.iter().copied().skip(1))
            .zip(cat.path.iter().copied().skip(2));
        let angle_iter = p3_iter.map(|((p, p2), p3)| {
            let d = p2 - p;
            let d2 = p3 - p2;
            if d.length() >= std::f32::EPSILON * 1000.0 && d2.length() >= std::f32::EPSILON * 1000.0 {
                let dd = d2.unrotated(d.normalized());
                dd.y.atan2(dd.x)
            }
            else { 0.0 }
        });

        for (muscle, angle) in self.cat_muscles.iter().copied().rev().zip(angle_iter.rev()) {
            let mut joint = self.world.joint_mut(muscle);
            let motor = match &mut **joint {
                b2::UnknownJoint::Motor(motor) => motor,
                _ => unreachable!()
            };
            if turn.abs() >= std::f32::EPSILON {
                let offset = (turn + angle).max(-std::f32::consts::PI * 0.06).min(std::f32::consts::PI * 0.06);
                motor.set_angular_offset(offset);
                motor.set_max_torque(100.0);
                turn -= offset - angle;
            }
            else {
                motor.set_angular_offset(0.0);
                motor.set_max_torque(10.0);
            }
        }
    }
}
