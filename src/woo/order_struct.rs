use std::fmt::{Display, format, Formatter};
use crate::woo::enums::Side;

// So much duplicate code here find a way to clean this up.

pub trait Params {
    fn get_type() -> String;
}

pub trait Queryable {
    fn to_query_string(&self) -> String;
}

pub struct Limit {
    pub price: f64,
    pub order_quantity: i32,
}

impl Limit {
    pub fn new(price: f64, order_quantity: i32) -> Self {
        Self {
            price,
            order_quantity
        }
    }
}

impl Params for Limit {
    fn get_type() -> String {
        "LIMIT".to_string()
    }
}

impl Display for Limit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LIMIT")
    }
}

pub struct Market {
    pub order_quantity: i32,
}

impl Market {
    pub fn new(order_quantity: i32) -> Self {
        Self {
            order_quantity
        }
    }
}

impl Params for Market {
    fn get_type() -> String {
        "MARKET".to_string()
    }
}

impl Display for Market {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MARKET")
    }
}

pub struct IOC {
    pub price: f64,
    pub order_quantity: i32,
}

impl IOC {
    pub fn new(price: f64, order_quantity: i32) -> Self {
        Self {
            price,
            order_quantity
        }
    }
}

impl Params for IOC {
    fn get_type() -> String {
        "IOC".to_string()
    }
}

impl Display for IOC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IOC")
    }
}

pub struct FOK {
    pub price: f64,
    pub order_quantity: i32,
}

impl FOK {
    pub fn new(price: f64, order_quantity: i32) -> Self {
        Self {
            price,
            order_quantity
        }
    }
}

impl Params for FOK {
    fn get_type() -> String {
        "FOK".to_string()
    }
}

impl Display for FOK {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "FOK")
    }
}

pub struct PostOnly {
    pub price: f64,
    pub order_quantity: i32,
}

impl PostOnly {
    pub fn new(price: f64, order_quantity: i32) -> Self {
        Self {
            price,
            order_quantity
        }
    }
}

impl Params for PostOnly {
    fn get_type() -> String {
        "POST_ONLY".to_string()
    }
}

impl Display for PostOnly {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "POST_ONLY")
    }
}

pub struct Ask {
    pub price: f64,
    pub order_quantity: i32,
}

impl Ask {
    pub fn new(price: f64, order_quantity: i32) -> Self {
        Self {
            price,
            order_quantity
        }
    }
}

impl Params for Ask {
    fn get_type() -> String {
        "ASK".to_string()
    }
}

impl Display for PostOnly {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ASK")
    }
}

pub struct Bid {
    pub price: f64,
    pub order_quantity: i32,
}

impl Bid {
    pub fn new(price: f64, order_quantity: i32) -> Self {
        Self {
            price,
            order_quantity
        }
    }
}

impl Params for Bid {
    fn get_type() -> String {
        "BID".to_string()
    }
}

impl Display for Bid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BID")
    }
}

pub struct Order <T: Params> {
    symbol: String,
    side: Side,
    order_type: String,
    price: Option<f64>,
    broker_id: Option<String>,
    client_order_id: Option<i32>,
    order_tag: Option<String>,
    order_quantity: i32,
    order_amount: Option<i32>,
    reduce_only: Option<bool>,
    visible_quantity: Option<i32>,
    order_params: T
}

impl <T: Params> Order <T> {
    pub fn builder(symbol: String, side: Side, order_params: T) -> Self {
        Self {
            symbol,
            side,
            order_type: T::get_type(),
            price: None,
            broker_id: None,
            client_order_id: None,
            order_tag: None,
            order_quantity: 0,
            order_amount: None,
            reduce_only: None,
            visible_quantity: None,
            order_params
        }
    }

    pub fn with_broker_id(mut self, broker_id: String) -> Order<T> {
        self.broker_id = Some(broker_id);
        self
    }

    pub fn with_client_order_id(mut self, client_order_id: i32) -> Order<T> {
        self.client_order_id = Some(client_order_id);
        self
    }

    pub fn with_order_tag(mut self, order_tag: String) -> Order<T> {
        self.order_tag = Some(order_tag);
        self
    }

    pub fn with_order_quantity(mut self, order_quantity: i32) -> Order<T> {
        self.order_quantity = order_quantity;
        self
    }

    pub fn with_order_amount(mut self, order_amount: i32) -> Order<T> {
        self.order_amount = Some(order_amount);
        self
    }

    pub fn with_reduce_only(mut self, reduce_only: bool) -> Order<T> {
        self.reduce_only = Some(reduce_only);
        self
    }

    pub fn with_visible_quantity(mut self, visible_quantity: i32) -> Order<T> {
        self.visible_quantity = Some(visible_quantity);
        self
    }
}

pub trait Buildable {
    fn build(self) -> Self;
}

impl Buildable for Order<Limit> {
    fn build(mut self) -> Order<Limit> {
        self.price = Some(self.order_params.price);
        self.order_quantity = self.order_params.order_quantity;
        self
    }
}

impl Buildable for Order<Market> {
    fn build(mut self) -> Order<Market> {
        self.order_quantity = self.order_params.order_quantity;
        self
    }
}

impl Buildable for Order<IOC> {
    fn build(mut self) -> Order<IOC> {
        self.price = Some(self.order_params.price);
        self.order_quantity = self.order_params.order_quantity;
        self
    }
}

impl Buildable for Order<FOK> {
    fn build(mut self) -> Order<FOK> {
        self.price = Some(self.order_params.price);
        self.order_quantity = self.order_params.order_quantity;
        self
    }
}

impl Buildable for Order<PostOnly> {
    fn build(mut self) -> Order<PostOnly> {
        self.price = Some(self.order_params.price);
        self.order_quantity = self.order_params.order_quantity;
        self
    }
}

impl Queryable for Order<Limit> {
    fn to_query_string(&self) -> String {

        let mut query = String::new();

        if self.broker_id.is_some() { query.push_str(&*format!("broker_id={}&", &self.broker_id.as_ref().unwrap())) }
        if self.client_order_id.is_some() { query.push_str(&*format!("client_order_id={}&", &self.client_order_id.as_ref().unwrap())) }
        query.push_str(&*format!("order_price={}&", &self.price.unwrap()));
        if self.order_amount.is_some() { query.push_str(&*format!("order_amount={}&", &self.order_amount.as_ref().unwrap())) }
        query.push_str(&*format!("order_quantity={}&", &self.order_quantity));
        if self.order_tag.is_some() { query.push_str(&*format!("order_tag={}&", &self.order_tag.as_ref().unwrap())) }
        query.push_str(&*format!("order_type={}&", &self.order_type));
        if self.reduce_only.is_some() { query.push_str(&*format!("reduce_only={}&", &self.reduce_only.as_ref().unwrap())) }
        query.push_str(&*format!("side={}&", &self.side));
        query.push_str(&*format!("symbol={}&", &self.symbol));
        if self.visible_quantity.is_some() { query.push_str(&*format!("visible_quantity={}&", &self.visible_quantity.unwrap())) }

        query.pop();

        query
    }
}

impl Queryable for Order<Market> {
    fn to_query_string(&self) -> String {

        let mut query = String::new();

        if self.broker_id.is_some() { query.push_str(&*format!("broker_id={}&", &self.broker_id.as_ref().unwrap())) }
        if self.client_order_id.is_some() { query.push_str(&*format!("client_order_id={}&", &self.client_order_id.as_ref().unwrap())) }
        if self.order_amount.is_some() { query.push_str(&*format!("order_amount={}&", &self.order_amount.as_ref().unwrap())) }
        query.push_str(&*format!("order_quantity={}&", &self.order_quantity));
        if self.order_tag.is_some() { query.push_str(&*format!("order_tag={}&", &self.order_tag.as_ref().unwrap())) }
        query.push_str(&*format!("order_type={}&", &self.order_type));
        if self.reduce_only.is_some() { query.push_str(&*format!("reduce_only={}&", &self.reduce_only.as_ref().unwrap())) }
        query.push_str(&*format!("side={}&", &self.side));
        query.push_str(&*format!("symbol={}&", &self.symbol));
        if self.visible_quantity.is_some() { query.push_str(&*format!("visible_quantity={}&", &self.visible_quantity.unwrap())) }

        query.pop();

        query
    }
}

impl Queryable for Order<IOC> {
    fn to_query_string(&self) -> String {

        let mut query = String::new();

        if self.broker_id.is_some() { query.push_str(&*format!("broker_id={}&", &self.broker_id.as_ref().unwrap())) }
        if self.client_order_id.is_some() { query.push_str(&*format!("client_order_id={}&", &self.client_order_id.as_ref().unwrap())) }
        query.push_str(&*format!("order_price={}&", &self.price.unwrap()));
        if self.order_amount.is_some() { query.push_str(&*format!("order_amount={}&", &self.order_amount.as_ref().unwrap())) }
        query.push_str(&*format!("order_quantity={}&", &self.order_quantity));
        if self.order_tag.is_some() { query.push_str(&*format!("order_tag={}&", &self.order_tag.as_ref().unwrap())) }
        query.push_str(&*format!("order_type={}&", &self.order_type));
        if self.reduce_only.is_some() { query.push_str(&*format!("reduce_only={}&", &self.reduce_only.as_ref().unwrap())) }
        query.push_str(&*format!("side={}&", &self.side));
        query.push_str(&*format!("symbol={}&", &self.symbol));
        if self.visible_quantity.is_some() { query.push_str(&*format!("visible_quantity={}&", &self.visible_quantity.unwrap())) }

        query.pop();

        query
    }
}

impl Queryable for Order<FOK> {
    fn to_query_string(&self) -> String {

        let mut query = String::new();

        if self.broker_id.is_some() { query.push_str(&*format!("broker_id={}&", &self.broker_id.as_ref().unwrap())) }
        if self.client_order_id.is_some() { query.push_str(&*format!("client_order_id={}&", &self.client_order_id.as_ref().unwrap())) }
        query.push_str(&*format!("order_price={}&", &self.price.unwrap()));
        if self.order_amount.is_some() { query.push_str(&*format!("order_amount={}&", &self.order_amount.as_ref().unwrap())) }
        query.push_str(&*format!("order_quantity={}&", &self.order_quantity));
        if self.order_tag.is_some() { query.push_str(&*format!("order_tag={}&", &self.order_tag.as_ref().unwrap())) }
        query.push_str(&*format!("order_type={}&", &self.order_type));
        if self.reduce_only.is_some() { query.push_str(&*format!("reduce_only={}&", &self.reduce_only.as_ref().unwrap())) }
        query.push_str(&*format!("side={}&", &self.side));
        query.push_str(&*format!("symbol={}&", &self.symbol));
        if self.visible_quantity.is_some() { query.push_str(&*format!("visible_quantity={}&", &self.visible_quantity.unwrap())) }

        query.pop();

        query
    }
}

impl Queryable for Order<PostOnly> {
    fn to_query_string(&self) -> String {

        let mut query = String::new();

        if self.broker_id.is_some() { query.push_str(&*format!("broker_id={}&", &self.broker_id.as_ref().unwrap())) }
        if self.client_order_id.is_some() { query.push_str(&*format!("client_order_id={}&", &self.client_order_id.as_ref().unwrap())) }
        query.push_str(&*format!("order_price={}&", &self.price.unwrap()));
        if self.order_amount.is_some() { query.push_str(&*format!("order_amount={}&", &self.order_amount.as_ref().unwrap())) }
        query.push_str(&*format!("order_quantity={}&", &self.order_quantity));
        if self.order_tag.is_some() { query.push_str(&*format!("order_tag={}&", &self.order_tag.as_ref().unwrap())) }
        query.push_str(&*format!("order_type={}&", &self.order_type));
        if self.reduce_only.is_some() { query.push_str(&*format!("reduce_only={}&", &self.reduce_only.as_ref().unwrap())) }
        query.push_str(&*format!("side={}&", &self.side));
        query.push_str(&*format!("symbol={}&", &self.symbol));
        if self.visible_quantity.is_some() { query.push_str(&*format!("visible_quantity={}&", &self.visible_quantity.unwrap())) }

        query.pop();

        query
    }
}

impl Queryable for Order<Ask> {
    fn to_query_string(&self) -> String {

        let mut query = String::new();

        if self.broker_id.is_some() { query.push_str(&*format!("broker_id={}&", &self.broker_id.as_ref().unwrap())) }
        if self.client_order_id.is_some() { query.push_str(&*format!("client_order_id={}&", &self.client_order_id.as_ref().unwrap())) }
        query.push_str(&*format!("order_price={}&", &self.price.unwrap()));
        if self.order_amount.is_some() { query.push_str(&*format!("order_amount={}&", &self.order_amount.as_ref().unwrap())) }
        query.push_str(&*format!("order_quantity={}&", &self.order_quantity));
        if self.order_tag.is_some() { query.push_str(&*format!("order_tag={}&", &self.order_tag.as_ref().unwrap())) }
        query.push_str(&*format!("order_type={}&", &self.order_type));
        if self.reduce_only.is_some() { query.push_str(&*format!("reduce_only={}&", &self.reduce_only.as_ref().unwrap())) }
        query.push_str(&*format!("side={}&", &self.side));
        query.push_str(&*format!("symbol={}&", &self.symbol));
        if self.visible_quantity.is_some() { query.push_str(&*format!("visible_quantity={}&", &self.visible_quantity.unwrap())) }

        query.pop();

        query
    }
}

impl Queryable for Order<Bid> {
    fn to_query_string(&self) -> String {

        let mut query = String::new();

        if self.broker_id.is_some() { query.push_str(&*format!("broker_id={}&", &self.broker_id.as_ref().unwrap())) }
        if self.client_order_id.is_some() { query.push_str(&*format!("client_order_id={}&", &self.client_order_id.as_ref().unwrap())) }
        query.push_str(&*format!("order_price={}&", &self.price.unwrap()));
        if self.order_amount.is_some() { query.push_str(&*format!("order_amount={}&", &self.order_amount.as_ref().unwrap())) }
        query.push_str(&*format!("order_quantity={}&", &self.order_quantity));
        if self.order_tag.is_some() { query.push_str(&*format!("order_tag={}&", &self.order_tag.as_ref().unwrap())) }
        query.push_str(&*format!("order_type={}&", &self.order_type));
        if self.reduce_only.is_some() { query.push_str(&*format!("reduce_only={}&", &self.reduce_only.as_ref().unwrap())) }
        query.push_str(&*format!("side={}&", &self.side));
        query.push_str(&*format!("symbol={}&", &self.symbol));
        if self.visible_quantity.is_some() { query.push_str(&*format!("visible_quantity={}&", &self.visible_quantity.unwrap())) }

        query.pop();

        query
    }
}

impl<T: Params> Display for Order<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", &self.symbol, &self.side, T::get_type())
    }
}

#[test]
fn bruh_test() {
    let _limit_params: Limit = Limit::new(10.1, 10);

    let _market_params: Market = Market::new(10);

    let ioc_params: IOC = IOC::new(10.1, 10);

    let order: Order<IOC> = Order::builder("EBJRAB".to_string(), Side::Buy, ioc_params).build();

    println!("{}", order.to_query_string());
}