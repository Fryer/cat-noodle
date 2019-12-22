use nalgebra::{Vector2, Isometry2};
use nphysics2d::object::{
    DefaultBodySet,
    DefaultColliderSet,
    DefaultBodyHandle,
    DefaultColliderHandle,
    RigidBodyDesc,
    BodyStatus,
    ColliderDesc,
    BodyPartHandle
};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use ncollide2d::shape::{
    ShapeHandle,
    Cuboid,
    Ball
};

use super::state;


pub struct World {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    joints: DefaultJointConstraintSet<f32>,
    forces: DefaultForceGeneratorSet<f32>,
    ground: DefaultBodyHandle,
    cat: DefaultBodyHandle,
    cat_head: DefaultColliderHandle
}


impl World {
    pub fn new() -> World {
        let mut bodies = DefaultBodySet::new();
        let mut colliders = DefaultColliderSet::new();

        let ground = bodies.insert(
            RigidBodyDesc::new()
                .status(BodyStatus::Static)
                .build()
        );

        let cat = bodies.insert(
            RigidBodyDesc::new()
                .status(BodyStatus::Dynamic)
                .build()
        );
        let cat_head = colliders.insert(
            ColliderDesc::new(ShapeHandle::new(Ball::new(0.49)))
                .build(BodyPartHandle(cat, 0))
        );

        World {
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, -10.0)),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies,
            colliders,
            joints: DefaultJointConstraintSet::new(),
            forces: DefaultForceGeneratorSet::new(),
            ground,
            cat,
            cat_head
        }
    }


    pub fn step(&mut self, state: &mut state::State, delta_time: f32) {
        let ground = &mut state.ground;
        if ground.dirty.contains(state::DirtyFlags::PHYSICS) {
            self.bodies.remove(self.ground);
            self.ground = self.bodies.insert(
                RigidBodyDesc::new()
                    .status(BodyStatus::Static)
                    .build()
            );
            let shape = ShapeHandle::new(Cuboid::new(Vector2::repeat(0.49)));
            for p in ground.boxes.iter() {
                self.colliders.insert(
                    ColliderDesc::new(shape.clone())
                        .translation(Vector2::new(p.x, p.y))
                        .build(BodyPartHandle(self.ground, 0))
                );
            }
            ground.dirty -= state::DirtyFlags::PHYSICS;
        }

        let cat = &mut state.cat;
        let cat_body = self.bodies.rigid_body_mut(self.cat).unwrap();
        cat_body.set_position(Isometry2::translation(cat.position.x, cat.position.y));

        self.mechanical_world.set_timestep(delta_time);
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.forces
        );

        for event in self.geometrical_world.contacts_with(&self.colliders, self.cat_head, true).unwrap() {
            let manifold = event.5;
            let normal = -manifold.deepest_contact().unwrap().contact.normal;
            if normal.x < -normal.y.abs() {
                cat.direction.x = -cat.direction.x.abs();
            } else if normal.y < -normal.x.abs() {
                cat.direction.y = -cat.direction.y.abs();
            } else if normal.x > normal.y.abs() {
                cat.direction.x = cat.direction.x.abs();
            } else if normal.y > normal.x.abs() {
                cat.direction.y = cat.direction.y.abs();
            }
        }
    }
}
