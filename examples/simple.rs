use libseat::{LogLevel, Seat, SeatRef};

use std::sync::Mutex;
use std::{cell::RefCell, rc::Rc};

use slog::*;

fn main() {
    let logger = slog::Logger::root(Mutex::new(slog_term::term_full().fuse()).fuse(), o!());

    libseat::set_log_level(LogLevel::Debug);

    let active = Rc::new(RefCell::new(false));

    let enable = {
        let active = active.clone();
        move |seat: &mut SeatRef| {
            println!("Enable");
            println!("Name: {}", seat.name());

            *active.borrow_mut() = true;
        }
    };

    let disable = {
        let active = active.clone();
        move |seat: &mut SeatRef| {
            println!("Disable");

            *active.borrow_mut() = false;
            seat.disable().unwrap();
        }
    };

    let seat = Seat::open(enable, disable, Some(logger));

    if let Ok(mut seat) = seat {
        while !(*active.borrow()) {
            println!("waiting for activation...n");
            seat.dispatch(-1).unwrap();
        }

        // Close seat
        drop(seat);
    }
}
