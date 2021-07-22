#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rusty_os::println;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");

  rusty_os::init();

  use x86_64::registers::control::Cr3;

  let (level_4_page_table, _) = Cr3::read();
  println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

  let ptr = 0x20402b as *mut u32;

  unsafe { let x = *ptr; } 
  println!("read worked");

  unsafe { *ptr = 42; }
  println!("write worked");

  fn stack_overflow() {
    stack_overflow();
  }

  stack_overflow();

  #[cfg(test)]
  test_main();

  println!("It did not crash!");
  rusty_os::hlt_loop();
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
  rusty_os::test_panic_handler(info);
  loop {}
}

#[test_case]
fn trivial_assertion() {
  assert_eq!(1, 1);
}

