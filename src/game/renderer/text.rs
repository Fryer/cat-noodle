extern crate freetype;

use std::rc::Rc;
use std::mem::ManuallyDrop;
use std::collections::HashMap;

use freetype as ft;
use harfbuzz_rs as hb;

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
    library: ManuallyDrop<Rc<ft::Library>>,
    face: ManuallyDrop<ft::Face>,
    font: ManuallyDrop<hb::Owned<hb::Font<'static>>>,
    atlas: rgl::Texture,
    glyphs: HashMap<u32, Glyph>
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
        let mut face = self.library.new_face(file, 0).unwrap();
        face.set_char_size(0, size * 64, 0, 0).unwrap();
        let metrics = face.size_metrics().unwrap();

        let mut atlas_data: Vec<u8> = std::iter::repeat(0).take(1024 * 1024 * 4).collect();

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
            x += bitmap.width() as usize + 1;
            glyphs.insert(face.get_char_index(c), Glyph {
                tex_tl,
                tex_br: tex_tl + vec2(size.x, -size.y) / 1024.0,
                size,
                offset: vec2(glyph.bitmap_left() as f32 - 0.5, glyph.bitmap_top() as f32 + 0.5),
                advance: (glyph.advance().x / 64) as _
            });
        }

        let mut atlas = rgl::Texture::new().unwrap();
        atlas.set_data(atlas_data.as_slice(), 1024, 1024).unwrap();

        extern "C" {
            fn hb_ft_font_create(ft_face: ft::ffi::FT_Face, _: hb::hb::hb_destroy_func_t) -> *mut hb::hb::hb_font_t;
        }
        let font = unsafe {
            let raw_font = hb_ft_font_create(face.raw_mut() as _, hb::hb::hb_destroy_func_t::default());
            hb::Owned::from_raw(raw_font)
        };

        Font {
            library: ManuallyDrop::new(self.library.clone()),
            face: ManuallyDrop::new(face),
            font: ManuallyDrop::new(font),
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
        let mut atlas_p = (x + y * 1024) * 4;
        let mut bitmap_p = 0;
        for _ in 0..bitmap.rows() {
            for _ in 0..bitmap.width() {
                let v = bitmap_buffer[bitmap_p];
                atlas[atlas_p] = v;
                atlas[atlas_p + 1] = v;
                atlas[atlas_p + 2] = v;
                atlas[atlas_p + 3] = v;
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


impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.font);
            ManuallyDrop::drop(&mut self.face);
            ManuallyDrop::drop(&mut self.library);
        }
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
            let glyph = &font.glyphs[&font.face.get_char_index(c as _)];
            if glyph.size == vec2(1.0, -1.0) {
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


    pub fn add_hb_text(&mut self, font: &Font, text: &str, position: Vec2) {
        self.vertex_data.reserve(text.len() * 6);

        let mut buffer = hb::UnicodeBuffer::new().add_str(text);
        buffer = buffer.guess_segment_properties();
        let glyphs = hb::shape(&font.font, buffer, &[]);
        let positions = glyphs.get_glyph_positions();
        let infos = glyphs.get_glyph_infos();
        let glyph_iter = positions.iter().zip(infos);

        let mut cursor = position + vec2(0.0, -(font.face.size_metrics().unwrap().ascender / 64) as _);
        for (position, info) in glyph_iter {
            let glyph = &font.glyphs[&info.codepoint];
            if glyph.size == vec2(1.0, -1.0) {
                cursor += vec2((position.x_advance / 64) as _, (position.y_advance / 64) as _);
                continue;
            }
            let tl = cursor + glyph.offset + vec2((position.x_offset / 64) as _, (position.y_offset / 64) as _);
            let br = tl + glyph.size;
            cursor += vec2((position.x_advance / 64) as _, (position.y_advance / 64) as _);
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
