use crate::Validator;
use mongodb::{error::Error, options::FindOptions, results::InsertOneResult, sync::Collection};
#[derive(Clone)]
pub struct UserService {
    collection: mongodb::sync::Collection<bson::Document>,
}

impl UserService {
    pub fn new(collection: Collection<bson::Document>) -> UserService {
        UserService { collection }
    }

    pub fn create(
        &self,
        epoch: &i32,
        network_participation: &i32,
        validator_participation: &i32,
    ) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(bson::doc! {"epoch": epoch,"network participation":network_participation,"validator participation":validator_participation}, None)
    }

    pub fn get(&self, epoch: &i32) -> Result<Option<bson::Document>, Error> {
        self.collection.find_one(bson::doc! {"epoch":epoch}, None)
    }
    pub fn is_exist(&self, epoch: &i32) -> Result<bool, Error> {
        let result = self
            .collection
            .find_one(bson::doc! {"epoch":epoch}, None)
            .unwrap();
        match result {
            Some(_docs) => {
                return Ok(true);
            }
            None => {
                return Ok(false);
            }
        }
    }
    pub async fn get_all_users(&self) -> Result<Vec<Validator>, Error> {
        let find_options = FindOptions::builder()
            .limit(5)
            .sort(bson::doc! { "epoch": -1 })
            .build();
        let mut cursor = self.collection.find(bson::doc! {}, find_options).unwrap();

        let mut res: Vec<Validator> = Vec::new();
        while let Some(result) = cursor.next() {
            match result {
                Ok(doc) => {
                    let mut resource_maker: Validator = Validator::new();
                    resource_maker.epoch = doc.get_i32("epoch").unwrap();
                    resource_maker.validator_participation =
                        doc.get_i32("validator participation").unwrap();
                    resource_maker.network_participation =
                        doc.get_i32("network participation").unwrap();

                    res.push(resource_maker);
                }
                Err(e) => {
                    eprintln!("Error retrieving document: {}", e);
                }
            }
        }
        Ok(res)
    }
}
