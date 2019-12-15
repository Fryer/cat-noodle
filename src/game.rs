use std::error::Error;
use std::sync::mpsc;

mod renderer;
use renderer::Renderer;


pub mod input {
    pub enum Event {
        Close,
        Key(glfw::Action, glfw::Key)
    }
}


pub struct Game {
    event_receiver: mpsc::Receiver<input::Event>,
    renderer: Renderer
}


impl Game {
    pub fn new(event_receiver: mpsc::Receiver<input::Event>) -> Result<Game, Box<dyn Error>> {
        Ok(Game {
            event_receiver,
            renderer: Renderer::new()?
        })
    }


    pub fn update(&mut self) -> Result<bool, Box<dyn Error>> {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                input::Event::Close => {
                    println!("close");
                    return Ok(false);
                }
                input::Event::Key(action, key) => {
                    println!("key {:?}: {:?}", action, key);
                }
            }
        }
        self.renderer.render()?;
        Ok(true)
    }
}
