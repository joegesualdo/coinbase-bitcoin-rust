// Source: https://developers.coinbase.com/docs/wallet/guides/price-data
use crate::currencies::{Currency, FiatCurrency};
use crate::request;
use anyhow::{Ok, Result};
use serde::Deserialize;
use FiatCurrency::*;

mod urls;

#[derive(Debug, Deserialize)]
pub struct BaseResponse {
    base: String,
    currency: String,
    pub amount: String,
}

#[derive(Debug, Deserialize)]
pub struct CoinbasePriceResponse {
    pub data: BaseResponse,
}

pub type SpotPriceResponse = CoinbasePriceResponse;
pub type BuyPriceResponse = CoinbasePriceResponse;
pub type SellPriceResponse = CoinbasePriceResponse;

pub fn request_spot_price() -> Result<SpotPriceResponse> {
    let currency: Currency = Currency::FiatCurrency(USD);
    let request_url: String = urls::get_spot_price_url(currency);
    let response: CoinbasePriceResponse = request::request(request_url)?;
    return Ok(response);
}

pub fn request_buy_price() -> Result<BuyPriceResponse> {
    let currency: Currency = Currency::FiatCurrency(USD);
    let request_url: String = urls::get_buy_price_url(currency);
    let response: CoinbasePriceResponse = request::request(request_url)?;
    return Ok(response);
}

pub fn request_sell_price() -> Result<SellPriceResponse> {
    let currency: Currency = Currency::FiatCurrency(USD);
    let request_url: String = urls::get_sell_price_url(currency);
    let response: CoinbasePriceResponse = request::request(request_url)?;
    return Ok(response);
}
