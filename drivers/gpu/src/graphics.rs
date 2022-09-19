use novuskinc::module::*;

#[cfg(target_arch = "arm")]
pub use armfb::{graphics_pixel, graphics_write};

pub struct GpuGraphics;

#[cfg(not(target_arch = "arm"))]
extern "C" {
    pub fn graphics_write(x: usize, y: usize, color: usize, string: &str);
    pub fn graphics_pixel(x: usize, y: usize, color: usize);
}

impl GpuGraphics {
    pub fn new() -> Self {
        return GpuGraphics;
    }

    pub fn write_string(&self, x: usize, y: usize, color: usize, string: &str) {
        unsafe { graphics_write(x, y, color, string); }
    }

    pub fn draw_pixel(&self, x: usize, y: usize, color: usize) {
        unsafe { graphics_pixel(x, y, color); }
    }
}

fn _init_gpug(km: &mut KernelModule) {
    #[cfg(target_arch = "aarch64")]
    armfb::armfb_init();

    #[cfg(target_arch = "x86_64")]
    vgag::vgag_init();

    km.initialized();
}

module_init!(ModuleType::InKernel, gpug);

fn _end_gpug(km: &mut KernelModule) {
    #[cfg(target_arch = "aarch64")]
    armfb::armfb_end();

    #[cfg(target_arch = "x86_64")]
    vgag::vgag_end();

    km.success();
}

module_end!(ModuleType::InKernel, gpug);
