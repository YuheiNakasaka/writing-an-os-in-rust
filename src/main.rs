#![no_std]
#![no_main]

#[allow(dead_code)]
static HELLO: &[u8] = b"Hello World!";

use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;
    // for (i, &byte) in HELLO.iter().enumerate() {
    //     // stringをVGAのcolor byteに変換してる
    //     // メモリのポインタに直接アクセスしてるのでunsafeを使ってるが多用すべきでない
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    // write!(
    //     vga_buffer::WRITER.lock(),
    //     ", some numbers: {} {}",
    //     42,
    //     1.337
    // )
    // .unwrap();

    println!("Hello World{}", "!");

    loop {}
}
