use std::fmt::{
    Display,
    Formatter,
    Result,
};

use crate::order::{
    LimitOrder,
    MarketOrder,
};
#[derive(Clone, Debug)]
pub enum Event
{
    BacktestEvent(BacktestEvent),
    DataClientEvent(DataClientEvent),
    ExecutorEvent(ExecutorEvent),
}

impl Event
{
    //todo 需要修改
    pub fn new() -> Self
    {
        Event::BacktestEvent(BacktestEvent::BacktestStart)
    }
}

impl Display for Event
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        match self {
            Event::BacktestEvent(event) => write!(f, "Backtest Event:"),
            Event::DataClientEvent(event) => write!(f, "Data Client Event:"),
            Event::ExecutorEvent(event) => write!(f, "Executor Event:"),
        }
    }
}
#[derive(Clone, Debug)]
enum BacktestEvent
{
    //status
    BacktestStart,
    BacktestStop,
}
#[derive(Clone, Debug)]
enum DataClientEvent
{
    DataClientStart,
    DataClientStop,

    //connect
    GetDataStreamConn,
    DropDataStreamConn,

    //data
    StartDataStream,
    StopDataStream,
}
#[derive(Clone, Debug)]
enum ExecutorEvent
{
    ExecutorStart,
    ExecutorStop,

    //order
    ProcessMarketOrder(MarketOrder),
    ProcessLimitOrder(LimitOrder),
}
