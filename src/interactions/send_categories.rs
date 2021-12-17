use serenity::model::prelude::{InteractionApplicationCommandCallbackDataFlags, InteractionResponseType};
use serenity::model::prelude::message_component::MessageComponentInteraction;
use serenity::prelude::Context;
use sqlx::SqlitePool;

pub const HELP_SELECT_CATEGORY_ID: &'static str = "help_select_category";

pub async fn send_categories_fn(
    component: &MessageComponentInteraction,
    ctx: &Context,
    db: &SqlitePool,
) {
    let categories = read_categories(db)
        .await;

    component.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|interaction| {
                interaction
                    .content("\
                        Aşağıdan önce sormak istediğiniz genel kategoriyi seçiniz. Sorunuz eğer ki \
                        bir dil ile alakalıysa dili; spesifik olarak dilin kullanıldığı bir \
                        platformla, mesela web framework, oyun motoru veya kütüphane ile alakalıysa \
                        onları seçiniz. Eğer ki emin değilseniz dili seçebilirsiniz, çok spesifik \
                        olması durumunda uygun kanala yönlendirme yapılacaktır.\
                    ")
                    .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                    .components(|components| {
                        components.create_action_row(|row| {
                            row.create_select_menu(|select| {
                                select
                                    .custom_id(HELP_SELECT_CATEGORY_ID)
                                    .min_values(1)
                                    .max_values(1)
                                    .placeholder("Sormak istediğiniz kategori")
                                    .options(|options| {
                                        for CategoryRecord {id, name} in categories {
                                            options.create_option(|option| {
                                                option
                                                    .label(name)
                                                    .value(id)
                                            });
                                        }

                                        options
                                    })
                            })
                        })
                    })

            })
    })
        .await
        .expect("");
}

pub struct CategoryRecord {
    id: i64,
    name: String,
}

pub async fn read_categories(db: &SqlitePool) -> Vec<CategoryRecord> {
    sqlx::query_as!(
        CategoryRecord,
        "SELECT id, name FROM help_categories"
    )
        .fetch_all(db)
        .await
        .unwrap()
}