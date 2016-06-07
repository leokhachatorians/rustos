#![feature(lang_items)]
#![feature(const_fn, unique)]

#![no_std]
extern crate rlibc;

mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
   // let hello = b"Come at me brah!";
   // let color_byte = 0x1f;

   // let mut hello_color = [color_byte; 32];
   // for (i, char_byte) in hello.into_iter().enumerate() {
   //     hello_color[i*2] = *char_byte;
   // }

   // // write Hello World to the center of the VGA text buffer
   // let buffer_ptr = (0xb8000 + 1988) as *mut _;
   // unsafe { *buffer_ptr = hello_color };
    vga_buffer::print_something();

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

