use rgl;

use super::vertex::{self, Vertex};


pub struct NoodleCat {
    vertex_array: rgl::VertexArray,
    vertices: usize
}


impl NoodleCat {
    pub fn new() -> Result<NoodleCat, rgl::GLError> {
        let vertices = [
            Vertex::new(-0.5, 0.5, 0.0, 0.0), Vertex::new(-0.5, -0.5, 0.0, 0.5), Vertex::new(0.5, -0.5, 0.5, 0.5),
            Vertex::new(-0.5, 0.5, 0.0, 0.0), Vertex::new(0.5, -0.5, 0.5, 0.5), Vertex::new(0.5, 0.5, 0.5, 0.0)
        ];
        let vertex_array = vertex::create_array(&vertices, rgl::BufferUsage::StreamDraw)?;

        Ok(NoodleCat {
            vertex_array,
            vertices: 6
        })
    }


    pub fn update(&mut self, path: &[(f32, f32)], tail: &[(f32, f32)]) -> Result<(), rgl::GLError> {
        let mut vertices: Vec<Vertex> = Vec::with_capacity((path.len() + tail.len() + 11) * 6);

        fn direction(x: f32, y: f32, target: Option<&(f32, f32)>, default: (f32, f32)) -> (f32, f32) {
            match target {
                Some((x2, y2)) => {
                    let (dx, dy) = (x2 - x, y2 - y);
                    let d = dx.hypot(dy);
                    if d == 0.0 { default }
                    else { (0.5 * dx / d, 0.5 * dy / d) }
                }
                None => default
            }
        }

        let (x, y) = *path.last().unwrap();
        let (mut dx, mut dy) = direction(x, y, path.get(path.len().wrapping_sub(2)), (-0.5, 0.0));
        dx = -dx;
        dy = -dy;
        let flip = if dx < 0.0 { -1.0 } else { 1.0 };

        // Far ear.
        let ear_x = x - dy * 0.8 * flip;
        let ear_y = y + dx * 0.8 * flip;
        let ear_dx = -dy * flip;
        let ear_dy = dx * flip;
        vertices.extend([
            Vertex::rgb(ear_x - dx * 0.5 + ear_dx, ear_y - dy * 0.5 + ear_dy, 0.125, 0.625, 127, 127, 127),
            Vertex::rgb(ear_x - dx * 0.5, ear_y - dy * 0.5, 0.125, 0.875, 127, 127, 127),
            Vertex::rgb(ear_x + dx * 0.5, ear_y + dy * 0.5, 0.375, 0.875, 127, 127, 127),
            Vertex::rgb(ear_x - dx * 0.5 + ear_dx, ear_y - dy * 0.5 + ear_dy, 0.125, 0.625, 127, 127, 127),
            Vertex::rgb(ear_x + dx * 0.5, ear_y + dy * 0.5, 0.375, 0.875, 127, 127, 127),
            Vertex::rgb(ear_x + dx * 0.5 + ear_dx, ear_y + dy * 0.5 + ear_dy, 0.375, 0.625, 127, 127, 127)
        ].into_iter());

        // Far front paw.
        let paw_x = x - dx * 0.4 + dy * flip;
        let paw_y = y - dy * 0.4 - dx * flip;
        vertices.extend([
            Vertex::rgb(paw_x - 0.2, paw_y + 0.2, 0.625, 0.125, 127, 127, 127),
            Vertex::rgb(paw_x - 0.2, paw_y - 0.2, 0.625, 0.375, 127, 127, 127),
            Vertex::rgb(paw_x + 0.2, paw_y - 0.2, 0.875, 0.375, 127, 127, 127),
            Vertex::rgb(paw_x - 0.2, paw_y + 0.2, 0.625, 0.125, 127, 127, 127),
            Vertex::rgb(paw_x + 0.2, paw_y - 0.2, 0.875, 0.375, 127, 127, 127),
            Vertex::rgb(paw_x + 0.2, paw_y + 0.2, 0.875, 0.125, 127, 127, 127)
        ].into_iter());

        let (x, y) = path[0];
        let (mut dx, mut dy) = direction(x, y, path.get(1), (0.5, 0.0));
        let flip = if dx < 0.0 { -1.0 } else { 1.0 };

        // Far back paw.
        let paw_x = x + dx * 0.4 + dy * flip;
        let paw_y = y + dy * 0.4 - dx * flip;
        vertices.extend([
            Vertex::rgb(paw_x - 0.2, paw_y + 0.2, 0.625, 0.125, 127, 127, 127),
            Vertex::rgb(paw_x - 0.2, paw_y - 0.2, 0.625, 0.375, 127, 127, 127),
            Vertex::rgb(paw_x + 0.2, paw_y - 0.2, 0.875, 0.375, 127, 127, 127),
            Vertex::rgb(paw_x - 0.2, paw_y + 0.2, 0.625, 0.125, 127, 127, 127),
            Vertex::rgb(paw_x + 0.2, paw_y - 0.2, 0.875, 0.375, 127, 127, 127),
            Vertex::rgb(paw_x + 0.2, paw_y + 0.2, 0.875, 0.125, 127, 127, 127)
        ].into_iter());

        // Butt.
        vertices.extend([
            Vertex::new(x - dx - dy, y - dy + dx, 0.0, 0.0),
            Vertex::new(x - dx + dy, y - dy - dx, 0.0, 0.5),
            Vertex::new(x + dy, y - dx, 0.25, 0.5),
            Vertex::new(x - dx - dy, y - dy + dx, 0.0, 0.0),
            Vertex::new(x + dy, y - dx, 0.25, 0.5),
            Vertex::new(x - dy, y + dx, 0.25, 0.0)
        ].into_iter());

        // Tail.
        let (tail_x, tail_y) = tail[0];
        let (mut tail_dx, mut tail_dy) = direction(tail_x, tail_y, tail.get(1), (0.5, 0.0));
        for (n, ((x, y), (x2, y2))) in tail.iter().zip(tail.iter().skip(1)).enumerate() {
            let (tail_dx2, tail_dy2) = direction(*x2, *y2, tail.get(n + 2), (tail_dx, tail_dy));
            // TODO: Offset the tail using its own root direction.
            let (x, y, x2, y2) = (x - dx * 0.8, y - dy * 0.8, x2 - dx * 0.8, y2 - dy * 0.8);
            let (dx, dy) = (tail_dx * 0.4, tail_dy * 0.4);
            let (dx2, dy2) = (tail_dx2 * 0.4, tail_dy2 * 0.4);
            vertices.extend([
                Vertex::new(x - dy, y + dx, 0.75, 0.125),
                Vertex::new(x + dy, y - dx, 0.75, 0.375),
                Vertex::new(x2 + dy2, y2 - dx2, 0.75, 0.375),
                Vertex::new(x - dy, y + dx, 0.75, 0.125),
                Vertex::new(x2 + dy2, y2 - dx2, 0.75, 0.375),
                Vertex::new(x2 - dy2, y2 + dx2, 0.75, 0.125)
            ].into_iter());
            tail_dx = tail_dx2;
            tail_dy = tail_dy2;
        }

        // Tail cap.
        let (tail_x, tail_y) = *tail.last().unwrap();
        let (tail_x, tail_y) = (tail_x - dx * 0.8, tail_y - dy * 0.8);
        let (tail_dx, tail_dy) = (tail_dx * 0.4, tail_dy * 0.4);
        vertices.extend([
            Vertex::new(tail_x - tail_dy, tail_y + tail_dx, 0.75, 0.125),
            Vertex::new(tail_x + tail_dy, tail_y - tail_dx, 0.75, 0.375),
            Vertex::new(tail_x + tail_dx + tail_dy, tail_y + tail_dy - tail_dx, 0.875, 0.375),
            Vertex::new(tail_x - tail_dy, tail_y + tail_dx, 0.75, 0.125),
            Vertex::new(tail_x + tail_dx + tail_dy, tail_y + tail_dy - tail_dx, 0.875, 0.375),
            Vertex::new(tail_x + tail_dx - tail_dy, tail_y + tail_dy + tail_dx, 0.875, 0.125)
        ].into_iter());

        // Body.
        for (n, ((x, y), (x2, y2))) in path.iter().zip(path.iter().skip(1)).enumerate() {
            let (dx2, dy2) = direction(*x2, *y2, path.get(n + 2), (dx, dy));
            vertices.extend([
                Vertex::new(x - dy, y + dx, 0.25, 0.0),
                Vertex::new(x + dy, y - dx, 0.25, 0.5),
                Vertex::new(x2 + dy2, y2 - dx2, 0.25, 0.5),
                Vertex::new(x - dy, y + dx, 0.25, 0.0),
                Vertex::new(x2 + dy2, y2 - dx2, 0.25, 0.5),
                Vertex::new(x2 - dy2, y2 + dx2, 0.25, 0.0)
            ].into_iter());
            dx = dx2;
            dy = dy2;
        }

        // Head.
        let (x, y) = *path.last().unwrap();
        vertices.extend([
            Vertex::new(x - dy, y + dx, 0.25, 0.0),
            Vertex::new(x + dy, y - dx, 0.25, 0.5),
            Vertex::new(x + dx + dy, y + dy - dx, 0.5, 0.5),
            Vertex::new(x - dy, y + dx, 0.25, 0.0),
            Vertex::new(x + dx + dy, y + dy - dx, 0.5, 0.5),
            Vertex::new(x + dx - dy, y + dy + dx, 0.5, 0.0)
        ].into_iter());

        let flip = if dx < 0.0 { -1.0 } else { 1.0 };
        
        // Eye.
        let eye_x = x + dx * 0.25 - dy * 0.25 * flip;
        let eye_y = y + dy * 0.25 + dx * 0.25 * flip;
        let pupil_x = x + dx * 0.375 - dy * 0.25 * flip;
        let pupil_y = y + dy * 0.375 + dx * 0.25 * flip;
        vertices.extend([
            // Eye ball.
            Vertex::new(eye_x - 0.2, eye_y + 0.2, 0.0, 0.0),
            Vertex::new(eye_x - 0.2, eye_y - 0.2, 0.0, 0.5),
            Vertex::new(eye_x + 0.2, eye_y - 0.2, 0.5, 0.5),
            Vertex::new(eye_x - 0.2, eye_y + 0.2, 0.0, 0.0),
            Vertex::new(eye_x + 0.2, eye_y - 0.2, 0.5, 0.5),
            Vertex::new(eye_x + 0.2, eye_y + 0.2, 0.5, 0.0),
            // Pupil.
            Vertex::new(pupil_x - 0.1, pupil_y + 0.1, 0.625, 0.625),
            Vertex::new(pupil_x - 0.1, pupil_y - 0.1, 0.625, 0.875),
            Vertex::new(pupil_x + 0.1, pupil_y - 0.1, 0.875, 0.875),
            Vertex::new(pupil_x - 0.1, pupil_y + 0.1, 0.625, 0.625),
            Vertex::new(pupil_x + 0.1, pupil_y - 0.1, 0.875, 0.875),
            Vertex::new(pupil_x + 0.1, pupil_y + 0.1, 0.875, 0.625)
        ].into_iter());

        // Mouth.
        let mouth_x = x + dx;
        let mouth_y = y + dy;
        let mouth_d = 1.0 / 5.0f32.sqrt();
        let mouth_dx = -mouth_d * dx * 2.0 + mouth_d * dy * flip;
        let mouth_dy = -mouth_d * dy * 2.0 - mouth_d * dx * flip;
        let mouth_x2 = mouth_x + mouth_dx * 0.5;
        let mouth_y2 = mouth_y + mouth_dy * 0.5;
        vertices.extend([
            // Line.
            Vertex::new(mouth_x - mouth_dy * 0.05, mouth_y + mouth_dx * 0.05, 0.75, 0.625),
            Vertex::new(mouth_x + mouth_dy * 0.05, mouth_y - mouth_dx * 0.05, 0.75, 0.875),
            Vertex::new(mouth_x2 + mouth_dy * 0.05, mouth_y2 - mouth_dx * 0.05, 0.75, 0.875),
            Vertex::new(mouth_x - mouth_dy * 0.05, mouth_y + mouth_dx * 0.05, 0.75, 0.625),
            Vertex::new(mouth_x2 + mouth_dy * 0.05, mouth_y2 - mouth_dx * 0.05, 0.75, 0.875),
            Vertex::new(mouth_x2 - mouth_dy * 0.05, mouth_y2 + mouth_dx * 0.05, 0.75, 0.625),
            // Cap.
            Vertex::new(mouth_x2 - mouth_dy * 0.05, mouth_y2 + mouth_dx * 0.05, 0.75, 0.625),
            Vertex::new(mouth_x2 + mouth_dy * 0.05, mouth_y2 - mouth_dx * 0.05, 0.75, 0.875),
            Vertex::new(mouth_x2 + mouth_dx * 0.05 + mouth_dy * 0.05,
                        mouth_y2 + mouth_dy * 0.05 - mouth_dx * 0.05, 0.875, 0.875),
            Vertex::new(mouth_x2 - mouth_dy * 0.05, mouth_y2 + mouth_dx * 0.05, 0.75, 0.625),
            Vertex::new(mouth_x2 + mouth_dx * 0.05 + mouth_dy * 0.05,
                        mouth_y2 + mouth_dy * 0.05 - mouth_dx * 0.05, 0.875, 0.875),
            Vertex::new(mouth_x2 + mouth_dx * 0.05 - mouth_dy * 0.05,
                        mouth_y2 + mouth_dy * 0.05 + mouth_dx * 0.05, 0.875, 0.625)
        ].into_iter());

        // Near ear.
        let ear_x = x - dx * 0.4 - dy * 0.8 * flip;
        let ear_y = y - dy * 0.4 + dx * 0.8 * flip;
        let ear_dx = -dy * flip;
        let ear_dy = dx * flip;
        vertices.extend([
            Vertex::new(ear_x - dx * 0.5 + ear_dx, ear_y - dy * 0.5 + ear_dy, 0.125, 0.625),
            Vertex::new(ear_x - dx * 0.5, ear_y - dy * 0.5, 0.125, 0.875),
            Vertex::new(ear_x + dx * 0.5, ear_y + dy * 0.5, 0.375, 0.875),
            Vertex::new(ear_x - dx * 0.5 + ear_dx, ear_y - dy * 0.5 + ear_dy, 0.125, 0.625),
            Vertex::new(ear_x + dx * 0.5, ear_y + dy * 0.5, 0.375, 0.875),
            Vertex::new(ear_x + dx * 0.5 + ear_dx, ear_y + dy * 0.5 + ear_dy, 0.375, 0.625)
        ].into_iter());

        // Near front paw.
        let paw_x = x + dy * flip;
        let paw_y = y - dx * flip;
        vertices.extend([
            Vertex::new(paw_x - 0.2, paw_y + 0.2, 0.625, 0.125),
            Vertex::new(paw_x - 0.2, paw_y - 0.2, 0.625, 0.375),
            Vertex::new(paw_x + 0.2, paw_y - 0.2, 0.875, 0.375),
            Vertex::new(paw_x - 0.2, paw_y + 0.2, 0.625, 0.125),
            Vertex::new(paw_x + 0.2, paw_y - 0.2, 0.875, 0.375),
            Vertex::new(paw_x + 0.2, paw_y + 0.2, 0.875, 0.125)
        ].into_iter());

        let (x, y) = path[0];
        let (dx, dy) = direction(x, y, path.get(1), (0.5, 0.0));
        let flip = if dx < 0.0 { -1.0 } else { 1.0 };

        // Near back paw.
        let paw_x = x + dy * flip;
        let paw_y = y - dx * flip;
        vertices.extend([
            Vertex::new(paw_x - 0.2, paw_y + 0.2, 0.625, 0.125),
            Vertex::new(paw_x - 0.2, paw_y - 0.2, 0.625, 0.375),
            Vertex::new(paw_x + 0.2, paw_y - 0.2, 0.875, 0.375),
            Vertex::new(paw_x - 0.2, paw_y + 0.2, 0.625, 0.125),
            Vertex::new(paw_x + 0.2, paw_y - 0.2, 0.875, 0.375),
            Vertex::new(paw_x + 0.2, paw_y + 0.2, 0.875, 0.125)
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
