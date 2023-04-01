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
            read_csv(&args.path.display().to_string());
        }
        _ => {
            println!("No command given");
        }
    }
    

    // let data= read_csv();
    // println!("{:?}", data);
    //    database_connection();
}

//data that will be processed
#[derive(Debug, PartialEq, Eq)]
struct Data {
    //  Header:csv::StringRecord,
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
fn execute_insert(
    data: &Vec<Data>,
    mut conn: PooledConn,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    conn.exec_batch(
        r"INSERT INTO Data(id, name, age, address, salary)
        VALUES (:id, :name, :age, :address, :salary)",
        data.iter().map(|p| {
            params! {
                "id" => p.id,
                "name" => &p.name,
                "age" => p.age,
                "address" => &p.address,
                "salary" => p.salary,
            }
        }),
    )?;
    let selected_data = conn.query_map(
        "SELECT id, name, age, address, salary FROM Data",
        |(id, name, age, address, salary)| Data {
            id,
            name,
            age,
            address,
            salary,
        },
    )?;
    println!("{:?}", selected_data);
    Ok(())
    //todo
}
fn read_csv(file: &String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    //fn read_csv() ->Vec<Data> {
    let mut rdr = Reader::from_path(file)?;
    let mut data: Vec<Data> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let id = record[0].parse::<i32>()?;
        let name = record[1].to_string();
        let age = record[2].parse::<i32>()?;
        let address = record[3].to_string();
        let salary = record[4].parse::<i32>()?;
        data.push(Data {
            id,
            name,
            age,
            address,
            salary,
        });
    }

    let connection = database_connection();
    execute_insert(&data, connection);
    println!("{:?}", data);
    Ok(())
}
//add CLI to read csv file
