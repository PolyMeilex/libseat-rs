use libseat::{Seat, SeatEvent};

use std::{cell::RefCell, rc::Rc};

use slog::*;
use std::sync::Mutex;

fn main() {
    let logger = slog::Logger::root(Mutex::new(slog_term::term_full().fuse()).fuse(), o!());

    let active = Rc::new(RefCell::new(false));

    let seat = {
        let active = active.clone();
        Seat::open(
            move |seat, event| match event {
                SeatEvent::Enable => {
                    println!("Enable");
                    println!("Name: {}", seat.name());

                    *active.borrow_mut() = true;
                }
                SeatEvent::Disable => {
                    println!("Disable");

                    *active.borrow_mut() = false;
                    seat.disable().unwrap();
                }
            },
            logger,
        )
    };

    if let Ok(mut seat) = seat {
        while !(*active.borrow()) {
            println!("waiting for activation...n");
            seat.dispatch(-1).unwrap();
        }

        // Close seat
        drop(seat);
    }
}
