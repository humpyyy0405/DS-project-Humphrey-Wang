// change first time
// this module focus on analysing the data based on the cleaned file and have the capability to 
// count the apprerance of each brand in the states, find n top brands in a curtain state
// calculate the brand probability distribution
// use binary search to get the car based on budget, with the filter of state and bodytype being determined
use std::collections::HashMap; // import hashmap
use crate::data_sort::Used_car; // import the cleaned data
// count the times that each brand appears in each of the states
pub fn brand_numbers(cars: &[Used_car]) -> HashMap<String, HashMap<String, usize>>{ 
//input: reference to the list of Used_car struct, ouput: Hashmap consisting the State name, with inner hashmap consisting the brand name along with its appreance number
    let mut brand_sheet = HashMap::new(); // new sheet
    for car in cars{ //look through each of car Structs inside the cars
        // gets or creating the inner hashmap, if the state exists in brand_sheet, copy its state string, if not create a empty Hashmap for that state
        let state_sheet = brand_sheet.entry(car.state.clone()).or_insert(HashMap::new()); 
        //if the brand exists in the inner loop return a reference to the count or insert 0 if it doesn't exist
        // increment by 1 for each loop
        *state_sheet.entry(car.make.clone()).or_insert(0) += 1; 
    }
    return brand_sheet; // return the outer loop
}
// based on the brand-numbers, do further interpretations to give the top n brand, with the brand name and number
pub fn top_brands(
    sheet: &HashMap<String, HashMap<String, usize>>, //referencing back to the HashMap from the previous function, with state alongside each of its car brand and count
    n: usize, // the top n brands, input as a usize integer
    ) -> HashMap<String, Vec<(String, usize)>> // return a new HashMap, with a inner vector to get the sorted brand apperance
    {
    let mut top_brand = HashMap::new(); // create the new HashMap for storing final results
    for (state, brand_count) in sheet{ // for loop getting each of the state and its brand count
        let mut counts: Vec<(String, usize)> = brand_count.iter().map(|(b, c)| (b.clone(), *c)).collect(); //convert the inner mapping into the vector pair of (brand, count)
        counts.sort_by(|a, b| b.1.cmp(&a.1)); //sort the vector in the descending order, with b.1 before a.1 meaning that the top brand is return first
        let top_n_brand = counts.into_iter().take(n).collect(); //take the top n brand and convert iterator into the vector
        top_brand.insert(state.clone(), top_n_brand); // put the top brands into the HashMap, copy the state name as the key
    }
    return top_brand; // return the HashMap
}
// calculate the probability destribution of each of the brand in curtain states
pub fn brand_prob_distribution(
    cars: &[Used_car], // input: reference to the list of Used_car struct
    state: &str, // the state name abbreviation
    ) -> Vec<(String, f64)>{ // output: A vector of tuples showing the probability among different brands
        let mut total = 0; // inital total counts of the cars
        let mut brand_appearance = HashMap::new(); // HashMap for counting the appearance for each of the brands

        for car in cars{ // for each row in the list
            if car.state == state{ 
                // if the car make exists in brnad_appearance, copy its make name and return the reference back to the list
                // if not insert 0
                // increment by 1
                *brand_appearance.entry(car.make.clone()).or_insert(0) += 1; 
                total += 1; // total count increment
            }
        }
            let mut brand_probability: Vec<(String, f64)> = brand_appearance
            .into_iter() // convert the Hashmap into the vector
            .map(|(brand, count)|(brand, count as f64 / total as f64)) // each brand's count is devided by total count to get probability
            .collect(); 
            brand_probability.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap()); // sorted on the order from highest probability to lowest
            return brand_probability;
}
// give recommendations on the top n cars suited based on the budget from a given state and bodytype, use binary research to compare prices
pub fn car_recommendation(
    cars: &[Used_car], // input: reference to the list of Used_car struct
    state: &str, // the state name abbreviation
    bodytype: &str, // chosen bodytype
    budget: f64, // the money input
    n:usize // top n choices
) -> Vec<Used_car>{ // a list of n good choices based on the conditions
    let mut filtered_cars: Vec<Used_car> = cars.iter()
    .filter(|c|{ // filter to keep only the state and bodytype that match the condition
        c.state == state 
        && c.bodytype == bodytype
    }).cloned().collect(); // own the Used_car data and store into the new filtered_cars vector

    filtered_cars.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap()); // sort with ascending order in price in order to create the structure for binary search
    // perform the binary search for finding the car cloest to the budget. If found, return its index, if not this would be the cloest index 
    let car_index = match filtered_cars.binary_search_by(|c| c.price.partial_cmp(&budget).unwrap()){
        Ok(i) => i,
        Err(i) => i,
    };
    //initalize the two pointers, left starts the index before the cloest index and right starts on the cloest index
    let mut result = Vec::new();
    let mut left = None;
    if car_index > 0{
        left = Some(car_index - 1);
    }
    let mut right = Some(car_index);
    // while loop for collecting the results until n recommendations are found or there's not any cars to check
    while result.len() < n && (left != None || right != None){
        // two if let to compare the difference and see if the left car or right car is closer in price to the budget
        if let Some(l) = left{ // if left pointer is still valid
            if let Some(r) = right{ // same as above but right pointer
                let left_difference = (filtered_cars[l].price - budget).abs(); // compute the absolute value among difference between left pointer and the budget
                let right_difference = (filtered_cars[r].price - budget).abs(); // same but right pointer
                // if else statement between coparision in the left and right difference in order to push the closer car to the result list
                // move the pointer either towards the left or right 1 more based on the outcome
                if left_difference <= right_difference{  // if left difference is smaller
                    result.push(filtered_cars[l].clone()); //push a clone of the left car into the results list
                    // move the left pointer 1 index to the left 
                    if l > 0{
                        left = Some(l - 1); 
                    }
                    // if there's no more left pointer, set the left pointer as None
                    else{
                        left = None;
                    }
                }
                // Same approach just in right pointer, every logic is the same though
                else{
                    result.push(filtered_cars[r].clone());
                    if r + 1 < filtered_cars.len(){
                        right = Some(r + 1);
                    }
                    else{
                        right = None;
                    }
                }
            }
            // if there's no more right pointer left, we just take from the left as we only have left pointer, save approach below
            else{
                result.push(filtered_cars[l].clone());
                if l > 0{
                    left = Some(l - 1);
                }
                else{
                    left = None;
                }
            }
        
         } 
         // if there's no more left pointer left and we still have the right pointer we take it from the right
         else if let Some(r) = right{ 
            result.push(filtered_cars[r].clone());
            if r + 1 < filtered_cars.len(){
                right = Some(r + 1);
            }
            else{
                right = None;
                }
        }
    
}
    return result; // return the n best recommendations
}


#[test]
// testing function for the brand_numbers
// 2 Honda and 1 Dodge
fn brand_numbers_test(){
    let test_cars = vec![
        Used_car{
            make: "Dodge".to_string(),
            state: "CA".to_string(),
            price: 32000.0,
            bodytype: "Coupe".to_string(),
            drivetype: "RWD".to_string(),
        },
        Used_car{
            make: "Honda".to_string(),
            state: "MA".to_string(),
            price: 25000.0,
            bodytype: "SUV".to_string(),
            drivetype: "FWD".to_string(),
        },
        Used_car{
            make: "Honda".to_string(),
            state: "MA".to_string(),
            price: 23000.0,
            bodytype: "SUV".to_string(),
            drivetype: "AWD".to_string(),
        },
    ];
    let result = brand_numbers(&test_cars);
    assert_eq!(result["MA"]["Honda"], 2);
    assert_eq!(result["CA"]["Dodge"], 1);
}

// I didn't test top brands as I think it can be kinda determined from the probability distribution function

#[test]
// testing functions for the brand_prob_distribution
// 3 out of 4 test_cars are honda, so it should be the favorate brand with probability of 0.75
fn brand_prob_distribution_test(){
    let test_cars = vec![
        Used_car{
            make: "Dodge".to_string(),
            state: "MA".to_string(),
            price: 32000.0,
            bodytype: "Coupe".to_string(),
            drivetype: "RWD".to_string(),
        },
        Used_car{
            make: "Honda".to_string(),
            state: "MA".to_string(),
            price: 25000.0,
            bodytype: "SUV".to_string(),
            drivetype: "FWD".to_string(),
        },
        Used_car{
            make: "Honda".to_string(),
            state: "MA".to_string(),
            price: 23000.0,
            bodytype: "SUV".to_string(),
            drivetype: "AWD".to_string(),
        },
        Used_car{
            make: "Honda".to_string(),
            state: "MA".to_string(),
            price: 33000.0,
            bodytype: "Coupe".to_string(),
            drivetype: "AWD".to_string(),
        },
    ];
    let result = brand_prob_distribution(&test_cars, "MA");
    assert_eq!(result[0].0, "Honda");
    assert_eq!(result[0].1, 0.75);
}

#[test]
// testing functions for the car_recommendation
// for the binary search, the 28000 is located between the 30k dodge and 25k Honda, but 30-28 = 2 < 28-25 = 3, 
// the first recommendation will be pointed to the right, then 25k honda would be the second recommendation
fn car_recommendation_test(){
    let test_cars = vec![
        Used_car{
            make: "Dodge".to_string(),
            state: "MA".to_string(),
            price: 30000.0,
            bodytype: "SUV".to_string(),
            drivetype: "RWD".to_string(),
        },
        Used_car{
            make: "Honda".to_string(),
            state: "MA".to_string(),
            price: 25000.0,
            bodytype: "SUV".to_string(),
            drivetype: "FWD".to_string(),
        },
        Used_car{
            make: "Kia".to_string(),
            state: "MA".to_string(),
            price: 23000.0,
            bodytype: "SUV".to_string(),
            drivetype: "AWD".to_string(),
        },
    ];
    let recommendations = car_recommendation(&test_cars, "MA", "SUV", 28000.0, 2);
    assert_eq!(recommendations[0].make, "Dodge");
    assert_eq!(recommendations[1].make, "Honda");
}