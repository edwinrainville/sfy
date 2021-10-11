#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

// pick a panicking behavior
#[cfg(not(test))]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use ambiq_hal as hal;
use hal::prelude::*;
use cortex_m_rt::entry;
// use ufmt::uwriteln;
use core::ptr;
#[allow(unused_imports)]
use defmt::{debug, error, info, trace, warn};
use defmt::{global_logger, Write};

mod note;

pub struct UartLogger {
    uart: hal::uart::Uart0,
}

#[global_logger]
pub struct UartGlobalLogger;

unsafe impl defmt::Logger for UartGlobalLogger {
    fn acquire() -> Option<ptr::NonNull<dyn Write>> {
        unsafe {
            LOGGER
                .as_mut()
                .map(|l| ptr::NonNull::new_unchecked(l as &mut dyn defmt::Write))
        }
    }

    unsafe fn release(_writer: ptr::NonNull<dyn Write>) {}
}

impl defmt::Write for UartLogger {
    fn write(&mut self, bytes: &[u8]) {
        for b in bytes {
            nb::block!(self.uart.write(*b)).ok();
        }
    }
}

static mut LOGGER: Option<UartLogger> = None;

#[entry]
fn main() -> ! {
    // Set the clock frequency.
    unsafe {
        halc::am_hal_clkgen_control(
            halc::am_hal_clkgen_control_e_AM_HAL_CLKGEN_CONTROL_SYSCLK_MAX,
            0 as *mut c_void,
        );

        // Set the default cache configuration
        halc::am_hal_cachectrl_config(&halc::am_hal_cachectrl_defaults);
        halc::am_hal_cachectrl_enable();

        // Configure the board for low power operation.
        halc::am_bsp_low_power_init();
    }

    let mut dp = hal::pac::Peripherals::take().unwrap();
    let core = hal::pac::CorePeripherals::take().unwrap();

    let pins = hal::gpio::Pins::new(dp.GPIO);

    let mut delay = hal::delay::Delay::new(core.SYST, &mut dp.CLKGEN);
    let serial = hal::uart::Uart0::new(dp.UART0, pins.tx0, pins.rx0);
    unsafe {
        LOGGER = Some(UartLogger { uart: serial });
    }

    let i2c = hal::i2c::I2c::new(dp.IOM4, pins.d15, pins.d14);
    let mut note = note::Note::new(i2c);

    // Set up BSP leds
    let mut led = pins.d13.into_push_pull_output();
    let mut i = 0;

    // Blink forever
    loop {
        defmt::info!("hello world {}!", i);
        // info!("hello world!");
        // uwriteln!(&mut serial, "hello world: {}\r", i).unwrap();
        i += 1;

        // Toggle LEDs
        led.toggle().unwrap();

        // Delay
        delay.delay_ms(300u32);

        if note.ping() {
            warn!("noteboard found!");
        } else {
            error!("noteboard not found!");
        }

        // Write something to the noteboard


        // i2c.write(noteaddr, r#"{"req": "card.time"}\n"#.as_bytes());

        delay.delay_ms(300u32);

        // let mut buffer = [0u8; 10];
        // i2c.read(noteaddr, &mut buffer);
        // ufmt::uwriteln!(&mut serial, "note: {}", unsafe { core::str::from_utf8_unchecked(&buffer) });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2, 1 + 1)
    }
}
