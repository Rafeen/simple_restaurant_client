mod random_generators;
mod request_data_types;

use crate::random_generators::{
    random_numb, random_string,
    generate_order_items,
    get_random_id_for_generated_items,
    generate_random_item_for_update,
};
use crate::request_data_types::{NewOrderItem, NewItem, Table, OrderItem};
use std::{env, thread};
use std::time::Duration;
use threadpool::ThreadPool;
use reqwest::Client;
use reqwest::Error;
use tokio::runtime::Runtime;


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Number of Tablets: cargo run [tablet_count]");
    }
    let total_tablet_count :usize = args[1].parse().unwrap();

    let pool = ThreadPool::with_name("tablets".into(), total_tablet_count);
    for table in 1..=total_tablet_count {
        pool.execute(move || {
            let runtime = Runtime::new().unwrap();
            let table_id = table as i32;

            if table_id > 13 {
                match runtime.block_on(async {
                    create_table(Table { id: table_id }).await
                })
                {
                    Ok(_) => {}
                    Err(e) => println!("Error : {}", e),
                }
            }

            // each thread will perform 15 random requests
            for _ in 1..=15 {
                let order_item = generate_order_items();
                let request_type: i32 = random_numb(1, 6);
                let interval: u64 = random_numb(1, 20) as u64;
                match request_type {
                    1 => match runtime.block_on(async {
                        create_order_for_table(
                            table_id,
                            order_item.clone(),
                        ).await
                    })
                    {
                        Ok(_) => {}
                        Err(e) => println!("Error : {}", e),
                    }
                    2 => match runtime.block_on(async {
                        get_table_order_by_item(
                            table_id,
                            get_random_id_for_generated_items(order_item.len() as i32),
                        ).await
                    })
                    {
                        Ok(_) => {}
                        Err(e) => println!("Error : {}", e),
                    }
                    3 => match runtime.block_on(async {
                        delete_order_item(
                            table_id,
                            get_random_id_for_generated_items(order_item.len() as i32),
                        ).await
                    })
                    {
                        Ok(_) => {}
                        Err(e) => println!("Error : {}", e),
                    }
                    4 => match runtime.block_on(async {
                        update_table_order(
                            table_id,
                            generate_random_item_for_update(order_item.len() as i32).await,
                        ).await
                    })
                    {
                        Ok(_) => {}
                        Err(e) => println!("Error : {}", e),
                    }
                    5 => match runtime.block_on(async {
                        get_remaining_orders_for_table(table_id).await
                    })
                    {
                        Ok(_) => {}
                        Err(e) => println!("Error : {}", e),
                    }
                    6 => match runtime.block_on(async {
                        get_all_order_items_by_table_id(table_id).await
                    })
                    {
                        Ok(_) => {}
                        Err(e) => println!("Error : {}", e),
                    }
                    _ => println!("[ERROR] Number out limit !"),
                }
                thread::sleep(Duration::from_millis(interval * 1000));
            }
        });
    }
    pool.join();
}

async fn create_order_for_table(table_id: i32, order_items: Vec<NewOrderItem>) -> Result<(), Error> {
    let table_id = table_id.clone();

    let url: String = format!("http://127.0.0.1:3000/order/{}", table_id);
    let executor = Client::new();
    let resp = executor
        .post(&url)
        .json(&order_items)
        .send()
        .await?;
    let msg = &resp.text().await?;
    println!("[POST] : {}", url);
    println!("[POST] Order for table {} :  {:?}", table_id, msg);
    Ok(())
}

async fn get_table_order_by_item(table_id: i32, item_id: i32) -> Result<(), Error> {
    let url: String = format!("http://127.0.0.1:3000/order/{}/{}", table_id, item_id);
    let executor = Client::new();
    let resp = executor
        .get(&url)
        .send()
        .await?;
    let msg = resp.text().await?.to_string();
    println!("[GET] : {}", url);
    println!("[GET] order item {} for table {} : {:?}", item_id, table_id, msg);
    Ok(())
}

async fn get_remaining_orders_for_table(table_id: i32) -> Result<(), Error> {
    let url: String = format!("http://127.0.0.1:3000/order/{}/remaining", table_id);
    let executor = Client::new();
    let resp = executor
        .get(&url)
        .send()
        .await?;
    let msg = resp.text().await?.to_string();
    println!("[GET] : {}", url);
    println!("[GET] remaining order items for table {} : {:?}", table_id, msg);
    Ok(())
}

async fn delete_order_item(table_id: i32, item_id: i32) -> Result<(), Error> {
    let url: String = format!("http://127.0.0.1:3000/order/{}/{}", table_id, item_id);
    let executor = Client::new();
    let resp = executor
        .delete(&url)
        .send()
        .await?;
    let msg = resp.text().await?.to_string();
    println!("[DELETE] : {}", url);
    println!("[DELETE] order item {} for table {} : {:?}", item_id, table_id, msg);
    Ok(())
}

async fn update_table_order(table_id: i32, item: OrderItem) -> Result<(), Error> {
    let url: String = format!("http://127.0.0.1:3000/order/{}", table_id);
    let executor = Client::new();
    let resp = executor
        .put(&url)
        .json(&item)
        .send()
        .await?;
    let msg = resp.text().await?.to_string();
    println!("[UPDATE] : {}", url);
    println!("[UPDATE] item {} for table {} : {:?}", item.item_id, table_id, msg);
    Ok(())
}

async fn get_all_order_items_by_table_id(table_id: i32) -> Result<(), Error> {
    let url: String = format!("http://127.0.0.1:3000/order/{}", table_id);

    let executor = Client::new();
    let resp = executor
        .get(&url)
        .send()
        .await?;
    let msg = resp.text().await?.to_string();
    println!("[GET] : {}", url);
    println!("[GET] all items for table {} : {:?}", table_id, msg);
    Ok(())
}

async fn create_table(table: Table) -> Result<(), Error> {
    let url: String = format!("http://127.0.0.1:3000/table/");

    let executor = Client::new();
    let resp = executor
        .post(&url)
        .json(&table)
        .send()
        .await?;
    let msg = resp.text().await?.to_string();
    println!("[POST] : {}", url);
    println!("[POST] created table  : {:?}", msg);
    Ok(())
}