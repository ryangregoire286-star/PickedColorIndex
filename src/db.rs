use mongodb::{bson::{Document, doc}, Client, Collection};
use std::env;
use dotenv::dotenv;


pub struct DBType {
    connect_uri: String,
    is_owned: bool
}


#[tokio::main]

pub async fn get_client(data_data: &str) -> mongodb::error::Result<()> {
    let data = DBType {
        connect_uri: String::from("mongodb://localhost:27017"),
        is_owned: true,
    };

    if data.is_owned {

        let keys = data.connect_uri;

        dotenv().ok();

        let key = env::var(keys);

        match key {
            Ok(val) => {
                let m = Client::with_uri_str(val).await?;

                let db = m.database("colorSelected");
                let col: Collection<Document> = db.collection("Color");

                let data_doc: &str = data_data;

                col.insert_one(doc! {"data": data_doc}).await?;

            }

            _ => println!("{}", "Cannot run Code Default")
        }
    }

    Ok(())
}

