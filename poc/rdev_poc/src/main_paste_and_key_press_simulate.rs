//https://docs.rs/rdev/latest/rdev/enum.Key.html
use rdev::{simulate, EventType, Key, SimulateError};
use std::{thread, time};

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

fn main() {
    // Paste
    // Simulate the key combination for "Paste"
    #[cfg(target_os = "macos")]
    {
        // macOS uses the Command key (⌘) for shortcuts
        send(&EventType::KeyPress(Key::MetaLeft));
        send(&EventType::KeyPress(Key::KeyV));
        send(&EventType::KeyRelease(Key::KeyV));
        send(&EventType::KeyRelease(Key::MetaLeft));
    }

    #[cfg(all(
        not(target_os = "macos"),
        any(target_os = "linux", target_os = "windows")
    ))]
    {
        // Windows and Linux use the Ctrl key for shortcuts
        send(&EventType::KeyPress(Key::ControlLeft));
        send(&EventType::KeyPress(Key::KeyV));
        send(&EventType::KeyRelease(Key::KeyV));
        send(&EventType::KeyRelease(Key::ControlLeft));
    }
    // End paste

    // send(&EventType::KeyPress(Key::KeyS));
    // send(&EventType::KeyRelease(Key::KeyS));
    //
    // send(&EventType::KeyPress(Key::KeyA));
    // send(&EventType::KeyRelease(Key::KeyA));
    //
    // send(&EventType::KeyPress(Key::KeyA));
    // send(&EventType::KeyRelease(Key::KeyA));
    //
    // send(&EventType::KeyPress(Key::KeyA));
    // send(&EventType::KeyRelease(Key::KeyA));
    //
    // send(&EventType::KeyPress(Key::KeyA));
    // send(&EventType::KeyRelease(Key::KeyA));

    // send(&EventType::MouseMove { x: 0.0, y: 0.0 });
    // send(&EventType::MouseMove { x: 400.0, y: 400.0 });
    // send(&EventType::ButtonPress(Button::Left));
    // send(&EventType::ButtonRelease(Button::Right));
    // send(&EventType::Wheel {
    //     delta_x: 0,
    //     delta_y: 1,
    // });
}
