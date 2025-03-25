use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::model::notification::Notification;

pub struct NotificationRepository;

lazy_static!{
    static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

impl NotificationRepository{
    
}