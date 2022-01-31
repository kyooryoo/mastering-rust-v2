use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Dimension {
    item: String,
    color: String,
    size: String,
    part_number: String
}

#[derive(Debug)]
struct Delivery {
    variant: Dimension,
    location: String,
    quantity: i32,
}

pub fn main() {
    let data = vec![
        Delivery {
            variant: Dimension {item: String::from("A"),
                                color: String::from("B"),
                                size: String::from("C"),
                                part_number: String::from("D")},
            location: String::from("L"),
            quantity: 10,
        },
        Delivery {
            variant: Dimension {item: String::from("A"),
                                color: String::from("B"),
                                size: String::from("C"),
                                part_number: String::from("D")},
            location: String::from("L2"),
            quantity: 3,
        },        
        Delivery {
            variant: Dimension {item: String::from("A"),
                                color: String::from("B"),
                                size: String::from("C"),
                                part_number: String::from("D1")},
            location: String::from("L"),
            quantity: 5,
        }, 
        Delivery {
            variant: Dimension {item: String::from("A"),
                                color: String::from("B"),
                                size: String::from("C"),
                                part_number: String::from("D")},
            location: String::from("L"),
            quantity: 5,
        },        
    ];

    // The keys of this map will be groups
    let mut map: HashMap<(Dimension, String), Delivery> = HashMap::new();
    for d in data {
        let record = map
            .entry((d.variant.clone(), d.location.clone()))
            .or_insert(Delivery {
                variant: d.variant.clone(),
                location: d.location.clone(),
                quantity: 0,
            });
        record.quantity += d.quantity;
    }

    let mut map_vec: Vec<(&(Dimension, String), &Delivery)> = map.iter().collect();
    map_vec.sort_by(|a, b| b.1.quantity.cmp(&a.1.quantity));

    for (variant, delivery) in &map_vec {
        println!("{:?} has {:?}", variant, delivery.quantity);
    }
}