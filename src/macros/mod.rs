#[macro_export]
macro_rules! toggle_combination {
    ($m:ident, $c1:ident, $c2:ident) => {
        use notify_rust::{Notification, Timeout};

        let macro_mode_clone = $m.clone();
        $c1.bind(move || {
            if $c2.is_pressed() {
                let mut locked = macro_mode_clone.lock().unwrap();
                *locked = !*locked;

                if *locked {
                    let _ = Notification::new()
                        .summary("Macros enabled")
                        .body("Macros are now enabled")
                        .timeout(Timeout::Milliseconds(500))
                        .show()
                        .unwrap();
                } else {
                    let _ = Notification::new()
                        .summary("Macros disabled")
                        .body("Macros are now disabled")
                        .timeout(Timeout::Milliseconds(500))
                        .show()
                        .unwrap();
                }
            }
        });

        let macro_mode_clone = $m.clone();
        $c2.bind(move || {
            if $c2.is_pressed() {
                let mut locked = macro_mode_clone.lock().unwrap();
                *locked = !*locked;

                if *locked {
                    let _ = Notification::new()
                        .summary("Macros enabled")
                        .body("Macros are now enabled")
                        .timeout(Timeout::Milliseconds(500))
                        .show()
                        .unwrap();
                } else {
                    let _ = Notification::new()
                        .summary("Macros disabled")
                        .body("Macros are now disabled")
                        .timeout(Timeout::Milliseconds(500))
                        .show()
                        .unwrap();
                }
            }
        });
    }
}

#[macro_export]
macro_rules! raw_combination {
    ($c1:ident, $c2:ident, $e:expr) => {
        $c2.bind(move || {
            if $c1.is_pressed() {
                $e
            }
        });
    }
}

#[macro_export]
macro_rules! combination {
    ($m:ident, $c1:ident, $c2:ident, $e:expr) => {
        let macro_mode_clone = $m.clone();
        raw_combination!($c1, $c2, {
            if *macro_mode_clone.lock().unwrap() {
                $e
            }
        });

        let macro_mode_clone = $m.clone();
        raw_combination!($c2, $c1, {
            if *macro_mode_clone.lock().unwrap() {
                $e
            }
        })
    }
}
