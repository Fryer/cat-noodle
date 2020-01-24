extern crate freetype;

use std::{
    error::Error,
    fmt,
    rc::Rc,
    mem::ManuallyDrop,
    collections::HashMap
};

use freetype as ft;
use harfbuzz_rs as hb;

use lib::rgl;
use lib::math::{Vec2, vec2};

use super::vertex::Vertex;


pub struct TextError {
    error: &'static str
}

pub struct Library {
    library: Rc<ft::Library>
}

struct Glyph {
    tex_tl: Vec2,
    tex_br: Vec2,
    size: Vec2,
    offset: Vec2
}

pub struct Font {
    library: ManuallyDrop<Rc<ft::Library>>,
    face: ManuallyDrop<ft::Face>,
    font: ManuallyDrop<hb::Owned<hb::Font<'static>>>,
    atlas: rgl::Texture,
    glyphs: HashMap<u32, Glyph>,
    height: f32
}

pub struct Text {
    vertex_data: Vec<Vertex>,
    vertex_array: Option<rgl::VertexArray>,
    vertices: usize
}


impl Error for TextError {}


impl fmt::Debug for TextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}


impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}


impl Library {
    pub fn new() -> Result<Library, Box<dyn Error>> {
        Ok(Library { library: Rc::new(ft::Library::init()?) })
    }


    pub fn new_font(&self, file: &str, size: isize) -> Result<Font, Box<dyn Error>> {
        let mut face = self.library.new_face(file, 0)?;
        face.set_char_size(0, size * 64, 0, 0)?;
        let metrics = match face.size_metrics() {
            Some(metrics) => metrics,
            None => return Err(TextError { error: "face doesn't have size metrics" }.into())
        };
        let ascender = -(metrics.ascender / 64) as f32;

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
            Self::blit_glyph(atlas_data.as_mut_slice(), &bitmap, x, y)?;
            let tex_tl = vec2(x as f32 - 0.5, y as f32 - 0.5) / 1024.0;
            let size = vec2(bitmap.width() as f32 + 1.0, -bitmap.rows() as f32 - 1.0);
            x += bitmap.width() as usize + 1;
            glyphs.insert(face.get_char_index(c), Glyph {
                tex_tl,
                tex_br: tex_tl + vec2(size.x, -size.y) / 1024.0,
                size,
                offset: vec2(glyph.bitmap_left() as f32 - 0.5, ascender + glyph.bitmap_top() as f32 + 0.5)
            });
        }

        let mut atlas = rgl::Texture::new()?;
        atlas.set_data(atlas_data.as_slice(), 1024, 1024)?;

        extern "C" {
            fn hb_ft_font_create(ft_face: ft::ffi::FT_Face, _: hb::hb::hb_destroy_func_t) -> *mut hb::hb::hb_font_t;
        }
        let font = unsafe {
            let raw_font = hb_ft_font_create(face.raw_mut() as _, hb::hb::hb_destroy_func_t::default());
            hb::Owned::from_raw(raw_font)
        };

        Ok(Font {
            library: ManuallyDrop::new(self.library.clone()),
            face: ManuallyDrop::new(face),
            font: ManuallyDrop::new(font),
            atlas,
            glyphs,
            height: (metrics.height / 64) as _
        })
    }


    fn blit_glyph(atlas: &mut [u8], bitmap: &ft::Bitmap, x: usize, y: usize) -> Result<(), Box<dyn Error>> {
        if bitmap.pixel_mode()? != ft::bitmap::PixelMode::Gray {
            return Err(TextError { error: "bitmap pixel mode is not gray" }.into());
        }
        if bitmap.raw().num_grays != 256 {
            return Err(TextError { error: "bitmap gray levels is not 256" }.into());
        }
        // TODO: Go up instead if the pitch is negative.
        if bitmap.pitch() < 0 {
            return Err(TextError { error: "bitmap pitch is negative" }.into());
        }
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
        Ok(())
    }
}


impl Font {
    pub fn bind(&self, unit: u32) -> Result<(), rgl::GLError> {
        self.atlas.bind(unit)
    }


    pub fn height(&self) -> f32 {
        self.height
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


    pub fn add_text(&mut self, font: &Font, text: &str, mut position: Vec2) {
        self.vertex_data.reserve(text.len() * 6);

        let buffer = hb::UnicodeBuffer::new()
            .add_str(text)
            .set_direction(hb::Direction::Ltr)
            .set_script("Latn".parse().unwrap())
            .set_language("en".parse().unwrap());
        let glyphs = hb::shape(&font.font, buffer, &[]);
        let positions = glyphs.get_glyph_positions();
        let infos = glyphs.get_glyph_infos();
        let glyph_iter = positions.iter().zip(infos);

        for (glyph_p, info) in glyph_iter {
            let glyph = &font.glyphs[&info.codepoint];
            if glyph.size == vec2(1.0, -1.0) {
                position += vec2((glyph_p.x_advance / 64) as _, (glyph_p.y_advance / 64) as _);
                continue;
            }
            let tl = position + glyph.offset + vec2((glyph_p.x_offset / 64) as _, (glyph_p.y_offset / 64) as _);
            let br = tl + glyph.size;
            position += vec2((glyph_p.x_advance / 64) as _, (glyph_p.y_advance / 64) as _);
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
