use ustr::Ustr;

use crate::event::Event;

pub trait Component
{
    fn get_id(&self) -> Ustr;
    fn registe_sender(&self);
    fn subscribe_receivers(&self, topic_froms: Vec<Ustr>);
    fn fetch_sender(&mut self);
    fn fetch_receivers(&mut self, topics: Vec<Ustr>);
    fn start_publish(&self, events: Vec<Event>);
    fn start_receive(&self);
}
