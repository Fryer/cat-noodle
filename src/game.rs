use std::{
    error::Error,
    sync::mpsc,
    time,
    collections::VecDeque
};

mod renderer;
use renderer::Renderer;

mod state {
    use std::collections::VecDeque;
    
    
    pub struct Input {
        pub left: bool,
        pub right: bool,
        pub forward: bool
    }

    pub struct Cat {
        pub position: (f32, f32),
        pub direction: (f32, f32),
        pub path: VecDeque<(f32, f32)>,
        pub tail: VecDeque<(f32, f32)>
    }

    pub struct State {
        pub input: Input,
        pub cat: Cat
    }
}
use state::State;


pub enum Event {
    Close,
    Key(glfw::Action, glfw::Key)
}

pub struct Game {
    last_update: time::Instant,
    event_receiver: mpsc::Receiver<Event>,
    state: State,
    renderer: Renderer
}


impl Game {
    pub fn new(event_receiver: mpsc::Receiver<Event>) -> Result<Game, Box<dyn Error>> {
        let path: VecDeque<(f32, f32)> = (0..180).map(|x| (
            x as f32 * 0.1 - 18.0,
            0.0
        )).collect();
        let tail: VecDeque<(f32, f32)> = (0..20).map(|x| (
            x as f32 * -0.1 - 18.0,
            0.0
        )).collect();
        
        let state = State {
            input: state::Input {
                left: false,
                right: false,
                forward: false
            },
            cat: state::Cat {
                position: *path.back().unwrap(),
                direction: (1.0, 0.0),
                path,
                tail
            }
        };
        
        Ok(Game {
            last_update: time::Instant::now(),
            event_receiver,
            state,
            renderer: Renderer::new()?
        })
    }


    pub fn update(&mut self) -> Result<bool, Box<dyn Error>> {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                Event::Close => {
                    println!("close");
                    return Ok(false);
                }
                Event::Key(action, glfw::Key::Left) => {
                    self.state.input.left = action != glfw::Action::Release;
                }
                Event::Key(action, glfw::Key::Right) => {
                    self.state.input.right = action != glfw::Action::Release;
                }
                Event::Key(action, glfw::Key::Up) => {
                    self.state.input.forward = action != glfw::Action::Release;
                }
                Event::Key(action, key) => {
                    println!("key {:?}: {:?}", action, key);
                }
            }
        }

        let delta_time = self.last_update.elapsed();
        self.last_update += delta_time;
        let delta_time = delta_time.as_secs_f32();

        self.update_cat(delta_time);

        self.renderer.render(&self.state)?;
        Ok(true)
    }


    fn update_cat(&mut self, delta_time: f32) {
        let input = &self.state.input;
        let cat = &mut self.state.cat;

        let mut moving = input.forward;

        let turn_x = (delta_time * 3.0).cos();
        let turn_y = (delta_time * 3.0).sin();
        match (input.left, input.right) {
            (true, false) => {
                cat.direction.0 = cat.direction.0 * turn_x - cat.direction.1 * turn_y;
                cat.direction.1 = cat.direction.0 * turn_y + cat.direction.1 * turn_x;
                moving = true;
            }
            (false, true) => {
                cat.direction.0 = cat.direction.0 * turn_x + cat.direction.1 * turn_y;
                cat.direction.1 = -cat.direction.0 * turn_y + cat.direction.1 * turn_x;
                moving = true;
            }
            _ => {}
        }

        let direction_len = cat.direction.0.hypot(cat.direction.1);
        cat.direction.0 /= direction_len;
        cat.direction.1 /= direction_len;

        if moving {
            cat.position.0 += cat.direction.0 * 4.0 * delta_time;
            cat.position.1 += cat.direction.1 * 4.0 * delta_time;
        }

        let last = *cat.path.back().unwrap();
        let diff = (cat.position.0 - last.0, cat.position.1 - last.1);
        let diff_len = diff.0.hypot(diff.1);
        if diff_len > 0.1 {
            cat.path.pop_front();
            cat.path.push_back((
                last.0 + diff.0 * 0.1 / diff_len,
                last.1 + diff.1 * 0.1 / diff_len
            ));
            cat.tail.pop_back();
            cat.tail.push_front(*cat.path.front().unwrap());
        }
    }
}
