use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::request_data_types::{NewItem, NewOrderItem, Table, OrderItem};

pub fn random_string() -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
}

pub fn random_numb(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min..=max);
}

pub fn generate_order_items() -> Vec<NewOrderItem> {
    let mut items = Vec::<NewOrderItem>::new();
    let mut rng = rand::thread_rng();
    let item_count = rand::thread_rng().gen_range(1..=5);

    for _item in 0..item_count {
        items.push(NewOrderItem {
            item_id: rng.gen_range(1..=5),
            quantity: rng.gen_range(1..=15),
        });
    }
    items
}

pub fn get_random_id_for_generated_items(item_count: i32) -> i32{
    let mut rng = rand::thread_rng();
    return rng.gen_range(1..=item_count);
}

pub async fn generate_random_item_for_update(item_count: i32) -> OrderItem {
    let mut rng = rand::thread_rng();

    OrderItem {
        item_id: get_random_id_for_generated_items(item_count),
        quantity: rng.gen_range(1..=15),
        served: false
    }
}