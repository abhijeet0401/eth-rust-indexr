use actix_web::{web, App, HttpServer};
use serde_json::Value;
mod controller;
mod request;
mod service;
mod common;
use common::*;
use eventsource_stream::Eventsource;
use futures::stream::StreamExt;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ServiceContainer {
    user: service::UserService,
}

impl ServiceContainer {
    pub fn new(user: service::UserService) -> Self {
        ServiceContainer { user }
    }
}

pub struct AppState {
    service_container: Arc<Mutex<ServiceContainer>>,
}
async fn event_stream(service_container: Arc<Mutex<ServiceContainer>>) {
    let mut stream = reqwest::Client::new()
        .get("https://docs-demo.quiknode.pro/eth/v1/events?topics=finalized_checkpoint")
        .send()
        .await
        .expect("not able to parse stream")
        .bytes_stream()
        .eventsource();

    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => {
                let json_epoch: Value = serde_json::from_str(&event.data).unwrap();
                let upcoming_epoch = &json_epoch["epoch"];
                println!("Upcoming epoch {}", upcoming_epoch);
                let epoch_int = upcoming_epoch.as_str().unwrap().parse::<i32>().unwrap();
                let ue_validator_length = request::get_propser(&epoch_int).await.unwrap() as i32;
                let new_index_result: Validator =
                    request::calculate_result(&epoch_int, &ue_validator_length);

                let _ = service_container.lock().unwrap().user.create(
                    &new_index_result.epoch,
                    &new_index_result.network_participation,
                    &new_index_result.validator_participation,
                );
            }
            Err(e) => eprintln!("error occured: {}", e),
        }
    }
}
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let client_options =
        mongodb::options::ClientOptions::parse("mongodb://localhost:27017").unwrap();
    let client = mongodb::sync::Client::with_options(client_options).unwrap();
    let db = client.database("mydb");
    let mut validator_collector: Vec<Validator> = Vec::new();
    let user_collection = db.collection("users");
    let service_container = Arc::new(Mutex::new(ServiceContainer::new(
        service::UserService::new(user_collection.clone()),
    )));
    let mut sync: bool = false;
    let mut latest_epoch = request::get_finalized_epoch().await.unwrap();
    let service_container_event = service_container.clone();
    println!("Syncing");
    //modular cursor get method
    tokio::spawn(async move { event_stream(service_container_event).await });

    for _i in 0..5 {
        let validator_size = request::get_propser(&latest_epoch).await.unwrap() as i32;
        let index_result: Validator =
            request::calculate_result(&latest_epoch, &validator_size);

        validator_collector.push(index_result);
        println!("Syncing process {:?}", (_i/5)*100);
        latest_epoch = latest_epoch - 1;
    }

    if validator_collector.len() == 5 {
        for i in validator_collector.iter() {
            let is_elementexist = service_container
                .clone()
                .lock()
                .unwrap()
                .user
                .is_exist(&i.epoch)
                .unwrap();
            if !is_elementexist {
                let _ = service_container.clone().lock().unwrap().user.create(
                    &i.epoch,
                    &i.network_participation,
                    &i.validator_participation,
                );
            }
        }
        sync = true;
    }

    if sync == true {
        println!("Syncing Complete");
        //see for new epoch
        let _ = service_container
            .clone()
            .lock()
            .unwrap()
            .user
            .get_all_users()
            .await;

        let service_container_data = web::Data::new(AppState { service_container });

        HttpServer::new(move || {
            App::new()
                .app_data(service_container_data.clone())
                .route("/get", web::get().to(controller::get))
        })
        .bind("0.0.0.0:3000")?
        .run()
        .await
    } else {
        Ok(println!("Error DB"))
    }
}
