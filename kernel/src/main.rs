#![feature(pointer_byte_offsets)]
#![no_main]
#![no_std]



use core::panic::PanicInfo;

use common_lib::{frame_buffer::FrameBufferConfig, vector::Vector2D};
use kernel_lib::gop::{
    char::{ascii_char_writer::AscIICharWriter, char_writable::CharWritable},
    pixel::{pixel_color::PixelColor, select_writer_from},
};


#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_config: FrameBufferConfig) -> () {


    let color = PixelColor::new(0xFF, 0xFF, 00);
    let mut writer = select_writer_from(frame_buffer_config);
    let mut c_writer = AscIICharWriter::new();

    (0x20..=0x7Eu8)
        .enumerate()
        .for_each(|(i, code)| c_writer.write(char::from(code),
                                             Vector2D::new((i * 8) as isize, 50),
                                             &color,
                                             &mut writer).unwrap());

    common_lib::assembly::hlt_forever();
}

#[allow(dead_code)]
unsafe fn fill_display_with_white(frame_buffer_base_addr: u64, frame_buffer_size: usize) {
    let frame_buffer = frame_buffer_base_addr as *mut u8;
    let frame_buffer = core::slice::from_raw_parts_mut(frame_buffer, frame_buffer_size);
    for i in 0..frame_buffer_size {
        frame_buffer[i] = 0xFF;
    }
}

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
