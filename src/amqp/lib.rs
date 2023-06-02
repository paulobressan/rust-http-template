use std::{error::Error, sync::Arc};

use deadpool_postgres::Pool;
use futures::StreamExt;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicRejectOptions,
        ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions,
    },
    types::{AMQPValue, FieldTable},
    Connection, ConnectionProperties,
};

use crate::{
    amqp::{config::get_config, dto::CategoryMessage},
    domain::categories::{self, repository::CategoryRepository},
    repository::categories::PgCategoryRepository,
};

pub async fn run(pg_pool: Arc<Pool>) -> Result<(), Box<dyn Error>> {
    let config = get_config();

    let connection =
        Arc::new(Connection::connect(&config.amqp_addr, ConnectionProperties::default()).await?);

    connection.on_error(|err| {
        log::error!("{}", err);
        std::process::exit(1);
    });

    let declare_channel = connection.create_channel().await?;

    declare_channel
        .exchange_declare(
            "categories-dead-letter.exchange",
            lapin::ExchangeKind::Direct,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;
    declare_channel
        .queue_declare(
            "categories-dead-letter.queue",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;
    declare_channel
        .queue_bind(
            "categories-dead-letter.queue",
            "categories-dead-letter.exchange",
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    declare_channel
        .exchange_declare(
            "categories.exchange",
            lapin::ExchangeKind::Direct,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    let mut queue_field = FieldTable::default();
    queue_field.insert(
        "x-dead-letter-exchange".into(),
        AMQPValue::LongString("categories-dead-letter.exchange".into()),
    );
    declare_channel
        .queue_declare(
            "categories.queue",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            queue_field,
        )
        .await?;
    declare_channel
        .queue_bind(
            "categories.queue",
            "categories.exchange",
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;
    declare_channel.close(0, "declare channel fineshed").await?;

    let category_repository: Arc<dyn CategoryRepository> =
        Arc::new(PgCategoryRepository::new(pg_pool.clone()));

    let consumer_channel = connection.create_channel().await?;
    let mut consumer = consumer_channel
        .basic_consume(
            "categories.queue",
            "consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    log::info!("server listener categories.queue");
    while let Some(result) = consumer.next().await {
        if let Ok(delivery) = result {
            match serde_json::from_slice::<CategoryMessage>(delivery.data.as_slice()) {
                Ok(category_message) => {
                    match categories::resources::create::execute(
                        category_repository.clone(),
                        category_message.into(),
                    )
                    .await
                    {
                        Ok(_) => delivery.ack(BasicAckOptions::default()).await?,
                        Err(err) => {
                            log::error!("Nack {}", err);
                            delivery.nack(BasicNackOptions::default()).await?;
                        }
                    }
                }
                Err(err) => {
                    log::error!("Reject {}", err);
                    delivery.reject(BasicRejectOptions::default()).await?;
                }
            }
        }
    }

    Ok(())
}
