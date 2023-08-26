#![no_std]

use core::ptr;

use libm::{cosf, sinf};

use crankit_sys::{LCDBitmapFlip, LCDColor, LCDSolidColor, LCDSprite, PDSystemEvent, PlaydateAPI};

static mut GAME: Option<Game> = None;

struct Game {
    playdate: *mut PlaydateAPI,
    sprite: *mut LCDSprite,
}

impl Game {
    unsafe fn init(playdate: *mut PlaydateAPI) -> Self {
        let gfx = (*playdate).graphics;
        let bitmap = (*gfx).newBitmap.unwrap()(10, 10, LCDSolidColor::kColorClear as LCDColor);
        (*gfx).pushContext.unwrap()(bitmap);
        (*gfx).fillEllipse.unwrap()(
            0,
            0,
            10,
            10,
            0.0,
            360.0,
            LCDSolidColor::kColorBlack as LCDColor,
        );
        (*gfx).popContext.unwrap()();
        let sprite_manager = (*playdate).sprite;
        let sprite = (*sprite_manager).newSprite.unwrap()();
        (*sprite_manager).setImage.unwrap()(sprite, bitmap, LCDBitmapFlip::kBitmapUnflipped);
        (*sprite_manager).moveTo.unwrap()(sprite, 200.0, 120.0);
        Self { playdate, sprite }
    }

    unsafe fn update(&mut self) {
        let crank_angle = (*(*self.playdate).system).getCrankAngle.unwrap()().to_radians();
        let dx = cosf(crank_angle);
        let dy = sinf(crank_angle);
        let mut x = 0.0;
        let mut y = 0.0;
        let sprite_manager = (*self.playdate).sprite;
        (*sprite_manager).getPosition.unwrap()(
            self.sprite,
            ptr::addr_of_mut!(x),
            ptr::addr_of_mut!(y),
        );
        (*sprite_manager).moveBy.unwrap()(self.sprite, dx, dy);
        (*sprite_manager).drawSprites.unwrap()();
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
            GAME = Some(Game::init(playdate));
            (*(*playdate).system).setUpdateCallback.unwrap()(Some(update), ptr::null_mut())
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

#[panic_handler]
#[cfg(not(test))]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
