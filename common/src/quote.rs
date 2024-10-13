struct Quote
{
    bids: Vec<f32>,
    asks: Vec<f32>,
    bid_size: u32,
    ask_size: u32,
}

pub struct Snapshot
{
    price: f32,
    volume: u32,
    timestamp: u64,
}

impl Quote {}
