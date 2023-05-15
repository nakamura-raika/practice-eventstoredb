use std::error::Error;

use eventstore::{Client, AppendToStreamOptions, EventData};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct HogeEvent {
    pub id: String,
    pub data: String,
}

type Result<A> = std::result::Result<A, Box<dyn Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = "esdb://localhost:2113?tls=false".parse()?;
    let client = Client::new(settings)?;

    let event1 = HogeEvent {
        id: Uuid::new_v4().to_string(),
        data: "hoge".to_string(),
    };
    let event2 = HogeEvent {
        id: Uuid::new_v4().to_string(),
        data: "fuga".to_string(),
    };

    let event_data1 = EventData::json("hoge-event", event1)?.id(Uuid::new_v4());
    let event_date2 = EventData::json("hoge-event", event2)?.id(Uuid::new_v4());

    client
        .append_to_stream("hoge-stream", &Default::default(), vec![event_data1, event_date2])
        .await?;

    let mut stream = client
        .read_stream("hoge-stream", &Default::default()).await?;

    while let Some(event) = stream.next().await? {
        let event = event.event;
        println!("{:?}", event);
    }

    Ok(())
}
