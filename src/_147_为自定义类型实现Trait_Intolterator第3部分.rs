#[derive(Debug, Clone)]
struct Car {
    make: String,
    model: String,
    price: u32,
}

#[derive(Debug, Clone)]
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

impl IntoIterator for CarCollection {
    type Item = Car;
    type IntoIter = CarPriceRangeIteratorByValue;

    fn into_iter(self) -> Self::IntoIter {
        CarPriceRangeIteratorByValue {
            reamaining_cars: self.cars.into_iter(),
            price_range: self.price_range,
        }
    }
}

#[cfg(feature = "not_yet_ready")]
impl IntoIterator for &CarCollection {
    type Item = &Car;
    type IntoIter = Self::Item;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

struct CarPriceRangeIteratorByValue {
    reamaining_cars: std::vec::IntoIter<Car>,
    price_range: (u32, u32),
}

impl Iterator for CarPriceRangeIteratorByValue {
    type Item = Car;

    fn next(&mut self) -> Option<Self::Item> {
        self.reamaining_cars.find(|car| (*car).price >= self.price_range.0 &&
            (*car).price <= self.price_range.1);
        /*while let Some(car) = self.reamaining_cars {
            if car.price >= self.price_range.0 && car.price <= self.price_range.1 {
                return Some(car);
            }
        }*/

        None
    }
}

#[allow(dead_code)]
pub fn main() {
    let cars = vec![
        Car {
            make: "Maruti Suzuki".to_string(),
            model: "Swift".to_string(),
            price: 8000,
        },
        Car {
            make: "Honda".to_string(),
            model: "City".to_string(),
            price: 12000,
        },
        Car {
            make: "Tata Motors".to_string(),
            model: "Nexon".to_string(),
            price: 10000,
        },
    ];

    let car_collection_1 = CarCollection::new(cars.clone(), (8000, 10000));
    let mut car_collection_2 = CarCollection::new(cars.clone(), (8000, 10000));

    for car in car_collection_1.into_iter() {
        println!("{},{},{}", car.make, car.model, car.price)
    }
    /*
    for car in &mut car_collection_2.into_iter() {
        println!("{},{},{}",car.make,car.model,car.price)
    }*/
}
