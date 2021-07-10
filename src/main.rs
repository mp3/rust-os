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

  let ptr = 0xdeadbeaf as *mut u32;
  unsafe { *ptr = 42; }

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

