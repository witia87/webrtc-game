use prost::{Message};
use crate::messages::notifications::NotificationType::{PlayerSetupNotification};
use crate::messages::notifications::{Notification, PlayerSetupNotificationPayload};

pub fn encode_player_setup_notification(assigned_player_id: &u32)
                                        -> Vec<u8> {
    let notification_payload = PlayerSetupNotificationPayload {
        assigned_player_id: assigned_player_id.clone()
    };

    let notification = Notification {
        notification_type: PlayerSetupNotification as i32,
        notification_payload: notification_payload.encode_to_vec(),
    };

    notification.encode_to_vec()
}
