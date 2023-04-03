//connect to mariadb sql server on local network
use clap::Parser;
use csv::Reader;
use mysql::prelude::*;
use mysql::*;
pub mod pushdata;
pub mod tablecreate;
fn main() {
    let pattern = std::env::args().nth(1).expect("No file given");
    let path= std::env::args().nth(2).expect("No file given");

    let args = CLI::parse();
    match args.pattern.as_str() {
        "create" => {
            let mut connection = database_connection();
            let columnname = vec!["id".to_string(), "name".to_string(), "age".to_string(), "address".to_string(), "salary".to_string()];
            tablecreate::create_table(&mut connection, &args.path.to_str().unwrap(), &columnname);
        }
        "insert" => {
           // pushdata::push_data();
            pushdata::read_csv(&args.path.display().to_string());
        }
        _ => {
            println!("No command given");
        }
    }
    

}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    id: i32,
    name: String,
    age: i32,
    address: String,
    salary: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Table {
    tablename: String,
    columnname: Vec<String>,
    columntype: Vec<String>,
}
#[derive(Parser)]
struct CLI {
    pattern: String,
    path:std::path::PathBuf,

}
fn database_connection() -> PooledConn {
    let url = "mysql://kylelocal:kcb@127.0.0.1:3306/testcsv";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    conn
}
