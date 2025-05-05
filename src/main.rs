// this is the main module in order to implement all of the methods defined in other modules and run based on the input file to shown on the terminal
mod data_sort; //import the two rs modules we created before
mod analysis;
// bring the function we wrote in the modules
use data_sort::load_csv; 
use analysis::{top_brands, brand_numbers, brand_prob_distribution, car_recommendation};


fn main() {
    let cars = load_csv("used_car_sales 2.csv"); // loaded the dataset and the load_csv cleaned it
    println!("{} valid used car records being loaded.\n", cars.len()); // get the numbers of the valid data rows 
    // below is a for loop for printing the first 5 datarows inside the file with our designed format output
    for (i, car) in cars.iter().take(5).enumerate() {
        println!(
            "Car {}: Make: {:<12} | State: {:<2} | Price: ${:<8.2} | Bodytype: {:<14} | Drivetype: {}",
            i + 1,
            car.make,
            car.state,
            car.price,
            car.bodytype,
            car.drivetype,

        );
    }
    let sheet = brand_numbers(&cars); // count the number of times every brand appears in every state
    let top3 = top_brands(&sheet, 3); // get the top3 brand for each of the states
    // sorted based on the alphabetical order of the statename (key), do it in main as we return the HashMap
    let mut states: Vec<_> = top3.keys().collect(); 
    states.sort();
    println!("\n Top 3 brands in each state:");
    for state in states { // each of the states
        if let Some(brands) = top3.get(state){ // if the state exists in the top 3 brand, assign it to the brand with its value
            print!("{}: ", state); 
            // use for loop to get the ranking number and the brandname + count
            for (i, (brand, count)) in brands.iter().enumerate() {
                print!("{}. {} ({})  ", i + 1, brand, count);
            }
        println!();
        }
    }
    // implementation of the brand_prob_distribution funciton
    let target_state = "WY"; 
    let probability = brand_prob_distribution(&cars, target_state);
    println!("\n Brand probability distribution in {}", target_state); // calculate how popular each brand in WY as percentage
    // output brand probabilities with the format of brand + probability, keep 2 desmo places
    for (brand, prob) in probability {
        // println!("{:.2}%", prob * 100.0); i comment these out because I used this initially to get the data values and use it to draw graphs in spreadsheet
        // println!("{}", brand);
        println!("{:<15}| {:.2}%", brand, prob * 100.0);
    }
    // implementation for the binary search method
    // my conditions: California, 30 grand and I need a SUV
    let budget = 30000.0;
    let target_state = "CA";
    let car_recommendations = car_recommendation(&cars, target_state, "SUV", budget, 5);
    println!("\n The top 5 cars based on your budget around: ${} in {}", budget, target_state);
    // for loop to display the top 5 recommendations with the format we have designed initially
    for (i, car) in car_recommendations.iter().enumerate(){
        println!(
            "Car {}: Make: {:<15} | State: {:<2} | Price: ${:<8.2} | Bodytype: {:<14} | Drivetype: {}",
            i + 1,
            car.make,
            car.state,
            car.price,
            car.bodytype,
            car.drivetype,

        );
    }
}

    
