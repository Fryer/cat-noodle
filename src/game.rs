use std::{
    error::Error,
    sync::mpsc,
    time,
    collections::VecDeque
};

use lib::math::vec2;

mod state;
use state::State;

mod renderer;
use renderer::Renderer;

mod physics;


pub enum Event {
    Close,
    Key(glfw::Action, glfw::Key)
}

pub struct Game {
    last_update: time::Instant,
    event_receiver: mpsc::Receiver<Event>,
    state: State,
    renderer: Renderer,
    physics: physics::World
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
                movement: state::CatMovement::None,
                path,
                tail
            }
        };

        let physics = physics::World::new(&state);
        
        Ok(Game {
            last_update: time::Instant::now(),
            event_receiver,
            state,
            renderer: Renderer::new()?,
            physics
        })
    }


    pub fn update(&mut self) -> Result<bool, Box<dyn Error>> {
        let step_time = time::Duration::from_secs(1) / 480;
        let max_step = step_time * 48;
        let mut delta_time = self.last_update.elapsed();
        if delta_time > max_step {
            delta_time = max_step;
        }
        while delta_time >= step_time {
            if !self.step(step_time.as_secs_f32()) {
                return Ok(false);
            }
            delta_time -= step_time;
            self.last_update += step_time;
        }

        self.renderer.render(&mut self.state)?;
        Ok(true)
    }


    fn step(&mut self, delta_time: f32) -> bool {
        while let Ok(event) = self.event_receiver.try_recv() {
            if !self.handle_event(event) {
                return false;
            }
        }

        self.update_cat(delta_time);

        self.physics.step(&mut self.state, delta_time);
        true
    }


    fn handle_event(&mut self, event: Event) -> bool {
        match event {
            Event::Close => {
                println!("close");
                return false;
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
        true
    }


    fn update_cat(&mut self, _delta_time: f32) {
        let input = &self.state.input;
        let cat = &mut self.state.cat;

        cat.movement = match (input.left, input.right) {
            (true, false) => state::CatMovement::Left,
            (false, true) => state::CatMovement::Right,
            _ =>
                if input.forward { state::CatMovement::Forward }
                else { state::CatMovement::None }
        }
    }
}
