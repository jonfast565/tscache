use serde::{Deserialize, Serialize};
use crate::utilities::*;

/*** 
 * Contains the cache settings
 * ***/
 
 #[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    data_directory: String,
    index_filename: String,
    data_file_prefix: String,
}