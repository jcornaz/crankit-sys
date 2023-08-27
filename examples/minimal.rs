//! This example is an empty game, and is here to shaw case what is the minimum setup required
//! setup to get a working (albeit empty) playdate app.
//!
//! The example can be run using the [crank cli](https://github.com/pd-rs/crank): `crank run --example minimal`

#![no_std]

use core::ptr;
use crankit_sys::{PDSystemEvent, PlaydateAPI};

#[no_mangle]
extern "C" fn eventHandler(
    playdate: *mut PlaydateAPI,
    event: PDSystemEvent,
    _arg: u32,
) -> core::ffi::c_int {
    if event == PDSystemEvent::kEventInit {
        // Initialize game state here..
        unsafe { (*(*playdate).system).setUpdateCallback.unwrap()(Some(update), ptr::null_mut()) }
    }
    0
}

extern "C" fn update(_user_data: *mut core::ffi::c_void) -> i32 {
    // Your update logic here..
    1
}

#[panic_handler]
#[cfg(not(test))]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
