use std::fmt::{Display, Formatter};
use crate::woo::enums::Side;

trait Params {
    fn get_type() -> String;
}

trait Queryable {} // this trait is to be implemented for each Order generic type.

struct Limit {
    pub price: f64,
    pub broker_id: Option<String>,
    pub client_order_id: Option<i32>,
    pub order_tag: Option<String>,
    pub order_quantity: Option<i32>,
    pub order_amount: Option<i32>,
    pub reduce_only: Option<bool>,
    pub visible_quantity: Option<i32>,
}

impl Limit {
    pub fn builder(price: f64) -> Self {
        Self { price,
            broker_id: None,
            client_order_id: None,
            order_tag: None,
            order_quantity: None,
            order_amount: None,
            reduce_only: None,
            visible_quantity: None
        }
    }

    pub fn set_broker_id(mut self, broker_id: String) -> Limit {
        self.broker_id = Some(broker_id);
        self
    }

    pub fn client_order_id(mut self, client_order_id: String) -> Limit {
        self.broker_id = Some(client_order_id);
        self
    }

    pub fn set_order_tag(mut self, order_tag: String) -> Limit {
        self.order_tag = Some(order_tag);
        self
    }

    pub fn set_order_quantity(mut self, order_quantity: i32) -> Limit {
        self.order_quantity = Some(order_quantity);
        self
    }

    pub fn set_order_amount(mut self, order_amount: i32) -> Limit {
        self.order_amount = Some(order_amount);
        self
    }

    pub fn set_reduce_only(mut self, reduce_only: bool) -> Limit {
        self.reduce_only = Some(reduce_only);
        self
    }

    pub fn set_visible_quantity(mut self, visible_quantity: i32) -> Limit {
        self.visible_quantity = Some(visible_quantity);
        self
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

pub(crate) struct Order <T: Params> {
    symbol: String,
    side: Side,
    order_type: String,
    order_params: T
}

impl <T: Params> Order <T> {
    pub fn new(symbol: String, side: Side, order_params: T) -> Self {
        Self {
            symbol,
            side,
            order_type: T::get_type(),
            order_params
        }
    }
}

impl<T: Params> Display for Order<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", &self.symbol, &self.side, T::get_type())
    }
}

#[test]
fn bruh_test() {
    let limit_params: Limit = Limit::builder(10.0);

    let order: Order<Limit> = Order::new("BTCUSDT".to_string(), Side::Buy, limit_params);

    println!("{}", order);
}