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

enum FloatPlatform {
    TRUNK1,
    TRUNK2
}

enum VehicleType {
    TRUCK1,
    CAR2,
    CAR3,
    CAR4,
    CAR5
}

struct Frogger {
    pos: (f32, f32),
    timebar: f32,
    vehicles: Vec<Vehicle>
}

struct Vehicle {
    pos: (f32, f32),
    movement: Direction,
    class: VehicleType
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
            frogger.timebar -= 0.25;
        }

        if draw_timer.exhaust().is_some() {
            frogger.draw_background(&mut gfx);
            frogger.draw_scoreboard(&mut gfx);
            frogger.draw_river(&mut gfx);
            frogger.draw_street(&mut gfx);
            frogger.draw_frog(&mut gfx);
            frogger.draw_vehicles(&mut gfx);

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
    pub fn new(pos: (f32, f32), movement: Direction, class: VehicleType) -> Self {
        Self {
            pos: pos,
            movement: movement,
            class: class
        }
    }

    fn draw(&self, gfx: &mut Graphics) {
        match self.class {
            VehicleType::TRUCK1 => Self::draw_truck1(gfx),
            VehicleType::CAR2 => Self::draw_car2(gfx),
            VehicleType::CAR3 => Self::draw_car3(gfx),
            VehicleType::CAR4 => Self::draw_car4(gfx),
            VehicleType::CAR5 => Self::draw_car5(gfx),
            _ => ()
        }
    }

    fn draw_truck1(gfx: &mut Graphics) {
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

    fn draw_car2(gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 297.0), Vector::new(36.0, 16.0)),
            Color::from_hex("cc69bd")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(609.0, 297.0), Vector::new(14.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(632.0, 297.0), Vector::new(12.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 299.0), Vector::new(36.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 308.0), Vector::new(36.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(609.0, 303.0), Vector::new(14.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(609.0, 310.0), Vector::new(14.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(632.0, 310.0), Vector::new(14.0, 3.0)),
            Color::BLACK
        );
    }

    fn draw_car3(gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 326.0), Vector::new(36.0, 16.0)),
            Color::from_hex("4b6d23")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 326.0), Vector::new(5.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(612.0, 326.0), Vector::new(10.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(609.0, 330.0), Vector::new(13.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(613.0, 333.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(609.0, 335.0), Vector::new(13.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 340.0), Vector::new(5.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(612.0, 340.0), Vector::new(10.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(632.0, 326.0), Vector::new(6.0, 4.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(632.0, 332.0), Vector::new(6.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(632.0, 338.0), Vector::new(6.0, 4.0)),
            Color::BLACK
        );
    }

    fn draw_car4(gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 356.0), Vector::new(36.0, 16.0)),
            Color::from_hex("915ac3")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 358.0), Vector::new(4.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 367.0), Vector::new(4.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(626.0, 356.0), Vector::new(5.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(622.0, 357.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(622.0, 362.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(622.0, 362.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(622.0, 367.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(627.0, 369.0), Vector::new(4.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(606.0, 362.0), Vector::new(9.0, 3.0)),
            Color::BLACK
        );
    }

    fn draw_car5(gfx: &mut Graphics) {
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 386.0), Vector::new(36.0, 16.0)),
            Color::from_hex("a57924")
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 386.0), Vector::new(6.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(614.0, 386.0), Vector::new(12.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 388.0), Vector::new(36.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(614.0, 392.0), Vector::new(14.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 398.0), Vector::new(36.0, 2.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(600.0, 400.0), Vector::new(6.0, 3.0)),
            Color::BLACK
        );
        gfx.fill_rect(
            &Rectangle::new(Vector::new(614.0, 400.0), Vector::new(12.0, 3.0)),
            Color::BLACK
        );
    }
}

impl Frogger {
    pub fn new() -> Self {
        Self {
            pos: (384 as f32, 410 as f32),
            timebar: 100.0,
            vehicles: vec![
                Vehicle { pos: (10.0, 10.0), movement: Direction::LEFT, class: VehicleType::TRUCK1 },
                Vehicle { pos: (10.0, 10.0), movement: Direction::RIGHT, class: VehicleType::CAR2 },
                Vehicle { pos: (10.0, 10.0), movement: Direction::LEFT, class: VehicleType::CAR3 },
                Vehicle { pos: (10.0, 10.0), movement: Direction::RIGHT, class: VehicleType::CAR4 },
                Vehicle { pos: (10.0, 10.0), movement: Direction::LEFT, class: VehicleType::CAR5 },
            ],
        }
    }

    fn draw_vehicles(&self, gfx: &mut Graphics) {
        for vehicle in self.vehicles.iter() {
            vehicle.draw(gfx);
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
            Direction::RIGHT => self.pos.0 += 30.0,
            _ => ()
        }
    }

    fn game_over(&self, gfx: &mut Graphics) {
        
    }
}
