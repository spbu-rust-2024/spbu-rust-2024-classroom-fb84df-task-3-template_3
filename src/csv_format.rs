use csv::StringRecord;
use csv::WriterBuilder;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    if !file_path.ends_with(".csv") {
        return Err("Type of file must be .csv to read it!".into());
    }
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false) // No headers in the file
        .delimiter(b';')
        .from_reader(file);

    for result in rdr.records() {
        let record: StringRecord = result?;
        let mut row_map = HashMap::new();

        for (index, value) in record.iter().enumerate() {
            row_map.insert(index.to_string(), value.to_string());
        }

        println!("{:?}", row_map);
    }

    Ok(())
}

pub fn write_csv<T>(file_path: &str, info: T) -> Result<(), Box<dyn Error>>
where
    T: IntoIterator,
    T::Item: Serialize,
{
    if !file_path.ends_with(".csv") {
        return Err("Type of file must be .csv to write in!".into());
    }
    let file = File::create(file_path)?;
    let mut wtr = WriterBuilder::new().delimiter(b';').from_writer(file);

    for item in info {
        wtr.serialize(item)?;
    }

    wtr.flush()?;
    Ok(())
}
