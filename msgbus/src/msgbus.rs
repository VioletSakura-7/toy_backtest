use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::Arc,
};

use common::Event;
use tokio::sync::{
    broadcast,
    broadcast::{
        Receiver,
        Sender,
    },
    Mutex,
};
use ustr::Ustr;
pub struct MsgBus
{
    senders: RefCell<HashMap<Ustr, Sender<Event>>>,
    receivers: RefCell<HashMap<Ustr, Arc<Mutex<Receiver<Event>>>>>,
}

impl MsgBus
{
    pub fn new() -> Self
    {
        MsgBus {
            senders: RefCell::new(HashMap::new()),
            receivers: RefCell::new(HashMap::new()),
        }
    }
    //todo 错误处理问题没解决
    pub fn registe_sender(&self, topic: Ustr)
    {
        let (tx, _rx) = broadcast::channel(16);

        // println!("{:?}=============", topic);
        self.senders.borrow_mut().insert(topic, tx);
        //todo 日志打印
    }

    pub fn subscribe_receiver(&self, topic_from: Ustr, topic: Ustr)
    {
        if !self.senders.borrow().contains_key(&topic_from) {
            println!("From not found");
        } else {
            let ss = self.senders.borrow();
            let sender = ss.get(&topic_from).unwrap();
            let rx = sender.subscribe();
            let insert_topic = Ustr::from(format!("{topic}_{topic_from}").as_str());
            let rx_mutex = Arc::new(Mutex::new(rx));
            self.receivers.borrow_mut().insert(insert_topic, rx_mutex);
            // println!("{:?}=====receiver========", topic_from);
            //todo 日志打印
        }
    }

    pub fn get_sender(&self, topic: Ustr) -> Option<Sender<Event>>
    {
        self.senders.borrow().get(&topic).cloned()
    }

    pub fn get_receiver(&self, topic: Ustr) -> Option<Arc<Mutex<Receiver<Event>>>>
    {
        self.receivers.borrow().get(&topic).cloned()
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_msgbus()
    {
        let mut msgbus = MsgBus::new();

        assert!(msgbus.get_sender("test".into()).is_none());
        assert!(msgbus.get_receiver("test".into()).is_none());

        let test1 = Ustr::from("test1");
        let test2 = Ustr::from("test2");

        msgbus.registe_sender(test1.clone());
        assert_eq!(msgbus.get_sender(test1.clone()).is_some(), true);

        msgbus.subscribe_receiver(test1.clone(), test2.clone());
        assert_eq!(msgbus.get_receiver(test2.clone()).is_some(), true);
    }
}
