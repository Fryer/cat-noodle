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
        let tiles = include_str!("level.txt").chars()
            .scan(vec2(0.0, 0.0), |p, c| {
                match c {
                    ' ' => {
                        p.x += 1.0;
                        Some(None)
                    }
                    '\n' => {
                        p.x = 0.0;
                        p.y -= 1.0;
                        Some(None)
                    }
                    '\r' => Some(None),
                    _ => {
                        let tile_p = *p;
                        p.x += 1.0;
                        Some(Some((tile_p, c)))
                    }
                }
            }).filter_map(|tile| tile);

        let boxes: Vec<_> = tiles.clone().filter_map(|tile| {
            if tile.1 == 'X' { Some(tile.0) }
            else { None }
        }).collect();

        let p = tiles.clone().find_map(|tile| {
            if tile.1 == 'P' { Some(tile.0) }
            else { None }
        }).unwrap();
        let path: VecDeque<_> = (0..80).map(|x|
            vec2(
                x as f32 * 0.1 + 2.0,
                0.0
            ) + p
        ).collect();
        let tail: VecDeque<_> = (0..20).map(|x|
            vec2(
                x as f32 * -0.1 + 1.6,
                0.0
            ) + p
        ).collect();
        
        let state = State {
            input: state::Input {
                left: false,
                right: false,
                up: false,
                down: false,
                fly: false
            },
            debug: state::DebugInfo {
                shapes: VecDeque::new()
            },
            ground: state::Ground {
                boxes,
                dirty: state::DirtyFlags::ALL
            },
            cat: state::Cat {
                direction: None,
                flying: false,
                path,
                tail,
                grab_d: None
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
        self.last_update += delta_time;
        if delta_time > max_step {
            delta_time = max_step;
        }
        while delta_time >= step_time {
            if !self.step(step_time.as_secs_f32()) {
                return Ok(false);
            }
            delta_time -= step_time;
        }

        self.physics.debug(&mut self.state.debug);
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
                self.state.input.up = action != glfw::Action::Release;
            }
            Event::Key(action, glfw::Key::Down) => {
                self.state.input.down = action != glfw::Action::Release;
            }
            Event::Key(action, glfw::Key::LeftControl) => {
                self.state.input.fly = action != glfw::Action::Release;
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

        if input.left ^ input.right || input.up ^ input.down {
            let d = vec2(
                input.right as i8 as f32 - input.left as i8 as f32,
                input.up as i8 as f32 - input.down as i8 as f32
            );
            cat.direction = Some(d.to_angle());
        }
        else {
            cat.direction = None;
        }

        cat.flying = input.fly;
    }
}
