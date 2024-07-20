#![warn(clippy::all, clippy::pedantic)]

use bracket_lib::prelude::*;

struct State {
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    mode: GameMode,
    score: i32,
}

// Constructor to initialize the State's GameMode.
impl State {
    fn new() -> Self {
        State {
            // Game starts at the Menu.
            player: Player::new(5, 25),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    // &mut self is a convention specific to instance methods
    fn play(&mut self, ctx: &mut BTerm) {
        // TODO: Fill in this stub later.
        // NAVY comes from bracket_lib.
        ctx.cls_bg(NAVY);
        // Output game instructin and score onto screen.
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        // The context provides frame_time_ms containing 
        // time elapsed since last tick() call. 
        // Add this to state’s frame_time. If frame_time_ms > FRAME_DURATION 
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

        // Render the obstacles on screen.
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(
                self.player.x + SCREEN_WIDTH,
                self.score,
            );
        }
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
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

// Render the walls using the '|' character.
struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    // Score is determined by how many Obstacles are avoided.
    fn new(x: i32, score: i32) -> Self {
        // Generate randomly placed gaps on the screen for
        // the obstacles.
        let mut random = RandomNumberGenerator::new();

        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score)
        }
    }

    /*
    screen_x location from the obstacle requires 
    converstion from world-spce to screen-space.
    Player is always at 0 in screen-space, but has 
    a game world position defined in player.x.
    The obstacle's x value is also in this world-space.
    Convert to screen-space by subtracting player.x's location 
    from obstacle.x's location.
                                                0
                            |              |
                            |   obstacle   |
                            ----------------
                                    +
        gap_y - (size / 2)  ------->|           Y
                                    +           
                    gap_y   ------->- point     A
                                    +           x
        gap_y + (size / 2)  ------->|           i
                                    +           s
                            ----------------
                            |   obstacle   |
                            |              |

                                                50
    */

    // Render the obstacle diagrammed above.
    // Each obstacle has a bottom half and top half with a gap.
    fn render(&mut self, ctx: &mut BTerm, player_x : i32) {
        // Convert to screen space.
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // Use 2 for loops to create the top and bottom obstacle.
        // Draw the top half of the obstacle.
        for y in 0..self.gap_y - half_size {
            ctx.set(
                screen_x,
                y,
                RED,
                BLACK,
                to_cp437('|')
            );
        }

        // Draw the bottom half of the obstacle.
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                screen_x,
                y,
                RED,
                BLACK,
                to_cp437('|'),
            );
        }
    }

    // Taking a borrowed reference to the player to determing
    // the player's position. 
    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let plaery_below_gap = player.y > self.gap_y + half_size;
        
        does_x_match && (plaery_below_gap || player_above_gap)
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
