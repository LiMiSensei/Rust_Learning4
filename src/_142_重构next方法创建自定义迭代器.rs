struct Booking {
    date: String,
    guest_name: String,
    room_bumber: u32,
}

struct BookingOnDate<'a> {
    date: &'a str,
    all_bookings: &'a Vec<Booking>,
    index: usize,
}

impl<'a> BookingOnDate<'a> {
    fn new(date: &'a str, all_bookings: &'a Vec<Booking>) -> Self {
        let date1 = BookingOnDate {
            date,
            all_bookings,
            index: 0,
        };
        date1
    }
}

impl<'a> Booking {
    fn new(date: String, guest_name: String) -> Self {
        let booking = Booking {
            date,
            guest_name,
            room_bumber: 0,
        };
        booking
    }
}

impl<'a> Iterator for BookingOnDate<'a> {
    type Item = &'a Booking;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.all_bookings.len() {
            let booking = &self.all_bookings[self.index];
            self.index += 1;
            if self.date == booking.date {
                return Some(booking);
            }
        }

        None
    }
}
#[allow(dead_code)]
pub fn main() {
    let mut number = ["One".to_string(), "Two".to_string(), "Three".to_string()];

    let iterate_by_immutable_borrow = number[..].iter();

    for i in iterate_by_immutable_borrow {
        println!("{}", i);
    }

    let a = [1, 2, 3, 4];
    let ret = a.iter().find(|&x| *x < 0);
}
