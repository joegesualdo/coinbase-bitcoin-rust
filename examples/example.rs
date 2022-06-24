use coinbase_bitcoin;
fn main() {
    let coinbase_price = coinbase_bitcoin::get_price_data();
    let coinbase_price_string = coinbase_price.spot.to_string();
    println!("coinbase: {}", coinbase_price_string);
}
