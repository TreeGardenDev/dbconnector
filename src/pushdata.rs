use crate::Data;
use mysql::prelude::*;
use mysql::*;
use crate::Reader;
fn execute_insert(
    data: &Vec<Data>,
    mut conn: PooledConn,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    //read from csv the column names
   //execute sql statement below
    //let columnname = conn.query_map("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA='testcsv' AND TABLE_NAME='Data'", |(COLUMN_NAME)| COLUMN_NAME)?;
    let columnname: Vec<String> = conn.query_map("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA='testcsv' AND TABLE_NAME='Data'", |(COLUMN_NAME)| COLUMN_NAME)?;
    for i in columnname{
        println!("{:?}", i.to_string());
    }
   //SELECT `COLUMN_NAME`  FROM `INFORMATION_SCHEMA`.`COLUMNS`  WHERE `TABLE_SCHEMA`='testcsv' AND `TABLE_NAME`='Data'; 
   
    
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
//    let selected_data = conn.query_map(
//        "SELECT id, name, age, address, salary FROM Data",
//        |(id, name, age, address, salary)| Data {
//            id,
//            name,
//            age,
//            address,
//            salary,
//        },
//    )?;
//    println!("{:?}", selected_data);
    Ok(())
    //todo
}
pub fn read_csv(file: &String) -> std::result::Result<(), Box<dyn std::error::Error>> {
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

    let connection = crate::database_connection();
    execute_insert(&data, connection);
    Ok(())
}
