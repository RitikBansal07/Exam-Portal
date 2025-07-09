use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewTopic {
    pub topic: String,
}
#[derive(Debug, Serialize,Deserialize)]
pub struct Topic {
   pub id: i32,
   pub topic: String,
}
