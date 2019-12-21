use std::{
    error::Error,
    sync::mpsc,
    time,
    collections::VecDeque
};

use lib::math::{Vec2, vec2};

mod renderer;
use renderer::Renderer;

mod state {
    use std::collections::VecDeque;

    use lib::math::Vec2;


    bitflags! {
        pub struct DirtyFlags: u8 {
            const NONE = 0;
            const RENDER = 0b1;
            const PHYSICS = 0b10;
            const ALL = Self::RENDER.bits | Self::PHYSICS.bits;
        }
    }
    
    pub struct Input {
        pub left: bool,
        pub right: bool,
        pub forward: bool
    }

    pub struct Ground {
        pub boxes: Vec<Vec2>,
        pub dirty: DirtyFlags
    }

    pub struct Cat {
        pub position: Vec2,
        pub direction: Vec2,
        pub path: VecDeque<Vec2>,
        pub tail: VecDeque<Vec2>
    }

    pub struct State {
        pub input: Input,
        pub ground: Ground,
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
        let boxes: Vec<_> = include_str!("level.txt").chars()
            .scan(vec2(-20.0, 10.0), |p, c| {
                match c {
                    ' ' => {
                        p.x += 1.0;
                        Some(None)
                    }
                    'X' => {
                        let box_p = *p;
                        p.x += 1.0;
                        Some(Some(box_p))
                    }
                    '\n' => {
                        p.x = -20.0;
                        p.y -= 1.0;
                        Some(None)
                    }
                    _ => Some(None)
                }
            }).filter_map(|p| p).collect();

        let path: VecDeque<_> = (0..180).map(|x| vec2(
            x as f32 * 0.1 - 18.0,
            0.0
        )).collect();
        let tail: VecDeque<_> = (0..20).map(|x| vec2(
            x as f32 * -0.1 - 18.0,
            0.0
        )).collect();
        
        let state = State {
            input: state::Input {
                left: false,
                right: false,
                forward: false
            },
            ground: state::Ground {
                boxes,
                dirty: state::DirtyFlags::ALL
            },
            cat: state::Cat {
                position: *path.back().unwrap(),
                direction: vec2(1.0, 0.0),
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

        self.renderer.render(&mut self.state)?;
        Ok(true)
    }


    fn update_cat(&mut self, delta_time: f32) {
        let input = &self.state.input;
        let cat = &mut self.state.cat;

        let mut moving = input.forward;

        match (input.left, input.right) {
            (true, false) => {
                let turn = Vec2::from_angle(delta_time * 3.0);
                cat.direction = cat.direction.rotated(turn).normalized();
                moving = true;
            }
            (false, true) => {
                let turn = Vec2::from_angle(-delta_time * 3.0);
                cat.direction = cat.direction.rotated(turn).normalized();
                moving = true;
            }
            _ => {}
        }

        if moving {
            cat.position += cat.direction * 4.0 * delta_time;
        }

        let last = *cat.path.back().unwrap();
        let diff = cat.position - last;
        if diff.length() > 0.1 {
            cat.path.pop_front();
            cat.path.push_back(last + diff.normalized() * 0.1);
            cat.tail.pop_back();
            cat.tail.push_front(*cat.path.front().unwrap());
        }
    }
}
