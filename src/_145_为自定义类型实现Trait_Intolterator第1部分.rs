#[derive(Debug,Clone)]
struct Car {
    make: String,
    model: String,
    price: u32,
}

#[derive(Debug,Clone)]
struct CarCollection {
    cars: Vec<Car>,
    price_range: (u32, u32),
}

impl CarCollection {
    fn new(cars: Vec<Car>, price_range: (u32, u32)) -> Self {
        CarCollection {
            cars: cars,
            price_range: price_range,
        }
    }
}

impl Iterator for CarCollection {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[allow(dead_code)]
pub fn main() {
    let cars = vec![
        Car{ make: "Maruti Suzuki".to_string(), model: "Swift".to_string(), price: 8000, },
        Car{ make: "Honda".to_string(), model: "City".to_string(), price: 12000, },
        Car{ make: "Tata Motors".to_string(), model: "Nexon".to_string(), price: 10000, },
    ];



    let car_collection_1 = CarCollection::new(cars.clone(), (8000, 10000));
    let mut car_collection_2 = CarCollection::new(cars.clone(), (8000, 10000));

    for car in car_collection_1 {
        //println!("{},{},{}",car.make,car.model,car.price)
    }

    for car in &mut car_collection_2 {
        //println!("{},{},{}",car.make,car.model,car.price)
    }
}
