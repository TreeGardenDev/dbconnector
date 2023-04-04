use crate::Data;
use mysql::prelude::*;
use mysql::*;
use crate::Reader;

struct InsertData{
    data: Vec<String>,
}

fn execute_insert(
    data: &Vec<Data>,
    tablename: &String,
    mut conn: PooledConn,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    //read from csv the column names
   //execute sql statement below
   let mut querystring:String=String::from("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA='testcsv' AND TABLE_NAME='");
   querystring.push_str(tablename);
   querystring.push_str("'");
    //let columnname = conn.query_map("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA='testcsv' AND TABLE_NAME='Data'", |(COLUMN_NAME)| COLUMN_NAME)?;
    let columnname: Vec<String> = conn.query_map(querystring, |(COLUMN_NAME)| COLUMN_NAME)?;
   //SELECT `COLUMN_NAME`  FROM `INFORMATION_SCHEMA`.`COLUMNS`  WHERE `TABLE_SCHEMA`='testcsv' AND `TABLE_NAME`='Data'; 
   
   let mut insertstatement=String::from("INSERT INTO "); 
   insertstatement.push_str(tablename);
   insertstatement.push_str(" (");
   //Todo combine these two for loops for better efficiency
    for i in &columnname{
         insertstatement.push_str(&i);
         insertstatement.push_str(",");
    }
    insertstatement.pop();
    insertstatement.push_str(") VALUES (");
    for i in &columnname{
        insertstatement.push_str(":");
        insertstatement.push_str(&i);
        insertstatement.push_str(",");
    }
    insertstatement.pop();
    insertstatement.push_str(")");
    println!("{:?}", insertstatement);

    //dynamically insert into table tablename based on number of columns in columname variable

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

    Ok(())
    //todo
}
pub fn read_csv(file: &String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    //fn read_csv() ->Vec<Data> {
    let mut rdr = Reader::from_path(file)?;
    let mut data: Vec<Data> = Vec::new();
    let mut data2 : Vec<InsertData>=Vec::new();
    //iterate through every column in csv file
    let mut rdr2=Reader::from_path(file)?;
    let columnname = rdr.headers()?;
    for column in columnname {
    
        for result in rdr2.records() {
        let record = result?;
        let id = record[0].parse::<String>()?;
        println!("{}", id);

       // data.push(Data {
       //     id,
       //     name,
       //     age,
       //     address,
       //     salary,
       // });
    }
    }
    let tablename= std::env::args().nth(2).expect("No Table");
    let connection = crate::database_connection();
   // execute_insert(&data, &tablename,connection);
    Ok(())
}
