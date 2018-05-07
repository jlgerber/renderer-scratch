
use traits::*;

pub type Job = Box<FnBox + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}