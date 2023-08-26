#![no_std]

extern crate alloc;

use alloc::ffi::CString;
use alloc::format;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

use libm::{cosf, sinf};

use crankit_sys::{
    playdate_graphics, playdate_sprite, playdate_sys, LCDBitmapFlip, LCDColor, LCDSolidColor,
    LCDSprite, PDSystemEvent, PlaydateAPI,
};

static mut PLAYDATE: Option<*mut PlaydateAPI> = None;
static mut GAME: Option<Game> = None;

unsafe fn gfx() -> *const playdate_graphics {
    (*PLAYDATE.unwrap()).graphics
}

unsafe fn system() -> *const playdate_sys {
    (*PLAYDATE.unwrap()).system
}

unsafe fn sprites() -> *const playdate_sprite {
    (*PLAYDATE.unwrap()).sprite
}

struct Game {
    sprite: *mut LCDSprite,
}

impl Game {
    unsafe fn init() -> Self {
        let bitmap = (*gfx()).newBitmap.unwrap()(10, 10, LCDSolidColor::kColorWhite as LCDColor);
        (*gfx()).pushContext.unwrap()(bitmap);
        (*gfx()).fillEllipse.unwrap()(
            0,
            0,
            10,
            10,
            0.0,
            360.0,
            LCDSolidColor::kColorBlack as LCDColor,
        );
        (*gfx()).popContext.unwrap()();
        let sprite = (*sprites()).newSprite.unwrap()();
        (*sprites()).setImage.unwrap()(sprite, bitmap, LCDBitmapFlip::kBitmapUnflipped);
        (*sprites()).moveTo.unwrap()(sprite, 200.0, 120.0);
        (*sprites()).addSprite.unwrap()(sprite);
        Self { sprite }
    }

    unsafe fn update(&mut self) {
        (*gfx()).clear.unwrap()(LCDSolidColor::kColorWhite as LCDColor);
        let crank_angle = (*system()).getCrankAngle.unwrap()().to_radians();
        let mut x = 0.0;
        let mut y = 0.0;
        (*sprites()).getPosition.unwrap()(self.sprite, ptr::addr_of_mut!(x), ptr::addr_of_mut!(y));
        x = (x + sinf(crank_angle)).clamp(5.0, 395.0);
        y = (y - cosf(crank_angle)).clamp(5.0, 235.0);
        (*sprites()).moveTo.unwrap()(self.sprite, x, y);
        let string = CString::new(format!("position: ({x:.0}, {y:.0})")).unwrap();
        (*system()).logToConsole.unwrap()(string.as_ptr());
        (*sprites()).drawSprites.unwrap()();
    }
}

#[no_mangle]
extern "C" fn eventHandler(
    playdate: *mut PlaydateAPI,
    event: PDSystemEvent,
    _arg: u32,
) -> core::ffi::c_int {
    if event == PDSystemEvent::kEventInit {
        unsafe {
            PLAYDATE = Some(playdate);
            GAME = Some(Game::init());
            (*system()).setUpdateCallback.unwrap()(Some(update), ptr::null_mut())
        }
    }
    0
}

extern "C" fn update(_user_data: *mut core::ffi::c_void) -> i32 {
    unsafe {
        GAME.as_mut().unwrap().update();
    }
    1
}

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        (*system()).realloc.unwrap()(ptr::null_mut(), layout.size()).cast::<u8>()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        (*system()).realloc.unwrap()(ptr.cast::<core::ffi::c_void>(), 0);
    }
}

#[global_allocator]
static mut GLOBAL_ALLOCATOR: Allocator = Allocator;

#[panic_handler]
#[cfg(not(test))]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
