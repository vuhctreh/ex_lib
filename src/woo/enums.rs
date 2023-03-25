use std::fmt::{Display, Formatter};

pub enum Timeframe {
    _1m,
    _5m,
    _15m,
    _30m,
    _1h,
    _4h,
    _12h,
    _1d,
    _2w,
    _1mon,
    _1y
}

impl Display for Timeframe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Timeframe::_1m => write!(f, "1m"),
            Timeframe::_5m => write!(f, "5m"),
            Timeframe::_15m => write!(f, "15m"),
            Timeframe::_30m => write!(f, "30m"),
            Timeframe::_1h => write!(f, "1h"),
            Timeframe::_4h => write!(f, "4h"),
            Timeframe::_12h => write!(f, "12h"),
            Timeframe::_1d => write!(f, "1d"),
            Timeframe::_2w => write!(f, "2w"),
            Timeframe::_1mon => write!(f, "1mon"),
            Timeframe::_1y => write!(f, "1y")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Side {
    Sell,
    Buy
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Sell => write!(f, "{}", "SELL"),
            Side::Buy => write!(f, "{}", "BUY")
        }
    }
}

// TODO symbol enums