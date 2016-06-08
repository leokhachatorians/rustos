#![feature(lang_items)]
#![feature(const_fn, unique)]

#![no_std]
extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    vga_buffer::clear_screen();
    //use core::fmt::Write;
    //vga_buffer::WRITER.lock().write_str("Hello Again");
    //write!(vga_buffer::WRITER.lock(), ",d adwd {}", 23);
    //vga_buffer::print_something();
    
    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("Memory Areas: ");
    for area in memory_map_tag.memory_areas() {
        println!("      start: 0x{:x}, length: 0x{:x}",
                 area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("kernel sections");
    for section in elf_sections_tag.sections() {
        println!("      addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
                 section.addr, section.size, section.flags);
    }

    let kernal_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("Kernal Start: 0x{:x}, Kernal End: 0x{:x}", kernal_start, kernel_end);
    println!("Multiboot Start: 0x{:x}, Multiboot End: 0x{:x}", multiboot_start, multiboot_end);

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("  {}", fmt);
    loop{}
}

