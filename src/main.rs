//connect to mariadb sql server on local network
use mysql::prelude::*;
use mysql::*;
use csv::Reader;
fn main(){
   let data= read_csv();
    println!("{:?}", data);
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
fn database_connection(data:&Vec<Data>) -> std::result::Result<(), Box<dyn std::error::Error>>  {
    
    let url = "mysql://kylelocal:kcb@127.0.0.1:3306/testcsv";
    mysql::Opts::try_from(url)?;
    
   //let url=get_opts();
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    conn.exec_batch(
        r"INSERT INTO Data(id, name, age, address, salary)
        VALUES (:id, :name, :age, :address, :salary)",
        data.iter().map(|p| params! {
            "id" => p.id,
            "name" => &p.name,
            "age" => p.age,
            "address" => &p.address,
            "salary" => p.salary,
        }),
    )?;
    let selected_data=conn.query_map(
        "SELECT id, name, age, address, salary FROM Data",
        |(id, name, age, address, salary)| Data {
            id,
            name,
            age,
            address,
            salary,
        },
    )?;
    println!("{:?}",selected_data);
    Ok(())
    //todo
}
fn read_csv() -> std::result::Result<(), Box<dyn std::error::Error>> {
    
//fn read_csv() ->Vec<Data> { 
    let mut rdr = Reader::from_path("data.csv")?;
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

    database_connection(&data);
    println!("{:?}", data);
    Ok(())
}
/*
fn read_csv(file_path: &str, headers:bool)->Data {
let file = std::fs::File::open(file_path).unwrap();
let mut rdr=csv::ReaderBuilder::new()
    .has_headers(headers)
    .from_reader(file);

    let mut data_frame=Data::new();
    for result in rdr.records(){
        let record=result.unwrap();
        data_frame.push(&record);
    }
    return data_frame;
}
fn new() -> Data {
       Data {
           Header: csv::StringRecord::new(),
           id: i32,
           name: String,
           age: i32,
           address: String,
           salary: i32,
       }
   }
fn push(&mut self, row: &csv::StringRecord) {
       self.id.push(row[0].parse().unwrap());
       self.name.push(row[1].to_string());
       self.age.push(row[2].parse().unwrap());
       self.address.push(row[3].to_string());
       self.salary.push(row[4].parse().unwrap());
   }
}
*/
