use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    input::Key,
    run, Graphics, Input, Result, Settings, Timer, Window
};

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct Frogger {
    pos: (f32, f32),
    timebar: f32,
    vehicles: Vec<Vehicle>,
}

struct Vehicle {
    x: f32,
    y: f32,
    vfactor: f32
}

struct Truck1 {
    v: Vehicle
}

struct Car2 {
    v: Vehicle
}

struct Car3 {
    v: Vehicle
}

struct Car4 {
    v: Vehicle
}

struct Car5 {
    v: Vehicle
}

trait Draw {
    fn draw(&self, gfx: &mut Graphics);
}

impl Truck1 {
    pub fn new() -> Self {
        Self {
            v: Vehicle {
                x: 730.0,
                y: 267.0,
                vfactor: -1.34
            }
        }
    }
}

impl Car2 {
    pub fn new() -> Self {
        Self {
            v: Vehicle {
                x: -5.0,
                y: 297.0,
                vfactor: 1.64
            }
        }
    }
}

impl Car3 {
    pub fn new() -> Self {
        Self {
            v: Vehicle {
                x: 730.0,
                y: 326.0,
                vfactor: -1.64
            }
        }
    }
}

impl Car4 {
    pub fn new() -> Self {
        Self {
            v: Vehicle {
                x: -5.0,
                y: 356.0,
                vfactor: 1.64
            }
        }
    }
}

impl Car5 {
    pub fn new() -> Self {
        Self {
            v: Vehicle {
                x: 730.0,
                y: 386.0,
                vfactor: -1.84
            }
        }
    }
}

impl Draw for Truck1 {
    fn draw(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y + 2.0), Vector::new(28.0, 12.0)),
            Color::WHITE
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 8.0, self.v.y), Vector::new(10.0, 16.0)),
            Color::WHITE
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 28.0, self.v.y), Vector::new(45.0, 16.0)),
            Color::WHITE
        );
    }
}

impl Draw for Car2 {
    fn draw(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y), Vector::new(36.0, 16.0)),
            Color::from_hex("cc69bd")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 9.0, self.v.y), Vector::new(14.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 32.0, self.v.y), Vector::new(12.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y + 2.0), Vector::new(36.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y + 11.0), Vector::new(36.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 9.0, self.v.y + 5.0), Vector::new(14.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 9.0, self.v.y + 13.0), Vector::new(14.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 32.0, self.v.y + 13.0), Vector::new(14.0, 3.0)),
            Color::BLACK
        );
    }
}

impl Draw for Car3 {
    fn draw(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y), Vector::new(36.0, 16.0)),
            Color::from_hex("4b6d23")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y), Vector::new(5.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 12.0, self.v.y), Vector::new(10.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 9.0, self.v.y + 4.0), Vector::new(13.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 13.0, self.v.y + 7.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 9.0, self.v.y + 9.0), Vector::new(13.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y + 14.0), Vector::new(5.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 12.0, self.v.y + 14.0), Vector::new(10.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 32.0, self.v.y), Vector::new(6.0, 4.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 32.0, self.v.y + 6.0), Vector::new(6.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 32.0, self.v.y + 12.0), Vector::new(6.0, 4.0)),
            Color::BLACK
        );
    }
}

impl Draw for Car4 {
    fn draw(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y), Vector::new(36.0, 16.0)),
            Color::from_hex("915ac3")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y + 2.0), Vector::new(4.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y + 11.0), Vector::new(4.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 26.0, self.v.y), Vector::new(5.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 22.0, self.v.y + 1.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 22.0, self.v.y + 6.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 22.0, self.v.y + 6.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 22.0, self.v.y + 11.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 27.0, self.v.y + 13.0), Vector::new(4.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 6.0, self.v.y + 6.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
    }
}

impl Draw for Car5 {
    fn draw(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y), Vector::new(36.0, 16.0)),
            Color::from_hex("a57924")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y), Vector::new(6.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 14.0, self.v.y), Vector::new(12.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y + 2.0), Vector::new(36.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 14.0, self.v.y + 6.0), Vector::new(14.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y + 12.0), Vector::new(36.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x, self.v.y + 14.0), Vector::new(6.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(self.v.x + 14.0, self.v.y + 14.0), Vector::new(12.0, 3.0)),
            Color::BLACK
        );
    }
}

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
    let mut truck1 = Truck1::new();
    let mut car2 = Car2::new();
    let mut car3 = Car3::new();
    let mut car4 = Car4::new();
    let mut car5 = Car5::new();
    loop {
        while let Some(_) = input.next_event().await {}

        while update_timer.tick() {
            if input.key_down(Key::W) {
                frogger.move_frog(Direction::UP);
            }
            if input.key_down(Key::D) {
                frogger.move_frog(Direction::RIGHT);
            }
            if input.key_down(Key::S) {
                frogger.move_frog(Direction::DOWN);
            }
            if input.key_down(Key::A) {
                frogger.move_frog(Direction::LEFT);
            }
            frogger.timebar -= 0.5;
        }

        if draw_timer.exhaust().is_some() {
            frogger.draw_background(&mut gfx);
            frogger.draw_scoreboard(&mut gfx);
            frogger.draw_river(&mut gfx);
            frogger.draw_street(&mut gfx);
            frogger.draw_frog(&mut gfx);
            truck1.draw(&mut gfx);
            car2.draw(&mut gfx);
            car3.draw(&mut gfx);
            car4.draw(&mut gfx);
            car5.draw(&mut gfx);
            truck1.v.run();
            car2.v.run();
            car3.v.run();
            car4.v.run();
            car5.v.run();
            frogger.left_ground(&mut gfx);
            frogger.right_ground(&mut gfx);

            frogger.draw_trunk1(&mut gfx);
            frogger.draw_trunk2(&mut gfx);
            frogger.draw_float_floor(&mut gfx);
            frogger.draw_arrival(&mut gfx);

            frogger.draw_timebar(&mut gfx);

            gfx.present(&window)?;
        }
    }
}

impl Vehicle {
    pub fn new(x: f32, y: f32, vfactor: f32) -> Self {
        Self {
            x: x,
            y: y,
            vfactor: vfactor
        }
    }

    fn run(&mut self) {
        self.x += self.vfactor;
    }
}

impl Frogger {
    pub fn new() -> Self {
        Self {
            pos: (384 as f32, 410 as f32),
            timebar: 100.0,
            vehicles: vec![],
        }
    }

    fn draw_background(&self, gfx: &mut Graphics) {
        gfx.clear(Color::from_hex("507326"));
    }

    fn left_ground(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(0.0, 50.0), Vector::new(30.0, 500.0)),
            Color::from_hex("507326")
        );
    }

    fn right_ground(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(740.0, 50.0), Vector::new(30.0, 500.0)),
            Color::from_hex("507326")
        );
    }

    fn draw_scoreboard(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(0.0, 0.0), Vector::new(768.0, 40.0)),
            Color::BLACK
        );
    }

    fn draw_river(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(30.0, 50.0), Vector::new(710.0, 180.0)),
            Color::from_hex("0c3991")
        );
    }

    fn draw_street(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(30.0, 250.0), Vector::new(710.0, 160.0)),
            Color::BLACK
        );
        self.draw_sidewalk(gfx);
    }

    fn draw_sidewalk(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(30.0, 230.0), Vector::new(710.0, 30.0)),
            Color::from_hex("f3e347")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(30.0, 410.0), Vector::new(710.0, 30.0)),
            Color::from_hex("b9ac32")
        );
    }

    fn draw_timebar(&self, gfx: &mut Graphics) {
        if self.timebar > 0.0 {
            gfx.fill_rect(
                &Rectangle::new(Vector::new(730.0 - (2.0 * self.timebar), 448.0), Vector::new(2.0 * self.timebar, 16.0)),
                Color::BLACK
            );
        }
    }

    fn draw_lifes(&self, gfx: &mut Graphics) {

    }

    fn draw_life_point(&self, gfx: &mut Graphics) {

    }

    fn draw_frog(&self, gfx: &mut Graphics) {
        let x = self.pos.0;
        let y = self.pos.1;
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

    fn draw_trunk1(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(540.0, 145.0), Vector::new(158.0, 18.0)),
            Color::from_hex("6d6314")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(603.0, 148.0), Vector::new(24.0, 3.0)),
            Color::from_hex("0c3991")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(540.0, 153.0), Vector::new(24.0, 3.0)),
            Color::from_hex("0c3991")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(638.0, 153.0), Vector::new(24.0, 3.0)),
            Color::from_hex("0c3991")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(660.0, 160.0), Vector::new(24.0, 3.0)),
            Color::from_hex("0c3991")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(580.0, 159.0), Vector::new(24.0, 3.0)),
            Color::from_hex("0c3991")
        );
    }

    fn draw_trunk2(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(540.0, 174.0), Vector::new(88.0, 18.0)),
            Color::from_hex("6d6314")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(552.0, 176.0), Vector::new(6.0, 3.0)),
            Color::from_hex("0c3991")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 176.0), Vector::new(6.0, 3.0)),
            Color::from_hex("0c3991")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(564.0, 180.0), Vector::new(6.0, 3.0)),
            Color::from_hex("0c3991")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(614.0, 180.0), Vector::new(6.0, 3.0)),
            Color::from_hex("0c3991")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(540.0, 186.0), Vector::new(6.0, 3.0)),
            Color::from_hex("0c3991")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(584.0, 186.0), Vector::new(6.0, 3.0)),
            Color::from_hex("0c3991")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(604.0, 186.0), Vector::new(6.0, 3.0)),
            Color::from_hex("0c3991")
        );
    }

    fn draw_float_floor(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(580.0, 207.0), Vector::new(32.0, 12.0)),
            Color::from_hex("416fcd")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(576.0, 210.0), Vector::new(40.0, 7.0)),
            Color::from_hex("416fcd")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(584.0, 204.0), Vector::new(24.0, 18.0)),
            Color::from_hex("416fcd")
        );
    }

    fn draw_arrival(&self, gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 50.0), Vector::new(68.0, 30.0)),
            Color::from_hex("507326")
        );
    }

    fn move_frog(&mut self, direction: Direction) {
        match direction {
            Direction::UP => self.pos.1 -= 30.0,
            Direction::DOWN => self.pos.1 += 30.0,
            Direction::LEFT => self.pos.0 -= 30.0,
            Direction::RIGHT => self.pos.0 += 30.0
        }
    }

    fn game_over(&self, gfx: &mut Graphics) {
        
    }
}
