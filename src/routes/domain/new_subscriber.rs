use crate::routes::domain::subscriber_name::SubscriberName;
use crate::routes::domain::subscriber_email::SubscriberEmail;
pub struct NewSubscriber{
    pub email:SubscriberEmail,
    pub name:SubscriberName
}

