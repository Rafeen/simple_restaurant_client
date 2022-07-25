# Simple Restaurant Client

## Context

This git repository contains the client application for this [application](https://github.com/Rafeen/simple_restaurant_api).

## Prerequisites

- Rust
- Simple Restaurant Api([server](https://github.com/Rafeen/simple_restaurant_api))


## Build & Launch the Application
Before running the client application its required to run the server application first.
get the server application from [here](https://github.com/Rafeen/simple_restaurant_api).
To build and run this application, apply theses commands in the project folder
```sh
cargo build
cargo run [number_of_tablets]
```
`[number_of_tablets]` denotes how many devices will be used to perform simultaneous CRUD operation to the server.
Example: `cargo run 15`

## Explanation
Each tablet wil have its own seperate thread and will perform `15` random request on random intervals to simulate order operations from restaurant tables.

##Console Output
For each request performed two lines will be printed in the console
- line 1 prints request method `Ex: GET, DELETE, POST, PUT` and `url `
```sh
[GET] : http://127.0.0.1:3000/order/4
```

- line 2 prints request method `Ex: GET, DELETE, POST, PUT`  and `response details` 
```sh
[GET] all items for table 4 : "{\"data\":[{\"item_id\":4,\"quantity\":9,\"served\":false},{\"item_id\":5,\"quantity\":14,\"served\":false}]}"

```


## License

MIT


