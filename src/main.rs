use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    //recipe has Arc to allow multiple threads to safely own list data space. Mutex to prevent race conditions. Recipe is a vector containing strings. 
    //left in main thread so it can be cloned/accessible by all threads
    let recipe: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    //instantiating an ingredients vector with declared string literals of type &string that are then converted to String objects
    let ingredients = vec![
        "Flour".to_string(),
        "Sugar".to_string(),
        "Eggs".to_string(),
        "Butter".to_string(),
        "Milk".to_string(),
    ];

    // handles vector is declared to store the threads about to be made
    let mut handles = vec![];
    for ingredient in ingredients {
        // Clone the Arc from original recipe vector so new thread can write to it's place in memory
        let recipe_clone = Arc::clone(&recipe);

        // Spawn a new thread, with move command to transfer by value and not reference into it's closure, allowing the value of ingredient[i] and recipe_clone to not be lost if the loop goes faster than the thread
        let handle = thread::spawn(move || {
            println!("Adding {} to recipe...", ingredient);

            // Pause for dramatic effect/to ensure threads have to wait on another for access to the recipe list. Acquires (or blocks depending on unwrap) the mutex lock 
            thread::sleep(Duration::from_millis(50));
            let mut recipe_list = recipe_clone.lock().unwrap();
            recipe_list.push(ingredient.clone());
            println!("Added {} to recipe", ingredient);
            // lock releases here automatically as recipe_list goes out of scope
        });

        handles.push(handle);
    }

    // Checks each thread sequentially to ensure they have finished before moving to print statement
    for handle in handles {
        handle.join().unwrap();
    }

    // Grabs the lock on recipe vector and assigns it to final_recipe to print
    let final_recipe = recipe.lock().unwrap();
    println!("Final Recipe: {:?}", final_recipe);
}