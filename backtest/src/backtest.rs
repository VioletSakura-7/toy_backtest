use std::{
    collections::BTreeMap,
    rc::Rc,
    sync::Arc,
    time::Duration, vec,
};

use common::{
    order::Order,
    quote::Snapshot,
    tick::Tick,
    time::AtomicTime,
    Component,
    Event,
};
use msgbus::MsgBus;
use rand::Rng;
use tokio::{
    sync::{
        broadcast::{
            Receiver,
            Sender,
        },
        Mutex,
    },
    time::interval,
};
use ustr::Ustr;

////////////////////////////////////////////////////////////////////////////////
// Backtest
////////////////////////////////////////////////////////////////////////////////
pub struct Backtest
{
    //组件基本属性
    id: Ustr,
    msgbus: Rc<MsgBus>,
    sender: Option<Sender<Event>>,
    receivers: Option<Vec<Arc<Mutex<Receiver<Event>>>>>,

    //回测功能属性
    exchanges: Vec<OrderMatchMachineCore>,
    order_simulater: OrderSimulater,
    clock: Arc<AtomicTime>,
}

impl Backtest
{
    pub fn new(id: Ustr, msgbus: Rc<MsgBus>) -> Self
    {
        Backtest {
            id,
            msgbus,
            sender: None,
            receivers: None,
            exchanges: vec![OrderMatchMachineCore::new(),OrderMatchMachineCore::new(),OrderMatchMachineCore::new()],
            order_simulater: OrderSimulater::new(),
            clock: AtomicTime::new(),
        }
    }

    pub fn start(&mut self) {
        //todo
    }

    pub fn stop(&mut self) {
        //todo
    }

    pub fn add_exchange(&mut self ) {
        //todo
    }

    pub fn remove_exchange(&mut self) {
        //todo
    }

    pub fn handle_event(&mut self, _event: Event) {
        //todo
    }

    pub fn publish_event(&mut self, _event: Event) {
        //todo
    }

    pub fn process_order(&mut self, _order: Order) {
        //todo
    }
    


}

impl Component for Backtest
{
    fn get_id(&self) -> Ustr
    {
        self.id
    }
    //todo 错误处理问题没解决
    fn registe_sender(&self)
    {
        self.msgbus.registe_sender(self.id);
    }

    fn subscribe_receivers(&self, topic_from: Vec<Ustr>)
    {
        for i in topic_from {
            self.msgbus.subscribe_receiver(i, self.id);
        }
    }

    fn fetch_sender(&mut self)
    {
        self.sender = self.msgbus.get_sender(self.id);
    }

    fn fetch_receivers(&mut self, topics: Vec<Ustr>)
    {
        let id = self.id;
        let topics = topics
            .iter()
            .map(|i| Ustr::from(format!("{id}_{i}").as_str()));
        if let Some(receivers) = &mut self.receivers {
            for i in topics {
                println!("{}", i);
                if let Some(receiver) = self.msgbus.get_receiver(i) {
                    receivers.push(receiver);
                } else {
                    println!("fail to fetch");
                    continue;
                }
            }
        } else {
            let mut receivers = Vec::new();
            for i in topics {
                println!("{}", i);
                if let Some(receiver) = self.msgbus.get_receiver(i) {
                    receivers.push(receiver);
                } else {
                    println!("fail to fetch");
                    continue;
                }
            }
            self.receivers = Some(receivers);
        }
    }

    fn start_publish(&self, events: Vec<Event>)
    {
        let sender = self.sender.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(10));
            if let Some(sender) = sender {
                for event in events {
                    interval.tick().await;
                    println!("send event : {}", event);
                    let _ = sender.send(event);
                }
            }
        });
    }

    fn start_receive(&self)
    {
        // 调整间隔时间as needed
        let receivers_self = self.receivers.clone();
        if let Some(receivers) = receivers_self {
            for receiver in receivers {
                println!("flag1");
                tokio::spawn(async move {
                    let mut interval = interval(Duration::from_millis(100));
                    println!("flag3");
                    loop {
                        interval.tick().await;
                        let result = {
                            let mut rx = receiver.lock().await;
                            rx.try_recv()
                        }; // 锁在这里被释放
                        match result {
                            Ok(event) => println!("Received event: {}", event),
                            Err(tokio::sync::broadcast::error::TryRecvError::Empty) => {
                                println!("Channel empty");
                            }
                            Err(tokio::sync::broadcast::error::TryRecvError::Closed) => {
                                println!("Channel closed");
                                return;
                            }
                            Err(tokio::sync::broadcast::error::TryRecvError::Lagged(n)) => {
                                println!("Lagged behind by {} messages", n);
                            }
                            _ => {
                                println!("flag4");
                            }
                        }
                    }
                });
                println!("flag2");
            }
        } else {
            println!("No receivers");
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// OrderMatchMachineCore
////////////////////////////////////////////////////////////////////////////////
type PriceLevel = f32;
struct OrderMatchMachineCore
{
    // 订单匹配机核心
    bids: BTreeMap<PriceLevel, Vec<Order>>,
    asks: BTreeMap<PriceLevel, Vec<Order>>,
    marketprice_cache: Vec<Snapshot>,
    tick_cache: Vec<Tick>,
    clock: Arc<AtomicTime>,
}
impl OrderMatchMachineCore
{
    fn new() -> Self
    {
        //todo
        OrderMatchMachineCore {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            marketprice_cache: Vec::new(),
            tick_cache: Vec::new(),
            clock: AtomicTime::new(),
        }
    }
    async fn process_order(&mut self, order: Order) -> Result<(), String>
    {
        //todo
        Ok(())
    }

    async fn match_order(&mut self, order: Order) -> Result<(), String>
    {
        //todo
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
// OrderSimulater
////////////////////////////////////////////////////////////////////////////////
struct OrderSimulater
{
    random: rand::prelude::ThreadRng,
    // 订单模拟器
    clock: Arc<AtomicTime>,
}

impl OrderSimulater
{
    fn new() -> Self
    {
        OrderSimulater {
            clock: AtomicTime::new(),
            random: rand::thread_rng(),
        }
    }

    async fn create_order(&mut self) -> Order
    {
        
        let rand_num = self.random.gen_range(0..=1);
        if rand_num == 0 {
            Order::new_market_order()
        } else {
            Order::new_limit_order()
        }
    }
}
