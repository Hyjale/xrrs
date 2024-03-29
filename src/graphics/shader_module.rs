use std::{
    io::Cursor,
    sync::{Arc}
};

use ash::{
    util::read_spv,
    vk::{self}
};

use crate::graphics::{
    device::Device
};

pub struct ShaderModule {}

impl ShaderModule {
    pub fn new(device: &Arc<Device>, path: &[u8]) -> ash::vk::ShaderModule {
        let code = read_spv(&mut Cursor::new(path)).unwrap();

        unsafe {
            let shader_module = device
                .handle
                .create_shader_module(&vk::ShaderModuleCreateInfo::builder().code(&code), None)
                .unwrap();

            shader_module
        }
    }
}
