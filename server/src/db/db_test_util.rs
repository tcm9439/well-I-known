use crate::db::db_connection::DbConnection;
use std::path::{PathBuf, Path};
use std::fs;

#[cfg(test)]
pub fn get_test_path(filename: &str) -> PathBuf {
    let base_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(base_dir).join(filename).to_path_buf()
}

#[cfg(test)]
/// Create a new database for the test case by copying the base database
pub async fn create_test_db(test_case_name: &str) -> DbConnection{

    let test_case_db_path = get_test_path(format!("output/{}.db", test_case_name).as_str());
    let test_base_path = get_test_path("resources/test/base-test.db");
    
    // delete the file if it exists
    if test_case_db_path.exists() {
        fs::remove_file(&test_case_db_path).unwrap();
    }
    // copy the base database
    fs::copy(&test_base_path, &test_case_db_path).unwrap();

    // create the connection
    DbConnection::new(&test_case_db_path).await.unwrap()
}
