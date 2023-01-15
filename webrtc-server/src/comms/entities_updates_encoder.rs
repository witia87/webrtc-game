use prost::{Message};
use crate::messages::entities_updates::EntitiesUpdate;
use crate::messages::notifications::NotificationType::EntitiesUpdatesNotification;
use crate::messages::notifications::{Notification, EntitiesUpdatesNotificationPayload};

pub fn encode_entities_updates_notification(entities_updates: Vec<EntitiesUpdate>)
                                            -> Vec<u8> {
    let notification_payload = EntitiesUpdatesNotificationPayload {
        entities_updates
    };

    let notification = Notification {
        notification_type: EntitiesUpdatesNotification as i32,
        notification_payload: notification_payload.encode_to_vec(),
    };

    notification.encode_to_vec()
}
