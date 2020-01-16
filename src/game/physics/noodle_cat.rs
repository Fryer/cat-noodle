use wrapped2d::b2;
use wrapped2d::dynamics::world::{BodyHandle, JointHandle};
use wrapped2d::dynamics::body::FixtureHandle;

use lib::math::{Vec2, vec2, wrap_angle};

use super::B2World;
use super::{
    state,
    to_vec2,
    to_bvec,
    b2_get_local_point,
    evaluate_contact
};


pub struct NoodleCat {
    links: Vec<BodyHandle>,
    muscles: Vec<JointHandle>,
    tail_links: Vec<BodyHandle>,
    head_sensor: FixtureHandle,
    grab: Option<JointHandle>,
    grabbed: Option<BodyHandle>
}


impl NoodleCat {
    pub fn new(world: &mut B2World, cat: &state::Cat) -> NoodleCat {
        let path = &cat.path;
        let mut links: Vec<_> = Vec::with_capacity(path.len());
        let mut muscles: Vec<_> = Vec::with_capacity(path.len().saturating_sub(1));
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
        links.push(link);
        for (p, p2) in path.iter().copied().zip(path.iter().copied().skip(1)) {
            let next = world.create_body(
                &b2::BodyDef {
                    body_type: b2::BodyType::Dynamic,
                    position: to_bvec(p),
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
            links.push(link);
            muscles.push(muscle);
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
                    position: to_bvec(p),
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
            grabbed: None
        }
    }


    pub fn update(&self, cat: &mut state::Cat, world: &B2World) {
        for (p, link) in cat.path.iter_mut().zip(self.links.iter().copied()) {
            let body = world.body(link);
            p.x = body.position().x;
            p.y = body.position().y;
        }
        for (p, link) in cat.tail.iter_mut().zip(self.tail_links.iter().copied()) {
            let body = world.body(link);
            p.x = body.position().x;
            p.y = body.position().y;
        }
    }


    pub fn control(&mut self, world: &mut B2World, cat: &state::Cat) {
        let mut separation = std::f32::INFINITY;
        let mut other = None;
        let mut normal = vec2(0.0, 0.0);
        let head = self.links.last().copied().unwrap();
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
        if grab {
            if let Some(grab) = self.grab {
                world.destroy_joint(grab);
            }
            else {
                println!("grab");
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
                let d = Vec2::from_angle(direction) * 3.0 / 480.0;
                let tangent = normal.rotated(vec2(0.0, 1.0));
                other_anchor += tangent.dot(d) * tangent;
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
        }
        else if let Some(grab) = self.grab {
            println!("release");
            world.destroy_joint(grab);
            self.grab = None;
        }

        let mut control_iter = self.make_control_iter(cat);
        if grab {
            Self::control_relaxed(world, &mut control_iter);
            self.follow_head(world, &cat);
            return;
        }
        if let Some(direction) = cat.direction {
            let mut body = world.body_mut(*self.links.last().unwrap());
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
            }
        }
        Self::control_relaxed(world, &mut control_iter);
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
            if length >= std::f32::EPSILON {
                let adjustment = stretch.max(0.0).min(length);
                let mut body = world.body_mut(link);
                let angle = body.angle();
                body.set_transform(&to_bvec(p + d * adjustment / length), angle);
                stretch -= adjustment;
            }
        }
    }


    fn make_control_iter<'a>(&'a self, cat: &'a state::Cat)
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
        let muscle_iter = self.muscles.iter().copied();
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