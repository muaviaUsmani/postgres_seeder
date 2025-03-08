use database::database::establish_connection;

pub mod database;
pub mod interfaces;
pub mod models;
pub mod schema;
pub mod service;

fn main() {
  let mut db_connection = establish_connection();
  match service::generate_data(&mut db_connection, true) {
    Ok(_) => println!("Data generation completed successfully."),
    Err(err) => eprintln!("Error generating data: {}", err),
  }
}
