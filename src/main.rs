use std::sync::{Arc, Mutex};
use inputbot::{KeybdKey::*, MouseButton::*, MouseCursor};

#[macro_use]
mod macros;

fn main() {
    let macro_mode = Arc::new(Mutex::new(false));

    combination!(macro_mode, UpKey, NumLockKey, {
        let (x, y) = MouseCursor::pos();

        // Supports up to 8k (temp impl)
        MouseCursor::move_abs(8000, 4500);
        LeftButton.press();
        LeftButton.release();

        MouseCursor::move_abs(x, y);
    });

    if cfg!(debug_assertions) {
        let macro_mode_clone = macro_mode.clone();
        DownKey.bind(move || {
            println!("macros status -> {:?}", macro_mode_clone.lock().unwrap());
        });
    }

    toggle_combination!(macro_mode, LShiftKey, RShiftKey);

    inputbot::handle_input_events();
}
