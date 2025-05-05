// this module is opening and sorting data
// with functions involving reading the car data from the csv file, cleaning + filtering the data by using sorting and transforming the zip code of each car being sold
// into the state where the zip code is located for better usage further in the analysis section
// a list of Used_car struct is returned for the later analysis
use std::collections::HashMap; // import Hashmap
use csv; //import CSV from dependencies
#[derive(Clone)] // implementing the trait of clone in order to make copy of the Used_car with .clone()

// getting the useful column from the CSV file (important info of the used car)
pub struct Used_car{ 
    pub make: String, //car brand
    pub state: String, // state where car is being sold, need conversion
    pub price: f64, //Price is important for budget
    pub bodytype: String,  // bodytype is a big demand, some ppl like sporty cars, others prefer SUVs
    pub drivetype: String, //drivetype to represent the mode of drive configuration
}
// converting the zip code into the states in US
fn zip_conversion(zip: i32) -> Option<String> { // input as a zip code, output as Option<T>, two ways, either Some(state) or None if zipcode invalid
    match zip { // match the zipcode range into the State name
        99501..=99950 => Some("AK".to_string()), // the zipcode range of AK, change "AK" from &str to String for matching inside the struct
        35004..=36925 => Some("AL".to_string()), // used AI to help me do the rest as it's all the same, I give it the chart of the zipcode
        71601..=72959 | 75502 => Some("AR".to_string()),
        85001..=86556 => Some("AZ".to_string()),
        90001..=96162 => Some("CA".to_string()),
        80001..=81658 => Some("CO".to_string()),
        6001..=6389 | 6401..=6928 => Some("CT".to_string()),
        20001..=20039 | 20042..=20599 | 20799 => Some("DC".to_string()),
        19701..=19980 => Some("DE".to_string()),
        32004..=34997 => Some("FL".to_string()),
        30001..=31999 | 39901 => Some("GA".to_string()),
        96701..=96898 => Some("HI".to_string()),
        83201..=83876 => Some("ID".to_string()),
        60001..=62999 => Some("IL".to_string()),
        46001..=47997 => Some("IN".to_string()),
        50001..=52809 | 68119..=68120 => Some("IA".to_string()),
        66002..=67954 => Some("KS".to_string()),
        40003..=42788 => Some("KY".to_string()),
        70001..=71232 | 71234..=71497 => Some("LA".to_string()),
        3901..=4992 => Some("ME".to_string()),
        20331 | 20335..=20797 | 20812..=21930 => Some("MD".to_string()),
        1001..=2791 | 5501..=5544 => Some("MA".to_string()),
        48001..=49971 => Some("MI".to_string()),
        55001..=56763 => Some("MN".to_string()),
        38601..=39776 | 71233 => Some("MS".to_string()),
        63001..=65899 => Some("MO".to_string()),
        59001..=59937 => Some("MT".to_string()),
        27006..=28909 => Some("NC".to_string()),
        58001..=58856 => Some("ND".to_string()),
        68001..=68118 | 68122..=69367 => Some("NE".to_string()),
        3031..=3897 => Some("NH".to_string()),
        7001..=8989 => Some("NJ".to_string()),
        87001..=88441 => Some("NM".to_string()),
        88901..=89883 => Some("NV".to_string()),
        6390 | 10001..=14975 => Some("NY".to_string()),
        43001..=45999 => Some("OH".to_string()),
        73001..=73199 | 73401..=74966 => Some("OK".to_string()),
        97001..=97920 => Some("OR".to_string()),
        15001..=19640 => Some("PA".to_string()),
        0 => Some("PR".to_string()),
        2801..=2940 => Some("RI".to_string()),
        29001..=29948 => Some("SC".to_string()),
        57001..=57799 => Some("SD".to_string()),
        37010..=38589 => Some("TN".to_string()),
        73301 | 75001..=75501 | 75503..=79999 | 88510..=88589 => Some("TX".to_string()),
        84001..=84784 => Some("UT".to_string()),
        20040..=20042 | 20167 | 22001..=24658 => Some("VA".to_string()),
        5001..=5495 | 5601..=5907 => Some("VT".to_string()),
        98001..=99403 => Some("WA".to_string()),
        24701..=26886 => Some("WV".to_string()),
        53001..=54990 => Some("WI".to_string()),
        82001..=83128 => Some("WY".to_string()),
        _ => None, // if the numbers is not one of the valid zip codes in the states, return None
    }
}
// read the CSV file, filters the data and return the list consisting of the valid used car entry
pub fn load_csv(path: &str) -> Vec<Used_car>{ //input: string slice for the pathway, output: a vector of Used_car structs
    let mut data_reader = csv::Reader::from_path(path).expect("failed to read csv file"); // create csv reader based on the input path
    let mut used_cars = Vec::new(); // create new vector for storing the used cars
    // for loop to go over each of the rows (one used car sales record) inside the CSV file, consisting several steps inside as we have to sort
    //several columns to mkae sure whether the data is valid
    for results in data_reader.records(){
        let car_info = results.expect("Invalid car info"); //unwrap the results and panic if it failed to load
        let zip_code_str = &car_info[3]; // get the zip code (index 3) inside the CSV file
        // this function is part of the data cleaning process, as the info might be missing or not containing complete zip code, we delete
        //them during this stage in order to avoid errors, invalid zipcode includes sth like "021**", "EMPTY"
        if !zip_code_str.chars().all(|c| c.is_ascii_digit()) {
            continue; 
        } 
        // converting the zip code string into the actual number, does not return if it fails to convert
        let zip_code: i32 = match zip_code_str.parse() {
            Ok(z) => z,
            Err(_) => continue,
        };
        // converting the zip code numbers into the String states, does not return if it fails to convert (missing info)
        let state = match zip_conversion(zip_code) {
            Some(s) => s,
            None => continue,
        };
        // get the price of the car (index 1) (str at the moment) and convert it into the floating point, does not return if it fails to convert (missing price)
        let price: f64 = match car_info[1].parse() {
            Ok(p) => p,
            Err(_) => continue,
        };
        let brand_make = car_info[5].to_string(); //get the brand name of the data and convert into a string, valid all the times though
        // cleaning drivetypes
        let valid_drives = ["FWD", "RWD", "AWD", "4WD"]; //valid drivetypes
        let drive_raw = car_info[12].trim(); // get the inital drivetype and trim any spaces
        if !valid_drives.contains(&drive_raw) {
            continue; // if it doesn't contain the valid drivetypes we skip this row
        }
        let drivetype = drive_raw.to_string(); // get the drivetypes and convert into a string

        let bodytype_raw = car_info[10].trim(); // get the orgiinal bodytype and trip any spaces
        if bodytype_raw.is_empty(){
            continue; // skip the line if it does not contain the specific bodytype
        }
        let bodytype = bodytype_raw.to_string(); // get the bodytype and convert into a string
        used_cars.push(Used_car{ //push the data as a Used_car struct and add it to the vector
            make: brand_make,
            state,
            price,
            bodytype,
            drivetype,
        });
    }
    return used_cars; // return the vector and the cleaned values
}