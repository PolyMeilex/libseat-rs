use libseat::{LogLevel, Seat};

fn main() {
    libseat::set_log_level(LogLevel::Debug);

    let active = std::rc::Rc::new(std::cell::RefCell::new(false));

    let a1 = active.clone();
    let a2 = active.clone();
    let seat = Seat::open(
        move |seat| {
            println!("Enable");
            println!("Name: {}", seat.name());

            *a1.borrow_mut() = true;
        },
        move |seat| {
            println!("Disable");

            *a2.borrow_mut() = false;
            seat.disable();
        },
    );

    if let Some(mut seat) = seat {
        while !(*active.borrow()) {
            println!("waiting for activation...n");
            seat.dispatch(-1);
        }

        // Close seat
        drop(seat);
    }
}
