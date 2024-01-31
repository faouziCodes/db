use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver, Sender},
};

pub struct PubSub<Msg> {
    channels: HashMap<String, Channel<Msg>>,
}

struct Channel<Msg> {
    channel: String,
    subcribers: Vec<Subscription<Msg>>,
}

struct Subscription<Msg> {
    sender: Sender<Msg>,
}

pub struct Subscriber<Msg> {
    pub sender: Sender<Msg>,
    pub reciever: Receiver<Msg>,
}

impl<Msg: Clone> PubSub<Msg> {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }

    pub fn new_subscriber(&mut self) -> Subscriber<Msg> {
        let (sender, reciever) = mpsc::channel();
        Subscriber { sender, reciever }
    }

    pub fn new_channel(&mut self, channel: &str) {
        if self.channels.contains_key(channel) {
            return;
        }

        self.channels.insert(
            channel.into(),
            Channel {
                channel: channel.into(),
                subcribers: Vec::new(),
            },
        );
    }

    pub fn get_channel(&mut self, channel: &str) -> Result<&mut Channel<Msg>, String> {
        if !self.channels.contains_key(channel) {
            return Err("Channel does not exist.".into());
        }

        Ok(self.channels.get_mut(channel).unwrap())
    }

    pub fn subscribe(&mut self, channel: &str) -> Result<Subscriber<Msg>, String> {
        let channel = self.get_channel(channel)?;
        Ok(channel.subscribe())
    }

    pub fn subsribe_with_sender(
        &mut self,
        channel: &str,
        sender: Sender<Msg>,
    ) -> Result<(), String> {
        let channel = self.get_channel(channel)?;

        channel.subcribers.push(Subscription {
            sender: sender.clone(),
        });

        Ok(())
    }

    pub fn publish(&mut self, channel: &str, msg: Msg) -> Result<(), String> {
        let channel = self.get_channel(channel)?;

        for subcriber in &channel.subcribers {
            // Todo: Handle the case where the reciever is no longer listening.
            subcriber.sender.send(msg.clone());
        }

        Ok(())
    }
}

impl<Msg> Channel<Msg> {
    fn subscribe(&mut self) -> Subscriber<Msg> {
        let (sender, reciever) = mpsc::channel();

        self.subcribers.push(Subscription {
            sender: sender.clone(),
        });

        Subscriber { sender, reciever }
    }
}

impl<Msg> Subscriber<Msg> {
    fn new(sender: Sender<Msg>, reciever: Receiver<Msg>) -> Self {
        Self { sender, reciever }
    }

    pub fn recieve(&self) -> Result<Msg, String> {
        self.reciever
            .recv()
            .map_err(|_| "Failed to recieve message.".into())
    }
}

impl<Msg> Default for Subscriber<Msg> {
    fn default() -> Self {
        let (sender, reciever) = mpsc::channel();
        Self { sender, reciever }
    }
}
