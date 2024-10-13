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
struct DataClient
{
    id: Ustr,
    //todo  cache
    //cache : Ustr,
    msgbus: Rc<MsgBus>,
    sender: Option<Sender<Event>>,
    receivers: Option<Vec<Arc<Mutex<Receiver<Event>>>>>,
}

impl DataClient
{
    pub fn new(id: Ustr, msgbus: Rc<MsgBus>) -> Self
    {
        DataClient {
            id,
            msgbus,
            sender: None,
            receivers: None,
        }
    }
}

impl Component for DataClient
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

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_dataclient()
    {
        let msgbus = Rc::new(MsgBus::new());

        let mut dataclient1 = DataClient::new(Ustr::from("test1"), msgbus.clone());
        let mut dataclient2 = DataClient::new(Ustr::from("test2"), msgbus.clone());
        let mut dataclient3 = DataClient::new(Ustr::from("test3"), msgbus.clone());

        assert_eq!(dataclient1.sender.is_some(), false);
        assert_eq!(dataclient2.sender.is_some(), false);
        assert_eq!(dataclient1.receivers.is_some(), false);
        assert_eq!(dataclient2.receivers.is_some(), false);

        //注册发送端 订阅接收端
        dataclient1.registe_sender();
        dataclient2.registe_sender();
        dataclient3.registe_sender();
        dataclient1.subscribe_receivers(vec![Ustr::from("test2"), Ustr::from("test3")]);
        dataclient2.subscribe_receivers(vec![Ustr::from("test1"), Ustr::from("test3")]);
        dataclient3.subscribe_receivers(vec![Ustr::from("test1"), Ustr::from("test2")]);

        dataclient1.fetch_sender();
        dataclient2.fetch_sender();
        dataclient1.fetch_receivers(vec![Ustr::from("test2")]);
        dataclient2.fetch_receivers(vec![Ustr::from("test1")]);

        assert_eq!(dataclient1.sender.is_some(), true);
        assert_eq!(dataclient2.sender.is_some(), true);
        assert_eq!(dataclient1.receivers.is_some(), true);
        assert_eq!(dataclient2.receivers.is_some(), true);
    }
    #[tokio::test]
    async fn test_publish()
    {
        let msgbus = Rc::new(MsgBus::new());
        let mut dataclient1 = DataClient::new(Ustr::from("test1"), msgbus.clone());
        dataclient1.registe_sender();
        dataclient1.fetch_sender();
        let event = Event::new();
        let events = vec![event];
        dataclient1.start_publish(events);

        // 给一些时间让事件发送
        tokio::time::sleep(Duration::from_secs(6)).await;
    }
    #[tokio::test]
    async fn test_receive()
    {
        let msgbus = Rc::new(MsgBus::new());

        let mut dataclient1 = DataClient::new(Ustr::from("test1"), msgbus.clone());
        dataclient1.registe_sender();
        dataclient1.fetch_sender();

        let mut dataclient2 = DataClient::new(Ustr::from("test2"), msgbus.clone());
        dataclient2.registe_sender();
        dataclient2.fetch_sender();

        let mut dataclient3 = DataClient::new(Ustr::from("test3"), msgbus.clone());
        dataclient3.subscribe_receivers(vec![Ustr::from("test1"), Ustr::from("test2")]);
        dataclient3.fetch_receivers(vec![Ustr::from("test1"), Ustr::from("test2")]);

        let event1 = Event::new();
        let event2 = Event::new();
        let event3 = Event::new();
        let event4 = Event::new();
        let event5 = Event::new();
        let event6 = Event::new();
        let event7 = Event::new();
        let event8 = Event::new();
        let event9 = Event::new();
        let event10 = Event::new();
        let events1 = vec![
            event1.clone(),
            event2.clone(),
            event3.clone(),
            event4.clone(),
            event5.clone(),
            event6.clone(),
            event7.clone(),
            event8.clone(),
            event9.clone(),
            event10.clone(),
        ];
        let events2 = vec![event1, event2, event3, event4, event5, event6, event7, event8, event9, event10];

        dataclient1.start_publish(events1);
        dataclient2.start_publish(events2);

        dataclient3.start_receive();

        // assert_eq!(dataclient2.received_events.len(), 3);
        // assert_eq!(dataclient2.received_events[0], event1);
        // assert_eq!(dataclient2.received_events[1], event2);
        // assert_eq!(dataclient2.received_events[2], event3);
    }
}
