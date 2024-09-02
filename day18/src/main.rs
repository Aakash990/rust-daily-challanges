/* Compare and update the inventory stored in a 2D array against a second 2D array of a fresh delivery. Update the current existing inventory item quantities (in arr1). If an item cannot be found, add the new item and quantity into the inventory array. The returned inventory array should be in alphabetical order by item. */

use std::collections::HashMap;

fn update_inventory(arr1: Vec<(i32, &str)>, arr2: Vec<(i32, &str)>) -> Vec<(i32, String)> {
    // creating a HashMap from arr1
    let mut inventory_map: HashMap<&str, i32> = HashMap::new();

    // populating the HashMap with the initial inventory 
    for (quantity, item) in arr1 {
        *inventory_map.entry(item).or_insert(0) += quantity;
    }

    // updating the HashMap with new delivery
    for (quantity, item) in arr2 {
        *inventory_map.entry(item).or_insert(0) += quantity;
    }
    
    // converting HashMap to a Vec and sorting alphabetically by item name

    let mut inventory_vec: Vec<(i32, String)> = inventory_map
        .into_iter()
        .map(|(item, quantity)| (quantity, item.to_string()))
        .collect();

    inventory_vec.sort_by(|a, b| a.1.cmp(&b.1));
    
    inventory_vec
}

fn main() {
    // Example inventory lists
    let cur_inv = vec![
        (21, "Bowling Ball"),
        (2, "Dirty Sock"),
        (1, "Hair Pin"),
        (5, "Microphone")
    ];
    
    let new_inv = vec![
        (2, "Hair Pin"),
        (3, "Half-Eaten Apple"),
        (67, "Bowling Apple"),
        (7, "Toothpaste")
    ];

    let updated_inventory = update_inventory(cur_inv, new_inv);
    
    println!("{:?}", updated_inventory);
    //output: [(67, "Bowling Apple"), (21, "Bowling Ball"), (2, "Dirty Sock"), (3, "Hair Pin"), (3, "Half-Eaten Apple"), (5, "Microphone"), (7, "Toothpaste")]
}
