//connect to mariadb sql server on local network
use mysql::prelude::*;
use mysql::*;
use csv::Reader;
use clap::Parser;
fn main(){
    let pattern=std::env::args().nth(1).expect("No file given");
    //let args=CLI{
    //    pattern:pattern,
    //};
    let args=CLI::parse();
   // let content=std::fs::read_to_string(&args.pattern).expect("Could not read file");
   // for line in content.lines(){
   //     if line.contains(&args.pattern){
   //         println!("{}",line);
   //     }
   
   read_csv(&args.pattern);
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
#[derive(Parser)]
struct CLI {
    pattern: String,
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

    database_connection(&data);
    println!("{:?}", data);
    Ok(())
}
//add CLI to read csv file

