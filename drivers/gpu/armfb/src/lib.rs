#![no_std]

#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

use fb::{Fb};

//#[cfg(target_arch = "aarch64")]
pub mod a64;

#[cfg(target_arch = "arm")]
pub(crate) mod a32;

pub mod graphics;

pub use graphics::*;

pub static mut ARMFB: ArmFb = ArmFb {
    first_init: true,
};

pub struct ArmFb {
    first_init: bool,
}

impl ArmFb {
    pub fn init(&mut self) {
        #[cfg(target_arch = "aarch64")]
        a64::a64_fb_init();
    }
}

pub fn armfb_init() {
    unsafe { ARMFB.init(); }
}

// module_init!(gpug_init, armfb_init);

pub fn armfb_end() {
    unsafe { ARMFB.first_init = false; }
}

// module_end!(gpug_end, armfb_end);
