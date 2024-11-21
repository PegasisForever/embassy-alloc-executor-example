#![no_std]
#![no_main]

mod fmt;

use embedded_alloc::Heap;
#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Level, Output, Pin, Speed},
    Peripheral,
};
use embassy_time::{Duration, Timer};
use fmt::info;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello, World!");

    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024 * 10; // 10 KiB
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    spawner.spawn_fut(move || blink_task(p.PA15)).unwrap();
    info!("Used heap: {}Bytes", HEAP.used());
}

async fn blink_task(pin: impl Peripheral<P = impl Pin>) -> ! {
    let mut led = Output::new(pin, Level::High, Speed::Low);
    loop {
        info!("LED on");
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        info!("LED off");
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}
