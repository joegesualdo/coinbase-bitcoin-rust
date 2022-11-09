use anyhow::{Ok, Result};

mod api;
mod currencies;
mod request;

pub type Price = f32;

pub struct PriceData {
    pub spot: Price,
    pub buy: Price,
    pub sell: Price,
}

pub fn get_price_data() -> Result<PriceData> {
    let spot = get_spot_price()?;
    let buy = get_buy_price()?;
    let sell = get_buy_price()?;

    let price_data = PriceData { spot, buy, sell };
    Ok(price_data)
}
pub fn get_spot_price() -> Result<Price> {
    let response: api::v2::SpotPriceResponse = api::v2::request_spot_price()?;
    let price: f32 = response.data.amount.parse()?;
    return Ok(price);
}
pub fn get_buy_price() -> Result<Price> {
    let response: api::v2::BuyPriceResponse = api::v2::request_buy_price()?;
    let price: f32 = response.data.amount.parse()?;
    return Ok(price);
}
pub fn get_sell_price() -> Result<Price> {
    let response: api::v2::SellPriceResponse = api::v2::request_sell_price()?;
    let price: f32 = response.data.amount.parse()?;
    return Ok(price);
}
