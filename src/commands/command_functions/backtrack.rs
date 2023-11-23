use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::utils::Colour;

use crate::MessageStorageContainer;

pub async fn get_backtracked_message(ctx: &Context, msg: &Message) -> CreateEmbed {
    let channel_id = msg.channel_id.as_u64();

    let msg_storage_hashmap = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<MessageStorageContainer>()
            .expect("Expected Message Storage In TypeMap.")
            .clone()
    };

    let mut formatted_msg = String::new();

    // Lock Block
    {
        let msg_storage_hashmap_locked = msg_storage_hashmap.lock().await;

        match msg_storage_hashmap_locked.get(channel_id) {
            Some(msg_storage_vector) => {
                for msg in msg_storage_vector.iter() {
                    formatted_msg += msg;
                }
            }
            None => formatted_msg += "Cannot Retrieve Old Messages",
        }
    }

    let mut embed = CreateEmbed::default();
    embed
        .colour(Colour::RED)
        .description(formatted_msg)
        .timestamp(msg.timestamp);

    return embed;
}
