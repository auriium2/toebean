#![no_std]
#![no_main]

use core::any::Any;
use core::ops::Deref;

use embassy_executor::Spawner;

use embassy_stm32::gpio::AnyPin;
use embassy_stm32::peripherals::*;
use embassy_stm32::rcc::*;
use embassy_stm32::Peripheral;
use embassy_stm32::PeripheralRef;
use embassy_stm32::Peripherals;
use embassy_stm32::{bind_interrupts, can};
use embassy_stm32::time::Hertz;
use math::TrigBrain;

mod math;

//  'use' / linker 
use {panic_probe as _, defmt_rtt as _};

bind_interrupts!(struct Irqs {
    FDCAN1_IT0 => can::IT0InterruptHandler<FDCAN1>;
    FDCAN1_IT1 => can::IT1InterruptHandler<FDCAN1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = init_clock();
    let peripherals: embassy_stm32::Peripherals = embassy_stm32::init(config);
    
    use embassy_stm32::cordic::Precision::Iters16;
    use embassy_stm32::cordic::Scale::Arg1o16Res16;
    use embassy_stm32::cordic::{Config, Cordic, Function::Cos};
    use embassy_stm32::peripherals::{CORDIC, DMA1_CH1, DMA1_CH2};

    
    let cordic_config = Config::new(Cos, Iters16, Arg1o16Res16).expect("oops");
    let mut cordic: Cordic<CORDIC> = Cordic::new(peripherals.CORDIC, cordic_config);
    

   // let trig_brain: TrigBrain = math::TrigBrain::new(peripherals.DMA1_CH1, peripherals.DMA1_CH2, peripherals.CORDIC);
    
    
    //cordic_peripheral.async_calc_16bit(peripherals.DMA1_CH1, peripherals.DMA1_CH2, &[input], res);
        
    let mut can = can::CanConfigurator::new(peripherals.FDCAN1, peripherals.PA11, peripherals.PA12, Irqs);
    
        // can.properties().set_extended_filter(
        //     can::filter::ExtendedFilterSlot::_0,
        //     can::filter::ExtendedFilter::accept_all_into_fifo1(),
        // );
    
        // // 250k bps
        // can.set_bitrate(250_000);
    
        // let use_fd = false;
    
        // // 1M bps
        // if use_fd {
        //     can.set_fd_data_bitrate(1_000_000, false);
        // }
    
        // info!("Configured");
    
        // let mut can = can.start(match use_fd {
        //     true => can::OperatingMode::InternalLoopbackMode,
        //     false => can::OperatingMode::NormalOperationMode,
        // });
        
        // can.read_fd()
}

fn init_clock() -> embassy_stm32::Config {
    let mut config = embassy_stm32::Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000), //internal oscillator is 8 mhz
        mode: HseMode::Oscillator,
    });
    
    config.rcc.pll = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV2, //PLL_M reduces PLL_IN to 4 mhz
        mul: PllMul::MUL85, // 4mhz * 85 = 340mhz pll
    
        
        divp: Some(PllPDiv::DIV8), //42.5 MHZ adc
        divq: Some(PllQDiv::DIV8), // 42.5 Mhz for fdcan. 
        divr: Some(PllRDiv::DIV2), // Main system clock at 170 MHz
        //is pll running at 340 mhz?
    });
    
    config.rcc.boost = true; //170mhz requires boost
    
    config.rcc.sys = Sysclk::PLL1_R; // request R clock to drive the sys clock
    
    //ahb clock derived from sysclock
    config.rcc.ahb_pre = AHBPrescaler::DIV1; //ahb prescaler is *1 so ahb prescaled input is 170mhz
    config.rcc.apb1_pre = APBPrescaler::DIV1; //ahb prescaler is *1 so apb prescaler receives 170mhz * 1 = 170mhz
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    
    
    //mux
    config.rcc.mux.adc12sel = mux::Adcsel::PLL1_P;
    config.rcc.mux.fdcansel = mux::Fdcansel::PLL1_Q;
        
    config
}


#[embassy_executor::task]
async fn some_task() {
    
}