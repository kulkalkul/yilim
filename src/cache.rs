use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use lru::LruCache;
use serenity::model::id::MessageId;
use serenity::model::prelude::ChannelId;

pub struct CooldownsCache {
    cache: Mutex<LruCache<u64, Instant>>,
}
impl CooldownsCache {
    fn new() -> Self {
        Self { cache: Mutex::new(LruCache::new(1024)) }
    }
    pub fn put(&self, user_id: u64, time: Instant) -> Option<Instant> {
        self.cache
            .lock()
            .unwrap()
            .put(user_id, time)
    }
    pub fn since(&self, user_id: u64) -> Option<Duration> {
        match self.cache
            .lock()
            .unwrap()
            .peek(&user_id)
        {
            Some(prev) => Some(Instant::now().duration_since(*prev)),
            None => None,
        }
    }
}

pub struct SelectedChannelsCache {
    cache: Mutex<LruCache<u64, u64>>,
}

impl SelectedChannelsCache {
    fn new() -> Self {
        Self { cache: Mutex::new(LruCache::new(256)) }
    }
    pub fn put(&self, user_id: u64, channel_id: u64) -> Option<u64> {
        self.cache
            .lock()
            .unwrap()
            .put(user_id, channel_id)
    }
    pub fn get(&self, user_id: u64) -> Option<ChannelId> {
        match self.cache
            .lock()
            .unwrap()
            .peek(&user_id)
        {
            Some(channel_id) => Some(ChannelId(*channel_id)),
            None => None,
        }
    }
}

pub struct UserMessage {
    user_id: u64,
    message_id: u64,
    channel_id: u64,
}

pub struct HelpCreationMessagesCache {
    cache: Mutex<LruCache<u64, UserMessage>>,
}

impl HelpCreationMessagesCache {
    fn new() -> Self {
        Self { cache: Mutex::new(LruCache::new(128)) }
    }
    pub fn put(
        &self,
        thread_id: u64,
        user_id: u64,
        channel_id: u64,
        message_id: u64,
    ) -> Option<UserMessage> {
        self.cache
            .lock()
            .unwrap()
            .put(thread_id, UserMessage { user_id, channel_id, message_id })
    }
    pub fn get_message(&self, thread_id: u64, user_id: u64) -> Option<(ChannelId, MessageId)> {
        match self.cache
            .lock()
            .unwrap()
            .peek(&thread_id)
        {
            Some(message) => {
                if message.user_id != user_id {
                    None
                } else {
                    Some((ChannelId(message.channel_id), MessageId(message.message_id)))
                }
            },
            None => None,
        }
    }
    pub fn remove(&self, thread_id: u64) -> Option<UserMessage> {
        self.cache
            .lock()
            .unwrap()
            .pop(&thread_id)
    }
}

#[derive(Clone)]
pub struct Caches {
    pub cooldowns: Arc<CooldownsCache>,
    pub selected_channels: Arc<SelectedChannelsCache>,
    pub help_creation_messages: Arc<HelpCreationMessagesCache>,
}

impl Caches {
    pub fn new() -> Self {
        Self {
            cooldowns: Arc::new(CooldownsCache::new()),
            selected_channels: Arc::new(SelectedChannelsCache::new()),
            help_creation_messages: Arc::new(HelpCreationMessagesCache::new()),
        }
    }
}