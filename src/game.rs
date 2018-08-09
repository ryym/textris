use action::Action;
use errors::*;
use inputs::{Inputs, Order};
use play::Play;
use screen::{Modal, Screen};
use std::io::Write;
use std::thread;
use std::time::Duration;

const FRAME: u64 = 50;
const TICK: u64 = 1000 / FRAME;
const UPDATE: u64 = TICK / 2;

pub struct Game<W: Write> {
    inputs: Inputs,
    screen: Screen<W>,
    help_modal: Modal<'static>,
}

impl<W: Write> Game<W> {
    pub fn new(inputs: Inputs, screen: Screen<W>) -> Self {
        Game {
            inputs,
            screen,
            help_modal: Modal {
                title: "HELP",
                content: &[
                    "h - Move left",
                    "l - Move right",
                    "j - Speed up",
                    "d,f - Rotate",
                    "q - Quit",
                ],
                actions: Some(&[Action::Ok, Action::Reset, Action::Quit]),
            },
        }
    }

    pub fn stop_by_error(&mut self, err: Error) {
        self.screen
            .show_modal(
                &mut self.inputs,
                &Modal {
                    title: "ERROR",
                    content: &[
                        "Sorry, unexpected error occurred.",
                        "details:",
                        &err.to_string(),
                    ],
                    actions: None,
                },
            )
            .expect(&format!("show error dialog ({})", err));
    }

    pub fn start(&mut self) -> Result<()> {
        self.screen.render_title()?;
        thread::sleep(Duration::from_millis(800));

        loop {
            let next_action = self.play(Play::new())?;
            if next_action == Action::Quit {
                return Ok(());
            }
        }
    }

    fn play(&mut self, mut play: Play) -> Result<Action> {
        self.screen.render_header()?;

        let interval = Duration::from_millis(FRAME);
        let mut t = 0;
        loop {
            match self.handle_user_input(&mut play)? {
                Some(action) => {
                    if action != Action::Ok {
                        return Ok(action);
                    }
                }
                _ => {}
            };

            if t % UPDATE == 0 {
                if let Err(_) = play.update() {
                    return self.screen.render_game_over(&mut self.inputs, &play);
                }
            }
            if t % TICK == 0 {
                play.tick();
            }

            self.screen.render(&play)?;
            thread::sleep(interval);
            t += 1;
        }
    }

    fn handle_user_input(&mut self, play: &mut Play) -> Result<Option<Action>> {
        match self.inputs.try_recv_order()? {
            Some(Ok(order)) => match order {
                Order::Move(dir) => play.slide_tetro(dir),
                Order::Rotate(rotation) => play.rotate_tetro(rotation),
                Order::Quit => return Ok(Some(Action::Quit)),
                Order::Help => {
                    return self
                        .screen
                        .show_modal(&mut self.inputs, &self.help_modal)
                        .map(|action| Some(action));
                }
                _ => {}
            },
            Some(Err(err)) => return Err(err.into()),
            None => {}
        };
        Ok(None)
    }
}
