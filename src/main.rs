pub mod deduction {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Deduction {
        pub person: u32,
        pub deduction: f32,
        pub reason: String,
        pub date: String,
        pub place: String,
        pub deductor: u32,
        pub description: String,
        pub id: String,
    }
    pub fn delete(id: String) -> Result<String, String> {
        let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
            .map_err(|error| error.to_string())?;
        let collection = client
            .database("magnifique")
            .collection::<Deduction>("deduction");
        let result = collection.delete_one(
            mongodb::bson::doc! {
              "id": id
            },
            None,
        );
        match result {
            Ok(result) => {
                if result.deleted_count == 1 {
                    Ok("Deduction deleted".to_string())
                } else {
                    Err("Failed to delete deduction".to_string())
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }
    pub fn create(deduction: Deduction) -> Result<String, String> {
        let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
            .map_err(|error| error.to_string())?;
        let collection = client.database("magnifique").collection("deduction");
        let result = collection.insert_one(
            mongodb::bson::doc! {
                "person": &deduction.person,
                "deduction": &deduction.deduction,
                "date": &deduction.date,
                "id": &deduction.id,
                "description": &deduction.description,
                "reason": &deduction.reason,
                "place": &deduction.place,
                "deductor": &deduction.deductor,
            },
            None,
        );
        match result {
            Ok(result) => Ok(result.inserted_id.to_string()),
            Err(error) => Err(error.to_string()),
        }
    }
    pub fn list() -> Option<Vec<Deduction>> {
        let client = mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")
            .map_err(|error| error.to_string())
            .ok()?;
        let collection = client
            .database("magnifique")
            .collection::<Deduction>("deduction");
        let mut result: Vec<Deduction> = Vec::new();
        for item in collection.find(None, None).ok()? {
            result.push(item.unwrap());
        }
        Some(result)
    }
}

fn main() {
    deduction::create(deduction::Deduction {
        person: 1,
        deduction: 100.0,
        reason: "".to_string(),
        date: "".to_string(),
        place: "".to_string(),
        deductor: 1,
        description: "".to_string(),
        id: "".to_string(),
    })
    .unwrap();
    let result = deduction::list().unwrap();
    deduction::delete("".to_string()).unwrap();
    println!("{:?}", result);
}
