use nalgebra::{Vector2, Point2};
use nphysics2d::object::{
    DefaultBodySet,
    DefaultColliderSet,
    DefaultBodyHandle,
    Ground,
    RigidBodyDesc,
    ColliderDesc,
    BodyPartHandle
};
use nphysics2d::joint::RevoluteConstraint;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{
    ShapeHandle,
    Cuboid,
    Ball
};
use ncollide2d::pipeline::CollisionGroups;

use super::state;


pub struct World {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    joints: DefaultJointConstraintSet<f32>,
    forces: DefaultForceGeneratorSet<f32>,
    ground: DefaultBodyHandle,
    cat_links: Vec<DefaultBodyHandle>
}


impl World {
    pub fn new(state: &state::State) -> World {
        let mut bodies = DefaultBodySet::new();
        let mut colliders = DefaultColliderSet::new();
        let mut joints = DefaultJointConstraintSet::new();

        let ground = bodies.insert(Ground::new());

        let cat_collision = CollisionGroups::new()
            .with_membership(&[1])
            .with_blacklist(&[1]);
        let path = &state.cat.path;
        let mut cat_links: Vec<DefaultBodyHandle> = Vec::with_capacity(path.len());
        let mut link = bodies.insert(
            RigidBodyDesc::new()
                .translation(Vector2::new(path[0].x, path[0].y))
                .mass(1.0)
                .angular_inertia(1.0.into())
                .build()
        );
        colliders.insert(
            ColliderDesc::new(ShapeHandle::new(Ball::new(0.49)))
                .collision_groups(cat_collision)
                .build(BodyPartHandle(link, 0))
        );
        cat_links.push(link);
        for (p, p2) in path.iter().copied().zip(path.iter().copied().skip(1)) {
            let next = bodies.insert(
                RigidBodyDesc::new()
                    .translation(Vector2::new(p2.x, p2.y))
                    .mass(1.0)
                    .angular_inertia(1.0.into())
                    .build()
            );
            colliders.insert(
                ColliderDesc::new(ShapeHandle::new(Ball::new(0.49)))
                    .collision_groups(cat_collision)
                    .build(BodyPartHandle(next, 0))
            );
            let d = p - p2;
            joints.insert(
                RevoluteConstraint::new(
                    BodyPartHandle(link, 0),
                    BodyPartHandle(next, 0),
                    Point2::origin(),
                    Point2::new(d.x, d.y)
                )
            );
            link = next;
            cat_links.push(link);
        }

        World {
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -10.0)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies,
            colliders,
            joints,
            forces: DefaultForceGeneratorSet::new(),
            ground,
            cat_links
        }
    }


    pub fn step(&mut self, state: &mut state::State, delta_time: f32) {
        let ground = &mut state.ground;
        if ground.dirty.contains(state::DirtyFlags::PHYSICS) {
            self.bodies.remove(self.ground);
            self.ground = self.bodies.insert(Ground::new());
            let shape = ShapeHandle::new(Cuboid::new(Vector2::repeat(0.49)));
            let collision = CollisionGroups::new()
                .with_membership(&[0]);
            for p in ground.boxes.iter() {
                self.colliders.insert(
                    ColliderDesc::new(shape.clone())
                        .translation(Vector2::new(p.x, p.y))
                        .collision_groups(collision)
                        .build(BodyPartHandle(self.ground, 0))
                );
            }
            ground.dirty -= state::DirtyFlags::PHYSICS;
        }

        let cat = &mut state.cat;

        self.mechanical_world.set_timestep(delta_time);
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.forces
        );

        for (p, link) in cat.path.iter_mut().zip(self.cat_links.iter().copied()) {
            let body = self.bodies.rigid_body(link).unwrap();
            p.x = body.position().translation.x;
            p.y = body.position().translation.y;
        }
    }
}
