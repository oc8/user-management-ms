use yaml_rust::YamlLoader;
use std::fs;
use crate::models::user::{User};

impl From<yaml_rust::Yaml> for User {
    fn from(yaml: yaml_rust::Yaml) -> Self {
        let id = yaml[0]["fields"]["id"].as_str()
            .expect("id field is required in the fixture");
        let id = uuid::Uuid::parse_str(id)
            .expect("id field must be a valid UUID");
        let email = yaml[0]["fields"]["email"].as_str().map(|s| s.to_string())
            .expect("email field is required in the fixture");
        let otp_secret = yaml[0]["fields"]["otp_secret"].as_str().map(|s| s.to_string())
            .expect("otp_secret field is required in the fixture");
        User {
            id,
            email,
            otp_secret,
        }
    }
}


pub fn load_fixtures_from_yaml<T>(file_path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: std::convert::From<yaml_rust::Yaml>,
{
    let file_content = fs::read_to_string(file_path)?;

    let yaml_docs = YamlLoader::load_from_str(&file_content)?;
    let yaml_doc = &yaml_docs[0];

    let mut fixtures = Vec::new();
    if let yaml_rust::Yaml::Hash(hash_map) = yaml_doc {
        for (_, value) in hash_map {
            let fixture: T = T::from(value.clone());
            fixtures.push(fixture);
        }
    }

    Ok(fixtures)
}