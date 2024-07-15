#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
}

// Constructor to initialize the State's GameMode.
impl State {
    fn new() -> Self {
        State {
            // Game starts at the Menu.
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
        }
    }

    // &mut self is a convention specific to instance methods
    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Fill in this stub later.
        // NAVY comes from bracket_lib.
        ctx.cls_bg(NAVY);

        // The context provides a variable named frame_time_ms containing 
        // the time elapsed since the last time tick() was called. 
        // Add this to your state’s frame_time. If it exceeds the FRAME_DURATION 
        // constant, then it’s time to run the physics simulation and reset 
        // your frame_time to zero.
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        // Place player back at new position and 
        // set frame_time back to 0.0
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        // One liner match. Runs if user presses key and extracts
        // the key's value.
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You're dead!");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

// Capture players positions.
struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

// Constructor to initialize the Player's state.
impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        // set is a bracket_lib function.
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        // 0 is top of screen, so -2.0 moves you UP on screen.
        self.velocity += -2.0;
    }
}

enum GameMode {
    Menu,
    Playing,
    End,
}

// Magic numbers for game dev, taken from page 61
// of hands on rust.
const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 = 50;
const FRAME_DURATION : f32 = 75.0;

// bracket-lit defines a Trait named GameState
// for games state structures. Requires object implement tick().
// Implement a trait similar to implementing a method
// on a struct. You implement the trait for the struct
// and define functions which the Trait expects, here tick().
impl GameState for State {
    // Takes a mutable instance of self and a mutable context of type BTerm.
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    }
}

// BError type comes from bracket_lib and if the main_loop fails
// it can pass that up and still have the main loop run correctly.
fn main() -> BError {
    println!("Hello, world!");

    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}
