// extern crate tempdir;
// extern crate leveldb;

// use tempdir::TempDir;
use leveldb::database::Database;
// use leveldb::iterator::Iterable;
use leveldb::kv::KV;
use leveldb::options::{Options, ReadOptions, WriteOptions};
use std::path::Path;

fn main() {

    let db_path_str = "./airdb";
    let db_path = Path::new(db_path_str);

    print!("Starting LeveleDB with Airchains \n");

    let mut options = Options::new();
    options.create_if_missing = true;
    let database = match Database::open(db_path, options) {
        Ok(db) => db,
        Err(e) => {
            panic!("failed to open database: {:?}", e)
        }
    };

    let write_opts = WriteOptions::new();

    let key= 2147483647 ; // max 2,147,483,647. This library of rust support only i32 as key. not as stiring like golang and javascript....
    // need to find another way in this library or other library


    let value_string = r#"{"chainid": "uuid9871902834", "vkey": "long vkey" ,"chaininfo": "some information about chain"}"#;
    let value: &[u8] = value_string.as_bytes();

    match database.put(write_opts, key, value) {
        Ok(_) => (),
        Err(e) => {
            panic!("failed to write to database: {:?}", e)
        }
    };

    let read_opts = ReadOptions::new();
    let res = database.get(read_opts, key);

    // Convert byte slice to String
    let data = res.unwrap().unwrap();
    let string_data = String::from_utf8_lossy(&data);

    // Print the resulting String
    println!("Got Data: {}", string_data);

    // println!("Retrieved Data: {:?}", res);

    // match res {
    //     Ok(data) => {
    //         // Assuming the stored data is a JSON string, convert it to a String for better comparison
    //         let stored_value_string = String::from_utf8_lossy(data.as_deref().unwrap_or_default());
    //         let expected_value_string = String::from_utf8_lossy(value);

    //         assert_eq!(stored_value_string, expected_value_string);
    //         println!("Retrieved Data: {}", stored_value_string);
    //     }
    //     Err(e) => panic!("Failed reading data: {:?}", e),
    // }

    // Iterate through the database entries
    // let read_opts = ReadOptions::new();
    // let mut iter = database.iter(read_opts);
    // let entry = iter.next();
    // assert_eq!(entry, Some((key, value.to_vec())));
    // println!("Database Entry: {:?}", entry);
}
