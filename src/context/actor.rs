use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Deserialize, Serialize, EnumString)]
pub enum ContextActor {
    Admin,
    Parent,
    Student,
}
