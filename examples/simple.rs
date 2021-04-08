use libseat::{LogLevel, Seat};

fn main() {
    libseat::set_log_level(LogLevel::Debug);

    let active = std::rc::Rc::new(std::cell::RefCell::new(false));

    let a1 = active.clone();
    let a2 = active.clone();
    let (seat, guard) = Seat::open(
        move |seat| {
            println!("Enable");
            unsafe { println!("Name: {}", seat.name()) };

            *a1.borrow_mut() = true;
        },
        move |seat| {
            println!("Disable");

            *a2.borrow_mut() = false;
            unsafe { seat.disable() };
        },
    );

    if let Some(mut seat) = seat {
        while !(*active.borrow()) {
            println!("waiting for activation...n");
            unsafe { seat.dispatch(-1) };
        }

        unsafe { seat.close() };
    }

    // TODO: This guard is stupid, callback should live as long as seat is alive
    drop(guard);
}
