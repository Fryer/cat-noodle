use std::collections::VecDeque;

use wrapped2d::b2;
use wrapped2d::dynamics::world::{BodyHandle, JointHandle};
use wrapped2d::dynamics::body::FixtureHandle;

use lib::math::{Vec2, vec2, wrap_angle};

use super::{
    state,
    B2World,
    to_vec2,
    to_bvec,
    b2_get_local_point,
    evaluate_contact
};


pub struct NoodleCat {
    links: VecDeque<BodyHandle>,
    muscles: VecDeque<JointHandle>,
    tail_links: Vec<BodyHandle>,
    head_sensor: FixtureHandle,
    grab: Option<JointHandle>,
    grabbed: Option<BodyHandle>,
    grab_d: Option<Vec2>,
    walk_length: f32,
    extend_phase: f32,
    contracting: bool,
    contract_phase: f32
}


impl NoodleCat {
    pub fn new(world: &mut B2World, cat: &state::Cat) -> NoodleCat {
        let path = &cat.path;
        let mut links: VecDeque<_> = VecDeque::with_capacity(path.len());
        let mut muscles: VecDeque<_> = VecDeque::with_capacity(path.len().saturating_sub(1));
        let mut link = world.create_body(
            &b2::BodyDef {
                body_type: b2::BodyType::Dynamic,
                position: to_bvec(path[0]),
                linear_damping: 2.0,
                angular_damping: 1.0,
                ..b2::BodyDef::new()
            }
        );
        let butt = link;
        let mut head = link;
        let circle = b2::CircleShape::new_with(b2::Vec2 { x: 0.0, y: 0.0 }, 0.5);
        let mut fixture = b2::FixtureDef::new();
        fixture.density = 1.0;
        fixture.restitution = 0.0;
        fixture.friction = 0.0;
        fixture.filter.group_index = -1;
        world.body_mut(link).create_fixture(&circle, &mut fixture);
        links.push_back(link);
        for (p, p2) in path.iter().copied().zip(path.iter().copied().skip(1)) {
            let next = world.create_body(
                &b2::BodyDef {
                    body_type: b2::BodyType::Dynamic,
                    position: to_bvec(p2),
                    linear_damping: 2.0,
                    angular_damping: 1.0,
                    ..b2::BodyDef::new()
                }
            );
            world.body_mut(next).create_fixture(&circle, &mut fixture);
            let d = p - p2;
            world.create_joint(
                &b2::RevoluteJointDef {
                    local_anchor_b: to_bvec(d),
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
            head = link;
            links.push_back(link);
            muscles.push_back(muscle);
        }

        let tail = &cat.tail;
        let mut tail_links: Vec<_> = Vec::with_capacity(tail.len());
        let mut link = world.create_body(
            &b2::BodyDef {
                body_type: b2::BodyType::Dynamic,
                position: to_bvec(tail[0]),
                linear_damping: 2.0,
                angular_damping: 1.0,
                gravity_scale: 0.1,
                ..b2::BodyDef::new()
            }
        );
        let d = path[0] - tail[0];
        world.create_joint(
            &b2::RevoluteJointDef {
                local_anchor_b: to_bvec(d),
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
                    position: to_bvec(p2),
                    linear_damping: 2.0,
                    angular_damping: 1.0,
                    gravity_scale: 0.1,
                    ..b2::BodyDef::new()
                }
            );
            world.body_mut(next).create_fixture(&circle, &mut fixture);
            let d = p - p2;
            world.create_joint(
                &b2::RevoluteJointDef {
                    local_anchor_b: to_bvec(d),
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

        let circle = b2::CircleShape::new_with(b2::Vec2 { x: 0.0, y: 0.0 }, 1.0);
        let mut fixture = b2::FixtureDef::new();
        fixture.is_sensor = true;
        fixture.filter.group_index = -1;
        let head_sensor = world.body_mut(head).create_fixture(&circle, &mut fixture);

        NoodleCat {
            links,
            muscles,
            tail_links,
            head_sensor,
            grab: None,
            grabbed: None,
            grab_d: None,
            walk_length: 0.0,
            extend_phase: 1.0,
            contracting: false,
            contract_phase: 0.0
        }
    }


    pub fn update(&self, cat: &mut state::Cat, world: &B2World) {
        if cat.path.len() != self.links.len() {
            cat.path.resize(self.links.len(), vec2(0.0, 0.0));
        }
        for (p, link) in cat.path.iter_mut().zip(self.links.iter().copied()) {
            let body = world.body(link);
            p.x = body.position().x;
            p.y = body.position().y;
        }
        if self.extend_phase < 1.0 {
            let p = cat.path.get(cat.path.len() - 2).copied().unwrap();
            let head = cat.path.back_mut().unwrap();
            *head = p + (*head - p) * self.extend_phase;
        }
        if self.contract_phase > 0.0 {
            let p = cat.path.get(1).copied().unwrap();
            let butt = cat.path.front_mut().unwrap();
            *butt = p + (*butt - p) * self.contract_phase;
        }
        for (p, link) in cat.tail.iter_mut().zip(self.tail_links.iter().copied()) {
            let body = world.body(link);
            p.x = body.position().x;
            p.y = body.position().y;
        }
        cat.grab_d = self.grab_d;
        if cat.direction.is_some() {
            cat.walk_phase += self.walk_length;
        }
        else {
            cat.walk_phase = 0.0;
        }
    }


    pub fn control(&mut self, world: &mut B2World, cat: &state::Cat, delta_time: f32) {
        self.walk_length = 0.0;

        let mut separation = std::f32::INFINITY;
        let mut other = None;
        let mut normal = vec2(0.0, 0.0);
        let head = self.links.back().copied().unwrap();
        for (_, contact) in world.body(head).contacts() {
            if !contact.is_touching() {
                continue;
            }
            let is_a_sensor = contact.fixture_a().0 == head && contact.fixture_a().1 == self.head_sensor;
            let is_b_sensor = contact.fixture_b().0 == head && contact.fixture_b().1 == self.head_sensor;
            if !is_a_sensor && !is_b_sensor {
                continue;
            }
            if is_a_sensor && world.body(contact.fixture_b().0).body_type() == b2::BodyType::Dynamic {
                continue;
            }
            if is_b_sensor && world.body(contact.fixture_a().0).body_type() == b2::BodyType::Dynamic {
                continue;
            }
            let (_, manifold) = evaluate_contact(world, &*contact);
            // TODO: Prioritize contacts according to direction of movement and currently grabbed body.
            // Select the closest contact.
            if manifold.separations[0] < separation {
                separation = manifold.separations[0];
                if is_a_sensor {
                    other = Some(contact.fixture_b().0);
                    normal = to_vec2(manifold.normal);
                }
                else {
                    other = Some(contact.fixture_a().0);
                    normal = -to_vec2(manifold.normal);
                }
            }
        }
        // Grab if the paws can reach the ground and the direction of movement is not pointing away from it.
        // Release if the direction of movement points away from the ground.
        let mut grab = separation < -0.3 || (self.grab.is_some() && separation.is_finite());
        if grab {
            if let Some(direction) = cat.direction {
                grab = Vec2::from_angle(direction).dot(normal) > -0.8;
            }
            else if self.grab.is_none() {
                grab = false;
            }
        }
        // Don't grab while turning or extending.
        if cat.turning || cat.extending {
            grab = false;
        }
        if grab {
            if let Some(grab) = self.grab {
                world.destroy_joint(grab);
            }
            let other = if cat.direction.is_some() { other.unwrap() } else { self.grabbed.unwrap() };
            let head_body = world.body(head);
            let other_body = world.body(other);
            let head_anchor = to_vec2(head_body.transform().pos);
            let mut other_anchor = head_anchor;
            if let Some(direction) = cat.direction {
                // Keep separation constant.
                other_anchor += normal * (separation + 0.3);
                // Project movement onto the contact tangent.
                // TODO: Rotate the head instead when moving backwards or into the ground.
                let d = Vec2::from_angle(direction);
                let tangent = normal.rotated(vec2(0.0, 1.0));
                let projection_length = tangent.dot(d);
                // Normalize the projected movement unless it's directed into the ground.
                if projection_length.abs() > 0.2 {
                    self.walk_length = 4.0 * delta_time;
                    other_anchor += tangent * projection_length.signum() * self.walk_length;
                }
            }
            let def = b2::RevoluteJointDef {
                collide_connected: true,
                local_anchor_a: b2_get_local_point(&*head_body, &to_bvec(head_anchor)),
                local_anchor_b: b2_get_local_point(&*other_body, &to_bvec(other_anchor)),
                ..b2::RevoluteJointDef::new(head, other)
            };
            drop(head_body);
            drop(other_body);
            self.grab = Some(world.create_joint(&def));
            self.grabbed = Some(other);
            self.grab_d = Some(normal);
        }
        else if let Some(grab) = self.grab {
            world.destroy_joint(grab);
            self.grab = None;
            self.grab_d = None;
        }

        let mut control_iter = Self::make_control_iter(&self.muscles, cat);
        if grab {
            Self::control_relaxed(world, &mut control_iter);
            self.follow_head(world, &cat);
        }
        else {
            if let Some(direction) = cat.direction {
                let mut body = world.body_mut(*self.links.back().unwrap());
                let d = Vec2::from_angle(direction);
                if cat.flying {
                    body.set_linear_velocity(&to_bvec(d * 5.0));
                }
                else {
                    // Apply swimming force proportional to cat length.
                    let force = cat.path.len() as f32;
                    body.apply_force_to_center(&to_bvec(d * force), true);
                    drop(body);
                    Self::control_movement(world, cat, &mut control_iter);
                    self.walk_length = 8.0 * delta_time;
                }
            }
            Self::control_relaxed(world, &mut control_iter);
        }
        drop(control_iter);

        if cat.turning {
            let path: Vec<_> = self.links.iter().copied().rev().map(|link| {
                let body = world.body(link);
                (to_vec2(*body.position()), body.angle())
            }).collect();
            let p_iter = path.iter().copied().map(|(p, _)| p);
            let angle_iter = path.iter().copied().skip(1).map(|(_, angle)| angle + std::f32::consts::PI);
            let angle_iter = angle_iter.clone().chain(std::iter::once(angle_iter.last().unwrap()));
            let turn_iter = self.links.iter().copied()
                .zip(p_iter)
                .zip(angle_iter)
                .map(|((link, p), angle)| (link, p, angle));

            for (link, p, angle) in turn_iter {
                world.body_mut(link).set_transform(&to_bvec(p), angle);
            }

            let p = path[0].0 + Vec2::from_angle(path[0].1) * 0.3;
            let angle = path[0].1 + std::f32::consts::PI;
            for link in self.tail_links.iter().copied() {
                world.body_mut(link).set_transform(&to_bvec(p), angle);
            }
        }

        self.extend_phase += delta_time * 80.0;
        if cat.extending && cat.path.len() < 200 {
            if self.extend_phase > 1.0 {
                let previous = self.links.back().copied().unwrap();
                let p = cat.path.back().copied().unwrap();
                let angle = world.body(previous).angle();
                let d = Vec2::from_angle(angle);
                let p2 = p + d * 0.1;
                let link = world.create_body(
                    &b2::BodyDef {
                        body_type: b2::BodyType::Dynamic,
                        position: to_bvec(p2),
                        angle,
                        linear_damping: 2.0,
                        angular_damping: 1.0,
                        ..b2::BodyDef::new()
                    }
                );
                let circle = b2::CircleShape::new_with(b2::Vec2 { x: 0.0, y: 0.0 }, 0.5);
                let mut fixture = b2::FixtureDef::new();
                fixture.density = 1.0;
                fixture.restitution = 0.0;
                fixture.friction = 0.0;
                fixture.filter.group_index = -1;
                world.body_mut(link).create_fixture(&circle, &mut fixture);
                world.create_joint(
                    &b2::RevoluteJointDef {
                        local_anchor_b: b2::Vec2 { x: -0.1, y: 0.0 },
                        lower_angle: -std::f32::consts::PI * 0.06,
                        upper_angle: std::f32::consts::PI * 0.06,
                        enable_limit: true,
                        ..b2::RevoluteJointDef::new(previous, link)
                    }
                );
                let muscle = world.create_joint(
                    &b2::MotorJointDef {
                        max_force: 0.0,
                        max_torque: 10.0,
                        correction_factor: 1.0,
                        ..b2::MotorJointDef::new(previous, link)
                    }
                );
                self.links.push_back(link);
                self.muscles.push_back(muscle);

                // Recreate head sensor.
                world.body_mut(previous).destroy_fixture(self.head_sensor);
                let circle = b2::CircleShape::new_with(b2::Vec2 { x: 0.0, y: 0.0 }, 1.0);
                let mut fixture = b2::FixtureDef::new();
                fixture.is_sensor = true;
                fixture.filter.group_index = -1;
                self.head_sensor = world.body_mut(link).create_fixture(&circle, &mut fixture);

                self.extend_phase -= 1.0;
            }
        }
        else {
            self.extend_phase = self.extend_phase.min(1.0);
        }

        self.contract_phase -= delta_time * 80.0;
        if self.contracting && self.contract_phase <= 0.0 {
            let butt = self.links.pop_front().unwrap();
            world.destroy_body(butt);

            // Reconnect tail root.
            // TODO: Smoothly interpolate the tail root connection.
            let butt = self.links.front().copied().unwrap();
            let tail_root = self.tail_links.first().copied().unwrap();
            world.create_joint(
                &b2::RevoluteJointDef {
                    local_anchor_b: b2::Vec2 { x: 0.4, y: 0.0 },
                    lower_angle: -std::f32::consts::PI * 0.25,
                    upper_angle: std::f32::consts::PI * 0.25,
                    enable_limit: true,
                    ..b2::RevoluteJointDef::new(butt, tail_root)
                }
            );
            world.create_joint(
                &b2::MotorJointDef {
                    max_force: 0.0,
                    max_torque: 5.0,
                    correction_factor: 0.5,
                    ..b2::MotorJointDef::new(butt, tail_root)
                }
            );

            self.contracting = false;
        }
        else if cat.contracting && self.links.len() > 30 {
            if self.contract_phase < 0.0 {
                self.contracting = true;
                self.contract_phase += 1.0;
            }
        }
        else {
            self.contract_phase = self.contract_phase.max(0.0);
        }
    }


    fn follow_head(&self, world: &mut B2World, cat: &state::Cat) {
        let p2_iter = cat.path.iter().copied()
            .zip(cat.path.iter().copied().skip(1));
        let link_iter = self.links.iter().copied();
        let follow_iter = link_iter.rev().skip(1).zip(p2_iter.rev());

        let mut stretch = 0.0;
        for (link, (p, p2)) in follow_iter {
            let d = p2 - p;
            let length = d.length();
            stretch += length - 0.1;
            if length >= std::f32::EPSILON * 1000.0 {
                let adjustment = stretch.max(0.0).min(length);
                let mut body = world.body_mut(link);
                let angle = body.angle();
                body.set_transform(&to_bvec(p + d * adjustment / length), angle);
                stretch -= adjustment;
            }
        }
    }


    fn make_control_iter<'a>(muscles: &'a VecDeque<JointHandle>, cat: &'a state::Cat)
        -> impl Iterator<Item = (usize, JointHandle, f32, Vec2)> + Clone + 'a
    {
        let p3_iter = cat.path.iter().copied()
            .zip(cat.path.iter().copied().skip(1))
            .zip(cat.path.iter().copied().skip(2));
        let angle_iter = p3_iter.map(|((p, p2), p3)| {
            let d = p2 - p;
            let d2 = p3 - p2;
            if d.length() >= std::f32::EPSILON * 1000.0 && d2.length() >= std::f32::EPSILON * 1000.0 {
                d2.unrotated(d.normalized()).to_angle()
            }
            else { 0.0 }
        });
        let muscle_iter = muscles.iter().copied();
        let p_iter = cat.path.iter().copied();
        muscle_iter.rev()
            .zip(angle_iter.rev())
            .zip(p_iter.rev().skip(1))
            .enumerate()
            .map(|(n, ((muscle, angle), p))| (n, muscle, angle, p))
    }


    fn control_relaxed<I: Iterator<Item = (usize, JointHandle, f32, Vec2)>>(
        world: &mut B2World,
        control_iter: &mut I
    ) {
        for (_, muscle, _, _) in control_iter {
            let mut joint = world.joint_mut(muscle);
            let motor = match &mut **joint {
                b2::UnknownJoint::Motor(motor) => motor,
                _ => unreachable!()
            };
            motor.set_angular_offset(0.0);
            motor.set_max_torque(10.0);
        }
    }


    fn control_movement<I: Iterator<Item = (usize, JointHandle, f32, Vec2)> + Clone>(
        world: &mut B2World,
        cat: &state::Cat,
        control_iter: &mut I
    ) {
        let direction = cat.direction.unwrap();
        let mut head_p = cat.path.back().copied().unwrap();
        if let Some((n, _, _, _)) = control_iter.clone().next() {
            if n >= cat.path.len() * 2 / 3 {
                return;
            }
        }
        for (n, muscle, angle, p) in control_iter {
            let mut joint = world.joint_mut(muscle);
            let motor = match &mut **joint {
                b2::UnknownJoint::Motor(motor) => motor,
                _ => unreachable!()
            };
            let head_lp = head_p - p;
            if head_lp.length_squared() < 1000.0 * std::f32::EPSILON * std::f32::EPSILON {
                motor.set_angular_offset(0.0);
                motor.set_max_torque(10.0);
                continue;
            }
            let offset = wrap_angle(angle + direction - head_lp.to_angle())
                .max(-std::f32::consts::PI * 0.06).min(std::f32::consts::PI * 0.06);
            let mut factor = 1.0;
            if n > cat.path.len() / 3 {
                factor = (cat.path.len() * 2 / 3 - n) as f32 * 3.0 / cat.path.len() as f32;
            }
            motor.set_angular_offset(offset * factor);
            motor.set_max_torque(10.0 + 100.0 * factor);
            head_p = head_lp.rotated(Vec2::from_angle(offset - angle)) + p;
            if n >= cat.path.len() * 2 / 3 {
                break;
            }
        }
    }
}