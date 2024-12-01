use std::{error::Error, io};
use serde::Deserialize;

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

fn add_numbers_together(list: Vec<i32>) {
    let sum: i32 = list.iter().sum();
    println!("total sum is: {}", sum);
}

pub fn day_one() {
    if let Ok((list_one, list_two)) = assign_columns_to_vec() {
        let sorted_list_one = sort_vec(list_one);
        let sorted_list_two = sort_vec(list_two);
        let diff_list = create_list_diffs_vec(sorted_list_one, sorted_list_two);
        add_numbers_together(diff_list);
    }
    
}