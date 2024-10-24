use core::borrow::Borrow;
use core::{u32, usize};

use embassy_stm32::cordic::Precision::Iters16;
use embassy_stm32::cordic::Scale::Arg1Res1;
use embassy_stm32::cordic::{Config, Cordic, Function::Cos};
use embassy_stm32::peripherals::{CORDIC, DMA1_CH1, DMA1_CH2};
use embassy_stm32::{Peripheral, PeripheralRef};

// * turns f32 radians into i16 Q1.15
// / turns i16 Q1.15 into f32
const RAD2Q: f32 = 10430.37835; //2^16 / 2pi

type AngleRadians = f32;
type AngleQ115 = u16; //This is an integer but in rust is easier to represent as u16 signed bit

struct TrigBrain {
    dma_read: PeripheralRef<'static,DMA1_CH1>,
    dma_write: PeripheralRef<'static, DMA1_CH2>,
    cordic: Cordic<'static,CORDIC>
}


impl TrigBrain {
    pub fn new(dma_read: DMA1_CH1, dma_write: DMA1_CH2, cordic: CORDIC) -> TrigBrain {
        let dma_read = dma_read.into_ref();
        let dma_write = dma_write.into_ref();
      
        //iterate 16 times (4x4 range 4x(3-6)), ignore scale factor, res1 cos res2 sin
        let cordic_config = Config::new(Cos, Iters16, Arg1Res1).expect("oops");
        let cordic: Cordic<CORDIC> = Cordic::new(cordic, cordic_config);
        
        TrigBrain { dma_read, dma_write, cordic }
    }
    
    pub async fn cos_sin<const N: usize>(&mut self, angle: [AngleQ115; N]) -> [AngleQ115; N] {
        
        
        
        
 
        let angle_fixed: [u32; N] = angle.map(|q| {
            
            
            
            
            let mask: u32 = 0x7FFF0000;
            let val: u32 = q as u32; //if this is negative we add u32::max and end up with a 1 on bit 31 (0 indexed) 
            let val = mask + val; //which means the signed register is negative
        
            val
        });        
                
        // let res = self.cordic.async_calc_16bit(
        //     self.dma_write.reborrow(), 
        //     self.dma_read.reborrow(), 
            
        //     &mut [2]
        // ).await.expect("bad cordic dma");
        
        
        unimplemented!("oops")
    }
    
}