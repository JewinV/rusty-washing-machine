#![no_std]
#![no_main]
#![feature(associated_type_bounds)]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output(); // Arduino LED
    let direction = pins.d12.into_output(); // if high, clockwise else counter-clockwise
    let power = pins.d11.into_output(); // if high, then power is supplied to the main and drain motors
    let drain = pins.d10.into_output(); // if high, then drain motor is on
    let inlet = pins.d9.into_output(); // if high, then water inlet is on

    let water_level_sensor = pins.d8.into_pull_up_input(); // if ground, then water level is correct

    let mut washing_machine = WashingMachine::new(power, direction, drain, inlet, water_level_sensor);

    washing_machine.idle();
    washing_machine.filling();
    washing_machine.wash();
    washing_machine.drain();
    washing_machine.filling();
    washing_machine.wash();
    washing_machine.drain();
    washing_machine.spin();
    washing_machine.idle();
    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}

struct WashingMachine<O1, O2, O3, O4, I1> {
    power: O1,
    direction: O2,
    drain: O3,
    inlet: O4,
    water_level_sensor: I1,

}

impl<O1, O2, O3, O4, I1> WashingMachine<O1, O2, O3, O4, I1> 
    where
        O1: embedded_hal::digital::v2::OutputPin<Error: core::fmt::Debug>,
        O2: embedded_hal::digital::v2::OutputPin<Error: core::fmt::Debug>,
        O3: embedded_hal::digital::v2::OutputPin<Error: core::fmt::Debug>,
        O4: embedded_hal::digital::v2::OutputPin<Error: core::fmt::Debug>,
        I1: embedded_hal::digital::v2::InputPin<Error: core::fmt::Debug>,
    {
    fn new(power: O1, direction: O2, drain: O3, inlet: O4, water_level_sensor: I1) -> Self {
        WashingMachine {
            power,
            direction,
            drain,
            inlet,
            water_level_sensor,
        }
    }

    fn wash(&mut self) {
        for _ in 0..40 {
            self.wash_cycle_1();
        }
        for _ in 0..40 {
            self.wash_cycle_2();
        }
        for _ in 0..40 {
            self.wash_cycle_3();
        }
    }

    fn drain(&mut self) {
        self.drain.set_high().unwrap();
        arduino_hal::delay_ms(60000);
        self.drain.set_low().unwrap();
        arduino_hal::delay_ms(1000);
    }

    fn spin(&mut self) {
        self.direction.set_high().unwrap();
        self.drain.set_high().unwrap();
        arduino_hal::delay_ms(3000);
        self.power.set_high().unwrap();
        arduino_hal::delay_ms(1000);
        self.power.set_low().unwrap();
        arduino_hal::delay_ms(2000);
        self.power.set_high().unwrap();
        arduino_hal::delay_ms(2000);
        self.power.set_low().unwrap();
        arduino_hal::delay_ms(2000);
        self.power.set_high().unwrap();

        arduino_hal::delay_ms(60000);
        arduino_hal::delay_ms(60000);
        arduino_hal::delay_ms(60000);
        self.power.set_low().unwrap();
        arduino_hal::delay_ms(15000);
        self.drain.set_low().unwrap();
    }

    fn filling(&mut self) {
        while self.water_level_sensor.is_high().unwrap() {
            self.inlet.set_high().unwrap();
            }
        self.inlet.set_low().unwrap();
        arduino_hal::delay_ms(1000);
        }

    fn wash_cycle_1(&mut self) {
        self.direction.set_high().unwrap();
        self.power.set_high().unwrap();
        arduino_hal::delay_ms(2500);
        self.power.set_low().unwrap();
        arduino_hal::delay_ms(1500);
        self.direction.set_low().unwrap();
        self.power.set_high().unwrap();
        arduino_hal::delay_ms(2500);
        self.power.set_low().unwrap();
        arduino_hal::delay_ms(1500);
    }


    fn wash_cycle_2(&mut self) {
        self.direction.set_high().unwrap();
        self.power.set_high().unwrap();
        arduino_hal::delay_ms(2500);
        self.power.set_low().unwrap();
        arduino_hal::delay_ms(500);
        self.direction.set_low().unwrap();
        self.power.set_high().unwrap();
        arduino_hal::delay_ms(2500);
        self.power.set_low().unwrap();
        arduino_hal::delay_ms(500);
    }


    fn wash_cycle_3(&mut self) {
        self.direction.set_high().unwrap();
        self.power.set_high().unwrap();
        arduino_hal::delay_ms(4500);
        self.power.set_low().unwrap();
        arduino_hal::delay_ms(1500);
        self.direction.set_low().unwrap();
        self.power.set_high().unwrap();
        arduino_hal::delay_ms(4500);
        self.power.set_low().unwrap();
        arduino_hal::delay_ms(1500);
    }

    fn idle(&mut self) {
        self.power.set_low().unwrap();
        self.direction.set_low().unwrap();
        self.drain.set_low().unwrap();
        self.inlet.set_low().unwrap();
    }

}
