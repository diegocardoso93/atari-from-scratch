use quicksilver::{
    geom::{Rectangle, Triangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Timer, Window
};

pub fn start() {
    run(
        Settings {
            size: Vector::new(768 as f32, 540 as f32),
            title: "frogger",
            resizable: true,
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    gfx.clear(Color::from_hex("507326"));

    let mut update_timer = Timer::time_per_second(8.0);
    let mut draw_timer = Timer::time_per_second(60.0);

    let mut frogger = Frogger::new();
    loop {
        while let Some(_) = input.next_event().await {}
        
        while update_timer.tick() {
            if input.key_down(Key::W) {
                frogger.move_frog(0);
            }
            if input.key_down(Key::D) {
                frogger.move_frog(1);
            }
            if input.key_down(Key::S) {
                frogger.move_frog(2);
            }
            if input.key_down(Key::A) {
                frogger.move_frog(3);
            }
        }

        if draw_timer.exhaust().is_some() {
            frogger.draw_background(&mut gfx);
            frogger.draw_scoreboard(&mut gfx);
            frogger.draw_river(&mut gfx);
            frogger.draw_street(&mut gfx);
            frogger.draw_frog(&mut gfx);

            frogger.draw_truck1(&mut gfx);
            frogger.draw_car2(&mut gfx);
            frogger.draw_car3(&mut gfx);
            frogger.draw_car4(&mut gfx);
            frogger.draw_car5(&mut gfx);

            gfx.present(&window)?;
        }
    }
}

struct Frogger {
    player_pos: (f32, f32)
}

impl Frogger {
    pub fn new() -> Self {
        Self {
            player_pos: (384 as f32, 410 as f32)
        }
    }

    fn draw_background(&self, gfx: &mut Graphics) {
        gfx.clear(Color::from_hex("507326"));
    }

    fn draw_scoreboard(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(0.0, 0.0), Vector::new(768.0, 40.0)),
            Color::BLACK
        );
    }

    fn draw_river(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(30.0, 50.0), Vector::new(700.0, 180.0)),
            Color::from_hex("0c3991")
        );
    }

    fn draw_street(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(30.0, 250.0), Vector::new(700.0, 160.0)),
            Color::BLACK
        );
        self.draw_sidewalk(gfx);
    }

    fn draw_sidewalk(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(30.0, 230.0), Vector::new(700.0, 30.0)),
            Color::from_hex("f3e347")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(30.0, 410.0), Vector::new(700.0, 30.0)),
            Color::from_hex("b9ac32")
        );
    }

    fn draw_timebar(&self, gfx: &mut Graphics) {
        
    }

    fn draw_lifes(&self, gfx: &mut Graphics) {

    }

    fn draw_life_point(&self, gfx: &mut Graphics) {

    }

    fn draw_frog(&self, gfx: &mut Graphics) {
        let x = self.player_pos.0;
        let y = self.player_pos.1;
        gfx.fill_rect(
            &Rectangle::new(Vector::new(x - 15.0, y + 4.0), Vector::new(6.0, 6.0)),
            Color::from_hex("4d712d")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(x + 15.0, y + 4.0), Vector::new(6.0, 6.0)),
            Color::from_hex("4d712d")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(x - 10.0, y + 10.0), Vector::new(26.0, 3.0)),
            Color::from_hex("4d712d")
        );

        gfx.fill_rect(
            &Rectangle::new(Vector::new(x, y + 4.0), Vector::new(6.0, 5.0)),
            Color::from_hex("4d712d")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(x - 5.0, y + 7.0), Vector::new(16.0, 3.0)),
            Color::from_hex("4d712d")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(x - 5.0, y + 10.0), Vector::new(16.0, 5.0)),
            Color::from_hex("4d712d")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(x - 15.0, y + 15.0), Vector::new(36.0, 3.0)),
            Color::from_hex("4d712d")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(x - 15.0, y + 15.0), Vector::new(6.0, 10.0)),
            Color::from_hex("4d712d")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(x + 15.0, y + 15.0), Vector::new(6.0, 10.0)),
            Color::from_hex("4d712d")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(x, y + 15.0), Vector::new(6.0, 6.0)),
            Color::from_hex("4d712d")
        );
    }

    fn draw_truck1(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 269.0), Vector::new(28.0, 12.0)),
            Color::WHITE
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(608.0, 267.0), Vector::new(10.0, 16.0)),
            Color::WHITE
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(628.0, 267.0), Vector::new(45.0, 16.0)),
            Color::WHITE
        );
    }

    fn draw_car2(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 296.0), Vector::new(30.0, 16.0)),
            Color::from_hex("cc69bd")
        );
    }

    fn draw_car3(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 326.0), Vector::new(30.0, 16.0)),
            Color::from_hex("4b6d23")
        );
    }

    fn draw_car4(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 356.0), Vector::new(30.0, 16.0)),
            Color::from_hex("915ac3")
        );
    }

    fn draw_car5(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 386.0), Vector::new(30.0, 16.0)),
            Color::from_hex("a57924")
        );
    }

    fn move_frog(&mut self, direction: u8) {
        if (direction == 0) {
            self.player_pos.1 -= 30.0;
        }
        if (direction == 1) {
            self.player_pos.0 += 30.0;
        }
        if (direction == 2) {
            self.player_pos.1 += 30.0;
        }
        if (direction == 3) {
            self.player_pos.0 -= 30.0;
        }
    }
}
