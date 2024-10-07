#![no_std]
#![no_main]

use core::panic::PanicInfo;
use embedded_alloc::LlffHeap as Heap;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[allow(unused_imports)]
use swiftness_air;
#[allow(unused_imports)]
use swiftness_commitment;
#[allow(unused_imports)]
use swiftness_pow;
#[allow(unused_imports)]
use swiftness_stark;
