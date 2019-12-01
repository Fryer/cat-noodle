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


    pub fn update(&mut self, path: &[(f32, f32)]) -> Result<(), rgl::GLError> {
        let mut vertices: Vec<Vertex> = Vec::with_capacity(path.len() * 6 + 6);

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

        let (x, y) = path[0];
        let (mut dx, mut dy) = direction(x, y, path.get(1), (0.5, 0.0));
        vertices.extend([
            Vertex::new(x - dx - dy, y - dy + dx, 0.0, 0.0),
            Vertex::new(x - dx + dy, y - dy - dx, 0.0, 0.5),
            Vertex::new(x + dy, y - dx, 0.25, 0.5),
            Vertex::new(x - dx - dy, y - dy + dx, 0.0, 0.0),
            Vertex::new(x + dy, y - dx, 0.25, 0.5),
            Vertex::new(x - dy, y + dx, 0.25, 0.0),
        ].into_iter());

        for (n, ((x, y), (x2, y2))) in path.iter().zip(path.iter().skip(1)).enumerate() {
            let (dx2, dy2) = direction(*x2, *y2, path.get(n + 2), (dx, dy));
            vertices.extend([
                Vertex::new(x - dy, y + dx, 0.25, 0.0),
                Vertex::new(x + dy, y - dx, 0.25, 0.5),
                Vertex::new(x2 + dy2, y2 - dx2, 0.25, 0.5),
                Vertex::new(x - dy, y + dx, 0.25, 0.0),
                Vertex::new(x2 + dy2, y2 - dx2, 0.25, 0.5),
                Vertex::new(x2 - dy2, y2 + dx2, 0.25, 0.0),
            ].into_iter());
            dx = dx2;
            dy = dy2;
        }

        let (x, y) = *path.last().unwrap();
        vertices.extend([
            Vertex::new(x - dy, y + dx, 0.25, 0.0),
            Vertex::new(x + dy, y - dx, 0.25, 0.5),
            Vertex::new(x + dx + dy, y + dy - dx, 0.5, 0.5),
            Vertex::new(x - dy, y + dx, 0.25, 0.0),
            Vertex::new(x + dx + dy, y + dy - dx, 0.5, 0.5),
            Vertex::new(x + dx - dy, y + dy + dx, 0.5, 0.0),
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