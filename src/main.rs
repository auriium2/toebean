#![no_std]
#![no_main]

use defmt::{info, panic};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals::*;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, can, Config};
use embassy_time::Timer;
use static_cell::StaticCell;
//  'use' / linker 
use {panic_probe as _, defmt_rtt as _};

mod sex {
    
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {

    
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");
    
    let mut led = Output::new(p.PA5, Level::High, Speed::Low);

    loop {
        info!("high");
        led.set_high();
        Timer::after_millis(300).await;
    
        info!("low");
        led.set_low();
        Timer::after_millis(300).await;
        
    }
    
    
    // let mut config = Config::default();
    //     {
    //         use embassy_stm32::rcc::*;
            
    //         config.rcc.hse = Some(Hse {
    //             freq: Hertz(24_000_000),
    //             mode: HseMode::Oscillator,
    //         });
            
    //         config.rcc.pll = Some(Pll {
    //             source: PllSource::HSE,
    //             prediv: PllPreDiv::DIV6,
    //             mul: PllMul::MUL85,
    //             divp: None,
    //             divq: Some(PllQDiv::DIV8), // 42.5 Mhz for fdcan.
    //             divr: Some(PllRDiv::DIV2), // Main system clock at 170 MHz
    //         });
    //         config.rcc.mux.fdcansel = mux::Fdcansel::PLL1_Q;
    //         config.rcc.sys = Sysclk::PLL1_R;
    //     }
    //     let peripherals = embassy_stm32::init(config);
    
        // let mut can = can::CanConfigurator::new(peripherals.FDCAN1, peripherals.PA11, peripherals.PA12, Irqs);
    
    //     can.properties().set_extended_filter(
    //         can::filter::ExtendedFilterSlot::_0,
    //         can::filter::ExtendedFilter::accept_all_into_fifo1(),
    //     );
    
    //     // 250k bps
    //     can.set_bitrate(250_000);
    
    //     let use_fd = false;
    
    //     // 1M bps
    //     if use_fd {
    //         can.set_fd_data_bitrate(1_000_000, false);
    //     }
    
    //     info!("Configured");
    
    //     let mut can = can.start(match use_fd {
    //         true => can::OperatingMode::InternalLoopbackMode,
    //         false => can::OperatingMode::NormalOperationMode,
    //     });
}

#[embassy_executor::task]
async fn some_task() {
    
}