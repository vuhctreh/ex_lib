use crate::woo::enums::Side;

pub struct Order<T: OrderTypes> {
    symbol: String,
    side: Side,
    order_type: T
}

// TODO: MAKE ENUMS FOR EVERYTHING AND FINISH ALL THE IMPLS
impl <T: OrderTypes> Order<T> {
    pub(crate) fn new (symbol: String, side: Side, order_type: T) -> Self {
        Order {
            symbol,
            side,
            order_type
        }
    }
}

pub trait OrderTypes {}

pub trait Priceable {
    fn new(_: f64) -> Self;
}

pub trait NonPriceable {
    fn new() -> Self;
}

pub struct Limit {
    price: f64,
}

impl Priceable for Limit {
    fn new(price: f64) -> Self {
        Limit {
            price
        }
    }
}

impl OrderTypes for Limit {}

pub struct Market {}

impl OrderTypes for Market {}

impl NonPriceable for Market {
    fn new() -> Self {
        Market {}
    }
}

pub struct IOC {
    price: f64,
}

impl OrderTypes for IOC {}

impl Priceable for IOC {
    fn new(price: f64) -> Self {
        IOC {
            price
        }
    }
}

pub struct FOK {
    pub price: f64,
}

impl OrderTypes for FOK {}

impl Priceable for FOK {
    fn new(price: f64) -> Self {
        FOK {
            price
        }
    }
}

pub struct PostOnly {
    pub price: f64,
}

impl OrderTypes for PostOnly {}

impl Priceable for PostOnly {
    fn new(price: f64) -> Self {
        PostOnly {
            price
        }
    }
}

pub struct Ask {
    pub price: f64,
}

impl OrderTypes for Ask {}

impl Priceable for Ask {
    fn new(price: f64) -> Self {
        Ask {
            price
        }
    }
}

pub struct Bid {
    pub price: f64,
}

impl OrderTypes for Bid {}

impl Priceable for Bid {
    fn new(price: f64) -> Self {
        Bid {
            price
        }
    }
}

#[test]
fn test_order_new() {
    let order_type = Limit::new(10.0);

    let order = Order::new("BTCUSD".to_string(), Side::Buy, order_type);

    println!("{:?}", order.order_type.price);
}