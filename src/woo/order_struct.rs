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

// TO GET IT TO DO WHAT I WANT I PROBABLY NEED TO CREATE AND COMBINE MANY TRAITS

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

pub struct IOC {}

impl OrderTypes for IOC {}

pub struct FOK {}

impl OrderTypes for FOK {}

pub struct PostOnly {}

impl OrderTypes for PostOnly {}

pub struct Bid {}

impl OrderTypes for Bid {}

#[test]
fn test_order_new() {
    let order_type = Limit::new(10.0);

    let order = Order::new("BTCUSD".to_string(), Side::Buy, order_type);

    println!("{:?}", order.order_type.price);
}