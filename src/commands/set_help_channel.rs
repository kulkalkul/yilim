use serenity::model::interactions::message_component::ButtonStyle;
use serenity::model::prelude::application_command::{
    ApplicationCommandOptionType,
    ApplicationCommandPermissionType
};
use serenity::model::prelude::InteractionApplicationCommandCallbackDataFlags;
use sqlx::SqlitePool;
use crate::command::{Command, Response};
use crate::commands::set_guidelines_channel::GUIDELINES_CHANNEL_NAME;
use crate::commands::set_help_log_answered_channel::HELP_LOG_ANSWERED_CHANNEL_NAME;
use crate::commands::set_help_log_waiting_channel::HELP_LOG_WAITING_CHANNEL_NAME;
use crate::config::Config;

pub const HELP_MESSAGE: &'static str = "help";
pub const HELP_BUTTON_CLICK_ID: &'static str = "help_button_click";

const CHANNEL_OPTION: &'static str = "channel";

pub fn set_help_channel_fn(mut command: Command) -> Command {
    let Config {administrator_id, .. } = *command.config;

    command
        .description("Yardım kanalını ayarlar ve yardım mesajını yollar.")
        .default_permission(false)
        .create_option(|option| {
            option
                .name(CHANNEL_OPTION)
                .kind(ApplicationCommandOptionType::Channel)
                .required(true)
                .description("Ayarlanacak yardım kanalı.")
        })
        .set_permissions(move |permissions| {
            permissions
                .create_permission(|permission| {
                    permission
                        .id(administrator_id)
                        .kind(ApplicationCommandPermissionType::User)
                        .permission(true)
                })
        })
        .set_response(|mut ctx| async move {
            let channel = ctx.options
                .read_channel(CHANNEL_OPTION)
                .await
                .unwrap();

            if let Ok(record) = read_help_message(&ctx.db).await {
                ctx.http
                    .delete_message(record.channel_id as u64, record.id as u64)
                    .await
                    .ok();
            }

            let guidelines = match read_guidelines_channel(&ctx.db).await {
                Ok(id) => id,
                Err(_) => return Response::message(move |message| {
                    message
                        .content("\
                            Yönergeler kanalı mevcut değil, ``/set_guidelines_channel``\
                            komutunu kullanarak oluşturun.\
                        ")
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                }),
            };

            let help_log_waiting = match read_help_log_answered_channel(&ctx.db).await {
                Ok(id) => id,
                Err(_) => return Response::message(move |message| {
                    message
                        .content("\
                            Help log bekleyen kanalı mevcut değil, ``/set_help_log_waiting_channel``\
                            komutunu kullanarak oluşturun.\
                        ")
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                }),
            };

            if let Err(_) = read_help_log_waiting_channel(&ctx.db).await {
                return Response::message(move |message| {
                    message
                        .content("\
                            Help log çözümlenmiş kanalı mevcut değil, ``/set_help_answered_log_channel``\
                            komutunu kullanarak oluşturun.\
                        ")
                        .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                });
            }
            let message = channel.id.send_message(&ctx.http, |message| {
                message
                    .content(format!("\
                        Organizasyonun daha kolay olması açısından yardım kanallarını dolaylı \
                        yoldan bot aracılığıyla kullanıyoruz. Soru sormadan veya birisinin sorusunu \
                        cevaplamadan önce ilk olarak <#{}>'i okuduğunuzdan ve anladığınızdan emin \
                        olunuz. Sorulan her soru <#{}> kanalına gidecektir, buradan cevap almamış \
                        soruları takip edebilirsiniz.\
                        \n\
                        \n\
                        Sorunuzu sorarken daha kolay cevap alabilmeniz açısından bazı önerilerimiz \
                        mevcut. Bunlar;\
                        \n\
                        **1 -** En uygun kategori ve kanala sorduğunuzdan emin olun.\
                        \n\
                        **2 -** Kolay anlaşılabilir bir dile sahip olmasına özen gösterin.\
                        \n\
                        **3 -** Yeterli miktarda kod paylaşın, sorununuzun bir başkası tarafından \
                        test edilebilir olmasını sağlayın. Eğer ki sorununuzun çözülmesi için size \
                        kodunuzla alakalı tekrar tekrar soru sorulup sizden ek bilgi/kod isteniyorsa \
                        eksik bilgi vermişsiniz demektir.\
                        \n\
                        **4 -** Alâkasız detayları atlayın. Sorunuz rahat anlaşılabilir olması \
                        bakımından olabildiğince sade olmalı. Bir şeyi neden yaptığınız, eğer ki \
                        sorununuzun anlaşılması için bir faydada bulunmuyorsa bunu söylemeyi \
                        tekrardan düşünün.\
                        \n\
                        **5 -** XY probleminden kaçının. Eğer ki yapmak istediğiniz şey X ise ve \
                        bunu Y ile çözebileceğinize inanıyorsanız kendinizi çok çabuk bir şekilde \
                        Y ile alakalı bir sıkıntıyı çözerken ve bunu sorarken bulabilirsiniz. Bu \
                        hatalı bir çıkarım olduğu vakit boşu boşuna Y'ye çözüm üretmeye vakit \
                        harcamış olursunuz.\
                        \n\
                        **6 -** En önemlisi size yardım eden kişinin dakikalarını çaldığınızı asla \
                        unutmayın. Eğer ki karşıdaki sizin sorununuzu çözmek için 15 dakika vaktini \
                        feda edebiliyorsa, siz o sorunu çözmek için en az bir 15 dakika, soracağınız \
                        soruyu hazırlamak için de ayrı bir 15 dakika harcamış olmalısınız.\
                        \n\
                        \n\
                        Eğer ki yönergeler ve öneriler konusunda bir sıkıntı çekeceğinizi \
                        düşünmüyorsanız lütfen aşağıdaki butona tıklayarak sorunuzu sormaya \
                        devam ediniz.\
                    ", guidelines.id as u64, help_log_waiting.id as u64))
                    .components(|components| {
                        components.create_action_row(|row| {
                            row.create_button(|button| {
                                button
                                    .custom_id(HELP_BUTTON_CLICK_ID)
                                    .label("Kategorileri Göster")
                                    .style(ButtonStyle::Primary)
                            })
                        })
                    })
            })
                .await
                .expect("Error while sending message");

            insert_help_message(&ctx.db, message.channel_id.0 as i64, message.id.0 as i64)
                .await;

            Response::message(move |message| {
                message
                    .content(format!("\
                        Yardım kanalı başarıyla <#{}> olarak ayarlandı.\
                    ", channel.id.0))
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
            })
        });

    command
}

struct MessageRecord {
    channel_id: i64,
    id: i64,
}
struct ChannelRecord {
    id: i64,
}

async fn read_help_message(db: &SqlitePool) -> Result<MessageRecord, sqlx::Error> {
    sqlx::query_as!(
        MessageRecord,
        "SELECT id, channel_id FROM messages WHERE name = ?",
        HELP_MESSAGE,
    )
        .fetch_one(db)
        .await
}

async fn read_guidelines_channel(db: &SqlitePool) -> Result<ChannelRecord, sqlx::Error> {
    sqlx::query_as!(
        ChannelRecord,
        "SELECT id FROM channels WHERE name = ?",
        GUIDELINES_CHANNEL_NAME,
    )
        .fetch_one(db)
        .await
}

async fn read_help_log_answered_channel(db: &SqlitePool) -> Result<ChannelRecord, sqlx::Error> {
    sqlx::query_as!(
        ChannelRecord,
        "SELECT id FROM channels WHERE name = ?",
        HELP_LOG_ANSWERED_CHANNEL_NAME,
    )
        .fetch_one(db)
        .await
}
async fn read_help_log_waiting_channel(db: &SqlitePool) -> Result<ChannelRecord, sqlx::Error> {
    sqlx::query_as!(
        ChannelRecord,
        "SELECT id FROM channels WHERE name = ?",
        HELP_LOG_WAITING_CHANNEL_NAME,
    )
        .fetch_one(db)
        .await
}


async fn insert_help_message(db: &SqlitePool, channel_id: i64, message_id: i64) {
    sqlx::query!(
        "INSERT OR REPLACE INTO messages (name, id, channel_id) VALUES (?, ?, ?)",
        HELP_MESSAGE,
        message_id,
        channel_id,
    )
        .execute(db)
        .await
        .unwrap();
}