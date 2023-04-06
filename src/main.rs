//connect to mariadb sql server on local network
use clap::Parser;
use csv::Reader;
use mysql::prelude::*;
use mysql::*;
pub mod pushdata;
pub mod getfields;
pub mod tablecreate;
fn main() {
    let pattern = std::env::args().nth(1).expect("No Command Given");
    let table = std::env::args().nth(2).expect("No Table");
    let path= std::env::args().nth(3).expect("No file given");

    let args = CLI::parse();
    match args.pattern.as_str() {
        "create" => {

            let mut connection = database_connection();
            let tablename = args.table;
            let columns=getfields::read_fields(&args.path.display().to_string());
            tablecreate::create_table(&mut connection, &tablename, &columns);
        }
        "insert" => {

            let columns=getfields::read_fields(&args.path.display().to_string());
            pushdata::createtablestruct::read_csv2(&args.path.display().to_string());
        }
        _ => {
            println!("No command given");
        }
    }

}

#[derive(Debug, PartialEq, Eq)]
struct Data2 {
    columns: column
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
struct ColData{
    fields: Vec<String>,
}
#[derive(Parser)]
struct CLI {
    pattern: String,
    table: String,
    path:std::path::PathBuf,

}
type column = Vec<String>;

struct Table{
    tablename: String,
    columnname: Vec<column>,
}
fn database_connection() -> PooledConn {
    let url = "mysql://kylelocal:kcb@127.0.0.1:3306/testcsv";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    conn
}
