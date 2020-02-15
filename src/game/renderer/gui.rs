use std::error::Error;

use lib::rgl;
use lib::math::vec2;

use super::state;
use super::text::{self, Font, Text};


pub struct GUI {
    font: Font,
    text: Text
}


impl GUI {
    pub fn new(library: &text::Library) -> Result<GUI, Box<dyn Error>> {
        Ok(GUI {
            font: library.new_font("font/Roboto-Bold.ttf", 36)?,
            text: Text::new()
        })
    }


    pub fn update(&mut self, gui: &state::GUI) -> Result<(), rgl::GLError> {
        let energy = format!("Calories: {}", gui.cat_energy);
        self.text.add_text_rgb(&self.font, energy.as_str(), vec2(402.0, -101.0), 0, 0, 0);
        self.text.add_text(&self.font, energy.as_str(), vec2(400.0, -100.0));
        self.text.update(true)?;
        Ok(())
    }


    pub fn render_text(&self) -> Result<(), rgl::GLError> {
        self.font.bind(0)?;
        self.text.render()?;
        Ok(())
    }
}
