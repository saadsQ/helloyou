use grammers_client::Client;
use grammers_client::types::PackedChat;
use grammers_client::Update;
use grammers_session::PackedType;
use grammers_tl_types::enums::MessageEntity;
use std::collections::HashSet;
use std::error::Error;
use std::time::Instant;
use tokio::signal;

pub async fn handle_shutdown(client: Client, session_path: String) {
    if let Err(e) = signal::ctrl_c().await {
        eprintln!("Failed to listen for shutdown signal: {}", e);
        return;
    }
    println!("Received shutdown signal. Logging out...");
    if let Err(e) = client.sign_out().await {
        eprintln!("Failed to log out: {}", e);
    }
    if std::fs::remove_file(session_path).is_err() {
        eprintln!("Failed to remove session file");
    }
    std::process::exit(0);
}

pub async fn handle_new_messages(client: &Client, channel_id: i64) -> Result<(), Box<dyn Error>> {
    let mut sent_messages: HashSet<String> = HashSet::new();
    println!("Listening for new messages in {}...", channel_id);

    loop {
        match client.next_update().await? {
            Some(Update::NewMessage(message)) => {
                let chat = message.chat();
                let start = Instant::now();
                if chat.id() == channel_id {
                    let duration = start.elapsed();

                    println!("-----------------------------------------------");
                    println!("New message:");
                    println!("----------------");
                    println!("{}", message.text());
                    println!("----------------");
                      // Extract and print URLs or hyperlinks from message entities
                      if let Some(entities) = message.fmt_entities() {
                        for entity in entities {
                            if let MessageEntity::TextUrl (textUrl ) = entity {
                                println!("Found TextURL: {:?}", textUrl);
                            }
                            if let MessageEntity::Url (url ) = entity {
                                println!("Found URL: {:?}", url);
                            }
                            if let MessageEntity::Unknown (url ) = entity {
                                println!("Found Unknown: {:?}", url);
                            }
                            if let MessageEntity::Mention (url ) = entity {
                                println!("Found Mention: {:?}", url);
                            }
                            if let MessageEntity::Hashtag (url ) = entity {
                                println!("Found Hashtag: {:?}", url);
                            }
                            if let MessageEntity::BotCommand (url ) = entity {
                                println!("Found BotCommand: {:?}", url);
                            }
                            if let MessageEntity::Email (url ) = entity {
                                println!("Found Email: {:?}", url);
                            }
                            if let MessageEntity::Bold (url ) = entity {
                                println!("Found Bold: {:?}", url);
                            }
                            if let MessageEntity::Italic (url ) = entity {
                                println!("Found Italic: {:?}", url);
                            }
                            if let MessageEntity::Code (url ) = entity {
                                println!("Found Code: {:?}", url);
                            }
                            if let MessageEntity::Pre (url ) = entity {
                                println!("Found Pre: {:?}", url);
                            }
                            if let MessageEntity::MentionName (url ) = entity {
                                println!("Found MentionName: {:?}", url);
                            }
                            if let MessageEntity::InputMessageEntityMentionName (url ) = entity {
                                println!("Found InputMessageEntityMentionName: {:?}", url);
                            }
                            if let MessageEntity::Phone (url ) = entity {
                                println!("Found Phone: {:?}", url);
                            }
                            if let MessageEntity::Cashtag (url ) = entity {
                                println!("Found Cashtag: {:?}", url);
                            }
                            if let MessageEntity::Underline (url ) = entity {
                                println!("Found Underline: {:?}", url);
                            }
                            if let MessageEntity::Strike (url ) = entity {
                                println!("Found Strike: {:?}", url);
                            }
                            if let MessageEntity::BankCard (url ) = entity {
                                println!("Found BankCard: {:?}", url);
                            }
                            if let MessageEntity::Spoiler (url ) = entity {
                                println!("Found Spoiler: {:?}", url);
                            }
                            if let MessageEntity::CustomEmoji (url ) = entity {
                                println!("Found CustomEmoji: {:?}", url);
                            }
                            if let MessageEntity::Blockquote (url ) = entity {
                                println!("Found Blockquote: {:?}", url);
                            }
                        }
                    }
                    println!("----------------");
                    println!("Arrived at: {}", message.date());
                    println!("----------------");
                    println!("Time taken: {:?}", duration);
                    println!("-----------------------------------------------");
                    let addresses = crate::utils::extract_contract_addresses(message.text());
                    for address in addresses {
                        if !sent_messages.contains(&address) {
                          // send_message_to_channel(client, 2162690597, &address).await?;
                            sent_messages.insert(address);
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

pub async fn send_message_to_channel(client: &Client, channel_id: i64, message: &str) -> Result<(), Box<dyn Error>> {
    let packed_chat = PackedChat {
        ty: PackedType::User, // or the appropriate type for your channel
        id: channel_id,
        access_hash: None, // Replace with actual access hash if available
    };

    client.send_message(packed_chat, message).await?;
    println!("Message sent to channel {}: {}", channel_id, message);
    Ok(())
}
