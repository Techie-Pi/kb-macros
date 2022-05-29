use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use inputbot::{KeybdKey::*, MouseButton::*, MouseButton, MouseCursor};

#[macro_use]
mod macros;

fn main() {
    let macro_mode = Arc::new(Mutex::new(false));

    // Escape HK
    combination!(macro_mode, UpKey, NumLockKey, {
        let (x, y) = MouseCursor::pos();

        // Supports up to 8k (temp impl)
        MouseCursor::move_abs(8000, 4500);
        LeftButton.press();
        LeftButton.release();

        MouseCursor::move_abs(x, y);
    });

    // Bluetooth HK
    combination!(macro_mode, DownKey, UpKey, {
        let (x, y) = MouseCursor::pos();

        open_toolbar();

        MouseCursor::move_abs(1725, 550);
        LeftButton.press();
        LeftButton.release();

        thread::sleep(Duration::from_millis(300));

        MouseCursor::move_abs(x, y);
        LeftButton.press();
        LeftButton.release();
    });

    // Airplane Mode HK
    combination!(macro_mode, LeftKey, RightKey, {
        let (x, y) = MouseCursor::pos();

        open_toolbar();

        MouseCursor::move_abs(1800, 550);
        LeftButton.press();
        LeftButton.release();

        thread::sleep(Duration::from_millis(300));

        MouseCursor::move_abs(x, y);
        LeftButton.press();
        LeftButton.release();
    });

    if cfg!(debug_assertions) {
        let macro_mode_clone = macro_mode.clone();
        RControlKey.bind(move || {
            println!("macros status -> {:?}", macro_mode_clone.lock().unwrap());
            println!("cursor pos -> {:?}", MouseCursor::pos());
        });
    }

    toggle_combination!(macro_mode, LShiftKey, RShiftKey);

    inputbot::handle_input_events();
}


fn open_toolbar() {
    MouseCursor::move_abs(1780, 1050);
    LeftButton.press();
    LeftButton.release();

    thread::sleep(Duration::from_millis(300));
}
