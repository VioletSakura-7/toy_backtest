#[derive(Clone, Debug)]
pub struct MarketOrder {}
#[derive(Clone, Debug)]
pub struct LimitOrder {}

pub enum Order
{
    MarketOrder(MarketOrder),
    LimitOrder(LimitOrder),
}

impl Order {
    pub fn new_market_order() -> Self {
        Order::MarketOrder(MarketOrder {})
    }

    pub fn new_limit_order() -> Self {
        Order::LimitOrder(LimitOrder {})
    }
}
