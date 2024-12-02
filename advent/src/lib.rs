use std::{collections::HashMap, error::Error, io};
use serde::Deserialize;

// Day One Stuff
#[derive(Debug, Deserialize)]
struct Record {
    column1: i32,
    column2: i32,
}

fn assign_columns_to_vec() -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        list_one.push(record.column1);
        list_two.push(record.column2);
    }
    Ok((list_one, list_two))
}


fn sort_vec(list: Vec<i32>) -> Vec<i32> {
    let mut ints = list;
    ints.sort();
    return ints;
}

fn create_list_diffs_vec(list_one: Vec<i32>, list_two: Vec<i32>) -> Vec<i32> {
    let mut distance_list: Vec<i32> = vec![];
    for (i, el) in list_one.iter().enumerate() {
        let list_two_equivalent = list_two[i];
        let mut distance: i32 = 0;
        if el < &list_two_equivalent {
            distance = list_two_equivalent - el;
        } else if el > &list_two_equivalent {
            distance = el - list_two_equivalent;
        }
        distance_list.push(distance);
    }
    return distance_list;
}


fn create_similarity_score_list(list_one: Vec<i32>, list_two: Vec<i32>) -> HashMap<i32, i32> {
    let mut similarity_map: HashMap<i32, i32> = HashMap::new();
    for (i, el) in list_one.iter().enumerate() {
        let list_one_value = *el;
            println!("Will try to find a match for: {}", el);
            let mut list_two_value = list_two[i];
            let mut current_index = i;
            // Matches Dead on (rare)
            if list_one_value == list_two_value {
                similarity_map.entry(list_one_value).and_modify(|val| *val += 1).or_insert(1);
                let mut temp_index_backwards = current_index;
                let mut temp_index_forwards = current_index;
                let mut still_searching_backwards = true;
                let mut still_searching_forwards = true;
                // Flow backwards
                while temp_index_backwards > 0 && still_searching_backwards {
                    temp_index_backwards = temp_index_backwards - 1;
                    if list_two[temp_index_backwards] == list_one_value {
                        similarity_map.entry(list_one_value).and_modify(|val| *val += 1);
                    } else {
                        still_searching_backwards = false;
                    }
                }
                while temp_index_forwards > 0 && still_searching_forwards {
                    temp_index_forwards = temp_index_forwards + 1;
                    if list_two[temp_index_forwards] == list_one_value {
                        similarity_map.entry(list_one_value).and_modify(|val| *val += 1);
                    } else {
                        still_searching_forwards = false;
                    }
                }
            } else if list_one_value < list_two_value {
                while list_one_value <= list_two_value {
                    println!("Left {} is less than right {}", list_one_value, list_two_value);
                    if current_index > 0 {
                        current_index = current_index - 1;
                        println!("current_index: {}", current_index);
                        list_two_value = list_two[current_index];
                        if list_one_value == list_two_value {
                            println!("Found a match on left {} right {} after moving backwards", list_one_value, list_two_value);
                            similarity_map.entry(list_one_value).and_modify(|val| *val += 1).or_insert(1);
                        }
                    } else {
                        break;
                    }                    
                }
            } else if list_one_value > list_two_value {
                while list_one_value >= list_two_value {
                    if current_index < list_two.len() - 1 {
                        current_index = current_index + 1;
                        list_two_value = list_two[current_index];
                        if list_one_value == list_two_value {
                            println!("Found a match on left {} right {} after moving forwards", list_one_value, list_two_value);
                            similarity_map.entry(list_one_value).and_modify(|val| *val += 1).or_insert(1);
                        }    
                    } else {
                        break;
                    }
                }
            }
    }
    return similarity_map;
}

fn multiple_and_formulate_similarity_score_list(map: HashMap<i32, i32>) -> Vec<i32> {
    let mut score_list: Vec<i32> = vec![];
    println!("!!!!!!! multiple_and_formulate_similarity_score_list");
    for (value, occurances) in map {
        println!("value from left: {}", value);
        println!("total occurances on right: {}", occurances);
        score_list.push(value * occurances);
    }
    println!("{:?}", score_list);
    return score_list;
}

fn add_numbers_together(list: Vec<i32>) {
    let sum: i32 = list.iter().sum();
    println!("total sum is: {}", sum);
}

pub fn day_one_part_one() {
    if let Ok((list_one, list_two)) = assign_columns_to_vec() {
        let sorted_list_one = sort_vec(list_one);
        let sorted_list_two = sort_vec(list_two);
        let diff_list = create_list_diffs_vec(sorted_list_one, sorted_list_two);
        add_numbers_together(diff_list);
    }
}

pub fn day_one_part_two() {
    if let Ok((list_one, list_two)) = assign_columns_to_vec() {
        let sorted_list_one = sort_vec(list_one);
        let sorted_list_two = sort_vec(list_two);
        let similarity_map = create_similarity_score_list(sorted_list_one, sorted_list_two);
        let similarity_score_list = multiple_and_formulate_similarity_score_list(similarity_map);
        add_numbers_together(similarity_score_list);
    }
}