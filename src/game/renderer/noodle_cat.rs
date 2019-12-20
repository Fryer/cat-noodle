use lib::rgl;
use lib::math::{Vec2, vec2};

use super::vertex::{self, Vertex};


pub struct NoodleCat {
    vertex_array: rgl::VertexArray,
    vertices: usize
}


impl NoodleCat {
    pub fn new() -> Result<NoodleCat, rgl::GLError> {
        let vertex_array = vertex::create_array(&[], rgl::BufferUsage::StreamDraw)?;

        Ok(NoodleCat {
            vertex_array,
            vertices: 0
        })
    }


    pub fn update<P, T>(&mut self, path: P, tail: T) -> Result<(), rgl::GLError>
        where
            P: ExactSizeIterator<Item = Vec2> + Clone,
            T: ExactSizeIterator<Item = Vec2> + Clone
    {
        let mut vertices: Vec<Vertex> = Vec::with_capacity((path.len() + tail.len() + 11) * 6);

        fn direction(p: Vec2, target: Option<Vec2>, default: Vec2) -> Vec2 {
            match target {
                Some(t) => {
                    let d = t - p;
                    if d.length() < std::f32::EPSILON * 1000.0 { default }
                    else { 0.5 * d.normalized() }
                }
                None => default
            }
        }

        let p = path.clone().last().unwrap();
        let d = -direction(p, path.clone().nth(path.len().wrapping_sub(2)), vec2(-0.5, 0.0));
        let flip = if d.x < 0.0 { -1.0 } else { 1.0 };

        // Far ear.
        let ear_p = p + vec2(0.0, 0.8 * flip).rotated(d);
        let ear_d = vec2(0.0, flip).rotated(d);
        vertices.extend([
            Vertex::rgb(ear_p - d * 0.5 + ear_d, (0.125, 0.625), 127, 127, 127),
            Vertex::rgb(ear_p - d * 0.5, (0.125, 0.875), 127, 127, 127),
            Vertex::rgb(ear_p + d * 0.5, (0.375, 0.875), 127, 127, 127),
            Vertex::rgb(ear_p - d * 0.5 + ear_d, (0.125, 0.625), 127, 127, 127),
            Vertex::rgb(ear_p + d * 0.5, (0.375, 0.875), 127, 127, 127),
            Vertex::rgb(ear_p + d * 0.5 + ear_d, (0.375, 0.625), 127, 127, 127)
        ].into_iter());

        // Far front paw.
        let paw_p = p + vec2(-0.4, -flip).rotated(d);
        vertices.extend([
            Vertex::rgb(paw_p - vec2(-0.2, 0.2), (0.625, 0.125), 127, 127, 127),
            Vertex::rgb(paw_p - vec2(-0.2, -0.2), (0.625, 0.375), 127, 127, 127),
            Vertex::rgb(paw_p - vec2(0.2, -0.2), (0.875, 0.375), 127, 127, 127),
            Vertex::rgb(paw_p - vec2(-0.2, 0.2), (0.625, 0.125), 127, 127, 127),
            Vertex::rgb(paw_p - vec2(0.2, -0.2), (0.875, 0.375), 127, 127, 127),
            Vertex::rgb(paw_p - vec2(0.2, 0.2), (0.875, 0.125), 127, 127, 127)
        ].into_iter());

        let p = path.clone().next().unwrap();
        let mut d = direction(p, path.clone().nth(1), vec2(0.5, 0.0));
        let flip = if d.x < 0.0 { -1.0 } else { 1.0 };

        // Far back paw.
        let paw_p = p + vec2(0.4, -flip).rotated(d);
        vertices.extend([
            Vertex::rgb(paw_p - vec2(-0.2, 0.2), (0.625, 0.125), 127, 127, 127),
            Vertex::rgb(paw_p - vec2(-0.2, -0.2), (0.625, 0.375), 127, 127, 127),
            Vertex::rgb(paw_p - vec2(0.2, -0.2), (0.875, 0.375), 127, 127, 127),
            Vertex::rgb(paw_p - vec2(-0.2, 0.2), (0.625, 0.125), 127, 127, 127),
            Vertex::rgb(paw_p - vec2(0.2, -0.2), (0.875, 0.375), 127, 127, 127),
            Vertex::rgb(paw_p - vec2(0.2, 0.2), (0.875, 0.125), 127, 127, 127)
        ].into_iter());

        // Butt.
        vertices.extend([
            Vertex::new(p + vec2(-1.0, 1.0).rotated(d), (0.0, 0.0)),
            Vertex::new(p + vec2(-1.0, -1.0).rotated(d), (0.0, 0.5)),
            Vertex::new(p + vec2(0.0, -1.0).rotated(d), (0.25, 0.5)),
            Vertex::new(p + vec2(-1.0, 1.0).rotated(d), (0.0, 0.0)),
            Vertex::new(p + vec2(0.0, -1.0).rotated(d), (0.25, 0.5)),
            Vertex::new(p + vec2(0.0, 1.0).rotated(d), (0.25, 0.0))
        ].into_iter());

        // Tail.
        let tail_p = tail.clone().next().unwrap();
        let mut tail_d = direction(tail_p, tail.clone().nth(1), vec2(0.5, 0.0));
        for (n, (p, p2)) in tail.clone().zip(tail.clone().skip(1)).enumerate() {
            let tail_d2 = direction(p2, tail.clone().nth(n + 2), tail_d);
            // TODO: Offset the tail using its own root direction.
            let p = p - d * 0.8;
            let p2 = p2 - d * 0.8;
            let d = tail_d * 0.4;
            let d2 = tail_d2 * 0.4;
            vertices.extend([
                Vertex::new(p + vec2(0.0, 1.0).rotated(d), (0.75, 0.125)),
                Vertex::new(p + vec2(0.0, -1.0).rotated(d), (0.75, 0.375)),
                Vertex::new(p2 + vec2(0.0, -1.0).rotated(d2), (0.75, 0.375)),
                Vertex::new(p + vec2(0.0, 1.0).rotated(d), (0.75, 0.125)),
                Vertex::new(p2 + vec2(0.0, -1.0).rotated(d2), (0.75, 0.375)),
                Vertex::new(p2 + vec2(0.0, 1.0).rotated(d2), (0.75, 0.125))
            ].into_iter());
            tail_d = tail_d2;
        }

        // Tail cap.
        let tail_p = tail.clone().last().unwrap() - d * 0.8;
        let tail_d = tail_d * 0.4;
        vertices.extend([
            Vertex::new(tail_p + vec2(0.0, 1.0).rotated(tail_d), (0.75, 0.125)),
            Vertex::new(tail_p + vec2(0.0, -1.0).rotated(tail_d), (0.75, 0.375)),
            Vertex::new(tail_p + vec2(1.0, -1.0).rotated(tail_d), (0.875, 0.375)),
            Vertex::new(tail_p + vec2(0.0, 1.0).rotated(tail_d), (0.75, 0.125)),
            Vertex::new(tail_p + vec2(1.0, -1.0).rotated(tail_d), (0.875, 0.375)),
            Vertex::new(tail_p + vec2(1.0, 1.0).rotated(tail_d), (0.875, 0.125))
        ].into_iter());

        // Body.
        for (n, (p, p2)) in path.clone().zip(path.clone().skip(1)).enumerate() {
            let d2 = direction(p, path.clone().nth(n + 2), d);
            vertices.extend([
                Vertex::new(p + vec2(0.0, 1.0).rotated(d), (0.25, 0.0)),
                Vertex::new(p + vec2(0.0, -1.0).rotated(d), (0.25, 0.5)),
                Vertex::new(p2 + vec2(0.0, -1.0).rotated(d2), (0.25, 0.5)),
                Vertex::new(p + vec2(0.0, 1.0).rotated(d), (0.25, 0.0)),
                Vertex::new(p2 + vec2(0.0, -1.0).rotated(d2), (0.25, 0.5)),
                Vertex::new(p2 + vec2(0.0, 1.0).rotated(d2), (0.25, 0.0))
            ].into_iter());
            d = d2;
        }

        // Head.
        let p = path.clone().last().unwrap();
        vertices.extend([
            Vertex::new(p + vec2(0.0, 1.0).rotated(d), (0.25, 0.0)),
            Vertex::new(p + vec2(0.0, -1.0).rotated(d), (0.25, 0.5)),
            Vertex::new(p + vec2(1.0, -1.0).rotated(d), (0.5, 0.5)),
            Vertex::new(p + vec2(0.0, 1.0).rotated(d), (0.25, 0.0)),
            Vertex::new(p + vec2(1.0, -1.0).rotated(d), (0.5, 0.5)),
            Vertex::new(p + vec2(1.0, 1.0).rotated(d), (0.5, 0.0))
        ].into_iter());

        let flip = if d.x < 0.0 { -1.0 } else { 1.0 };

        // Eye.
        let eye_p = p + vec2(0.25, 0.25 * flip).rotated(d);
        let pupil_p = p + vec2(0.375, 0.25 * flip).rotated(d);
        vertices.extend([
            // Eye ball.
            Vertex::new(eye_p + vec2(-0.2, 0.2), (0.0, 0.0)),
            Vertex::new(eye_p + vec2(-0.2, -0.2), (0.0, 0.5)),
            Vertex::new(eye_p + vec2(0.2, -0.2), (0.5, 0.5)),
            Vertex::new(eye_p + vec2(-0.2, 0.2), (0.0, 0.0)),
            Vertex::new(eye_p + vec2(0.2, -0.2), (0.5, 0.5)),
            Vertex::new(eye_p + vec2(0.2, 0.2), (0.5, 0.0)),
            // Pupil.
            Vertex::new(pupil_p + vec2(-0.1, 0.1), (0.625, 0.625)),
            Vertex::new(pupil_p + vec2(-0.1, -0.1), (0.625, 0.875)),
            Vertex::new(pupil_p + vec2(0.1, -0.1), (0.875, 0.875)),
            Vertex::new(pupil_p + vec2(-0.1, 0.1), (0.625, 0.625)),
            Vertex::new(pupil_p + vec2(0.1, -0.1), (0.875, 0.875)),
            Vertex::new(pupil_p + vec2(0.1, 0.1), (0.875, 0.625))
        ].into_iter());

        // Mouth.
        let mouth_p = p + d;
        let mouth_d = vec2(-2.0, -flip).normalized().rotated(d);
        let mouth_p2 = mouth_p + mouth_d * 0.5;
        vertices.extend([
            // Line.
            Vertex::new(mouth_p + vec2(0.0, 0.05).rotated(mouth_d), (0.75, 0.625)),
            Vertex::new(mouth_p + vec2(0.0, -0.05).rotated(mouth_d), (0.75, 0.875)),
            Vertex::new(mouth_p2 + vec2(0.0, -0.05).rotated(mouth_d), (0.75, 0.875)),
            Vertex::new(mouth_p + vec2(0.0, 0.05).rotated(mouth_d), (0.75, 0.625)),
            Vertex::new(mouth_p2 + vec2(0.0, -0.05).rotated(mouth_d), (0.75, 0.875)),
            Vertex::new(mouth_p2 + vec2(0.0, 0.05).rotated(mouth_d), (0.75, 0.625)),
            // Cap.
            Vertex::new(mouth_p2 + vec2(0.0, 0.05).rotated(mouth_d), (0.75, 0.625)),
            Vertex::new(mouth_p2 + vec2(0.0, -0.05).rotated(mouth_d), (0.75, 0.875)),
            Vertex::new(mouth_p2 + vec2(0.05, -0.05).rotated(mouth_d), (0.875, 0.875)),
            Vertex::new(mouth_p2 + vec2(0.0, 0.05).rotated(mouth_d), (0.75, 0.625)),
            Vertex::new(mouth_p2 + vec2(0.05, -0.05).rotated(mouth_d), (0.875, 0.875)),
            Vertex::new(mouth_p2 + vec2(0.05, 0.05).rotated(mouth_d), (0.875, 0.625))
        ].into_iter());

        // Near ear.
        let ear_p = p + vec2(-0.4, 0.8 * flip).rotated(d);
        let ear_d = vec2(0.0, flip).rotated(d);
        vertices.extend([
            Vertex::new(ear_p - d * 0.5 + ear_d, (0.125, 0.625)),
            Vertex::new(ear_p - d * 0.5, (0.125, 0.875)),
            Vertex::new(ear_p + d * 0.5, (0.375, 0.875)),
            Vertex::new(ear_p - d * 0.5 + ear_d, (0.125, 0.625)),
            Vertex::new(ear_p + d * 0.5, (0.375, 0.875)),
            Vertex::new(ear_p + d * 0.5 + ear_d, (0.375, 0.625))
        ].into_iter());

        // Near front paw.
        let paw_p = p + vec2(0.0, -flip).rotated(d);
        vertices.extend([
            Vertex::new(paw_p - vec2(-0.2, 0.2), (0.625, 0.125)),
            Vertex::new(paw_p - vec2(-0.2, -0.2), (0.625, 0.375)),
            Vertex::new(paw_p - vec2(0.2, -0.2), (0.875, 0.375)),
            Vertex::new(paw_p - vec2(-0.2, 0.2), (0.625, 0.125)),
            Vertex::new(paw_p - vec2(0.2, -0.2), (0.875, 0.375)),
            Vertex::new(paw_p - vec2(0.2, 0.2), (0.875, 0.125))
        ].into_iter());

        let p = path.clone().next().unwrap();
        let d = direction(p, path.clone().nth(1), vec2(0.5, 0.0));
        let flip = if d.x < 0.0 { -1.0 } else { 1.0 };

        // Near back paw.
        let paw_p = p + vec2(0.0, -flip).rotated(d);
        vertices.extend([
            Vertex::new(paw_p - vec2(-0.2, 0.2), (0.625, 0.125)),
            Vertex::new(paw_p - vec2(-0.2, -0.2), (0.625, 0.375)),
            Vertex::new(paw_p - vec2(0.2, -0.2), (0.875, 0.375)),
            Vertex::new(paw_p - vec2(-0.2, 0.2), (0.625, 0.125)),
            Vertex::new(paw_p - vec2(0.2, -0.2), (0.875, 0.375)),
            Vertex::new(paw_p - vec2(0.2, 0.2), (0.875, 0.125))
        ].into_iter());

        self.vertex_array.buffer.set_data(vertices.as_slice(), rgl::BufferUsage::StreamDraw)?;
        self.vertices = vertices.len();
        Ok(())
    }


    pub fn render(&self) -> Result<(), rgl::GLError> {
        self.vertex_array.bind()?;
        rgl::draw(self.vertices as _)?;
        Ok(())
    }
}
