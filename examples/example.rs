use coinbase_bitcoin;
fn main() {
    let coinbase_price = coinbase_bitcoin::get_price_data();
    match coinbase_price {
        Ok(price) => {
            let coinbase_price_string = price.spot.to_string();
            println!("coinbase: {}", coinbase_price_string);
        }
        Err(err) => {
            println!("Error fetching coinbase price: {}", err);
        }
    }
}
