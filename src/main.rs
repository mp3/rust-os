#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
  serial_println!("Running {} tests", tests.len());
  for test in tests {
      test();
  }

  exit_qemu(QemuExitCode::Success);
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  let vga_buffer = 0xb8000 as *mut u8;

  for (i, &byte) in HELLO.iter().enumerate() {
    unsafe {
      *vga_buffer.offset(i as isize * 2) = byte;
      *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }
  }

  use core::fmt::Write;
  vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
  write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();

  println!("Hello World{}", "!");

  #[cfg(test)]
  test_main();

  loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  serial_println!("[failed]\n");
  serial_println!("Error: {}\n", info);
  exit_qemu(QemuExitCode::Failed);
  loop {}
}

#[test_case]
fn trivial_assertion() {
  serial_print!("trivial assertion...");
  assert_eq!(1, 1);
  serial_println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
  Success = 0x10,
  Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
  use x86_64::instructions::port::Port;

  unsafe {
    let mut port = Port::new(0xf4);
    port.write(exit_code as u32);
  }
}
