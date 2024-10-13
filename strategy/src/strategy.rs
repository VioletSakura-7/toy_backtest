use std::{
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use common::{
    Component,
    Event,
};
use msgbus::MsgBus;
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
use crate::excutor::Executor;
use crate::order_factory::OrderFactory;
use crate::single_generator::SingleGenerator;
use crate::strategyconfig::StrategyConfig;

struct Strategy {
    //组件属性
    id: Ustr,
    msgbus: Rc<MsgBus>,
    sender: Option<Sender<Event>>,
    receivers: Option<Vec<Arc<Mutex<Receiver<Event>>>>>,

    //功能属性
    excutor: Option<Executor>,
    single_generator: Option<Box<dyn SingleGenerator>>,
    order_factory: Option<OrderFactory>,

    //配置属性
    name : Option<Ustr>,
    //exchange_trading_pair  {Binance_BTC/USDT}
    portfolio_info : Option<Vec<Ustr>>,
    config: StrategyConfig,
}



impl Strategy {
    pub fn builder (id: Ustr, msgbus: Rc<MsgBus>) -> Self {
        Self {
            id,
            msgbus,
            sender: None,
            receivers: None,

            excutor: None,
            single_generator: None,
            order_factory: None,

            name: None,
            portfolio_info: None,
            config: StrategyConfig::default(),
        }
    }

    fn set_config(mut self, config: StrategyConfig) -> Self {}

    fn set_excutor(mut self, excutor: Executor) -> Self {}

    fn set_single_generator(mut self, single_generator: Box<dyn SingleGenerator>) -> Self {}

    fn set_order_factory(mut self, order_factory: OrderFactory) -> Self {}

    pub fn build(self) -> Self {}



}

impl Component for Strategy
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
            topics.for_each(|i| {
           
                    println!("{}", i);
                    if let Some(receiver) = self.msgbus.get_receiver(i) {
                        receivers.push(receiver);
                    } else {
                        println!("fail to fetch");
                        
                    }
                
            })

        } else {
            let mut receivers = Vec::new();
            topics.for_each(|i| {
                println!("{}", i);
                if let Some(receiver) = self.msgbus.get_receiver(i) {
                    receivers.push(receiver);
                } else {
                    println!("fail to fetch");
                }
            });
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