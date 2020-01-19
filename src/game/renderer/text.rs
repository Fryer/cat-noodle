extern crate freetype;

use std::rc::Rc;
use std::collections::HashMap;

use freetype as ft;

use lib::rgl;
use lib::math::{Vec2, vec2};

use super::vertex::Vertex;


pub struct Library {
    library: Rc<ft::Library>
}

struct Glyph {
    tex_tl: Vec2,
    tex_br: Vec2,
    size: Vec2,
    offset: Vec2,
    advance: f32
}

pub struct Font {
    _library: Rc<ft::Library>,
    face: ft::Face,
    atlas: rgl::Texture,
    glyphs: HashMap<usize, Glyph>
}

pub struct Text {
    vertex_data: Vec<Vertex>,
    vertex_array: Option<rgl::VertexArray>,
    vertices: usize
}


impl Library {
    pub fn new() -> Library {
        Library { library: Rc::new(ft::Library::init().unwrap()) }
    }


    pub fn new_font(&self, file: &str, size: isize) -> Font {
        let face = self.library.new_face(file, 0).unwrap();
        face.set_char_size(0, size * 64, 0, 0).unwrap();
        let metrics = face.size_metrics().unwrap();

        let mut atlas_data: Vec<u8> = [255, 255, 255, 0].iter().copied().cycle().take(1024 * 1024 * 4).collect();

        // TODO: Use a fast hasher.
        let mut glyphs = HashMap::new();

        let glyph = face.glyph();
        let bitmap = glyph.bitmap();
        let mut x = 0;
        let mut y = 0;
        for c in 32..=126 {
            if face.load_char(c, ft::face::LoadFlag::RENDER).is_err() {
                continue;
            }
            if x + bitmap.width() as usize >= 1024 {
                x = 0;
                y += metrics.height as usize / 64 + 1;
            }
            Self::blit_glyph(atlas_data.as_mut_slice(), &bitmap, x, y);
            let tex_tl = vec2(x as f32 - 0.5, y as f32 - 0.5) / 1024.0;
            let size = vec2(bitmap.width() as f32 + 1.0, -bitmap.rows() as f32 - 1.0);
            let glyph = Glyph {
                tex_tl,
                tex_br: tex_tl + vec2(size.x, -size.y) / 1024.0,
                size,
                offset: vec2(glyph.bitmap_left() as f32 - 0.5, glyph.bitmap_top() as f32 - 0.5),
                advance: (glyph.advance().x / 64) as _
            };
            x += bitmap.width() as usize + 1;
            glyphs.insert(c, glyph);
        }

        let mut atlas = rgl::Texture::new().unwrap();
        atlas.set_data(atlas_data.as_slice(), 1024, 1024).unwrap();

        Font {
            _library: self.library.clone(),
            face,
            atlas,
            glyphs
        }
    }


    fn blit_glyph(atlas: &mut [u8], bitmap: &ft::Bitmap, x: usize, y: usize) {
        assert_eq!(bitmap.pixel_mode().unwrap(), ft::bitmap::PixelMode::Gray);
        assert_eq!(bitmap.raw().num_grays, 256);
        // TODO: Go up instead if the pitch is negative.
        assert!(bitmap.pitch() >= 0);
        let bitmap_buffer = bitmap.buffer();
        let mut atlas_p = (x + y * 1024) * 4 + 3;
        let mut bitmap_p = 0;
        for _ in 0..bitmap.rows() {
            for _ in 0..bitmap.width() {
                atlas[atlas_p] = bitmap_buffer[bitmap_p];
                atlas_p += 4;
                bitmap_p += 1;
            }
            atlas_p += (1024 - bitmap.width() as usize) * 4;
            bitmap_p += (bitmap.pitch() - bitmap.width()) as usize;
        }
    }
}


impl Font {
    pub fn bind(&self, unit: u32) -> Result<(), rgl::GLError> {
        self.atlas.bind(unit)
    }


    pub fn height(&self) -> f32 {
        (self.face.size_metrics().unwrap().height / 64) as _
    }
}


impl Text {
    pub fn new() -> Text {
        Text {
            vertex_data: Vec::new(),
            vertex_array: None,
            vertices: 0
        }
    }


    pub fn add_text(&mut self, font: &Font, text: &str, position: Vec2) {
        self.vertex_data.reserve(text.len() * 6);

        let mut cursor = position + vec2(0.0, -(font.face.size_metrics().unwrap().ascender / 64) as _);
        for c in text.chars() {
            let glyph = &font.glyphs[&(c as usize)];
            if glyph.size == vec2(0.0, 0.0) {
                cursor.x += glyph.advance;
                continue;
            }
            let tl = cursor + glyph.offset;
            let br = tl + glyph.size;
            cursor.x += glyph.advance;
            self.vertex_data.extend([
                Vertex::new(vec2(tl.x, tl.y), vec2(glyph.tex_tl.x, glyph.tex_tl.y)),
                Vertex::new(vec2(tl.x, br.y), vec2(glyph.tex_tl.x, glyph.tex_br.y)),
                Vertex::new(vec2(br.x, br.y), vec2(glyph.tex_br.x, glyph.tex_br.y)),
                Vertex::new(vec2(tl.x, tl.y), vec2(glyph.tex_tl.x, glyph.tex_tl.y)),
                Vertex::new(vec2(br.x, br.y), vec2(glyph.tex_br.x, glyph.tex_br.y)),
                Vertex::new(vec2(br.x, tl.y), vec2(glyph.tex_br.x, glyph.tex_tl.y))
            ].into_iter());
        }
    }


    pub fn update(&mut self, stream: bool) -> Result<(), rgl::GLError> {
        let usage = if stream { rgl::BufferUsage::StreamDraw } else { rgl::BufferUsage::StaticDraw };
        self.vertex_array = Some(Vertex::create_array(self.vertex_data.as_slice(), usage)?);
        self.vertices = self.vertex_data.len();
        self.vertex_data.clear();
        Ok(())
    }


    pub fn render(&self) -> Result<(), rgl::GLError> {
        if let Some(vertex_array) = &self.vertex_array {
            vertex_array.bind()?;
            rgl::draw(rgl::DrawMode::Triangles, 0, self.vertices as _)?;
        }
        Ok(())
    }
}
