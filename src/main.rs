use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
  
fn main() {
    // Create two shared resources
    let recipe: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let extra_ingredients: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    //PART A PHASE 3 (Deadlock Scenario):
    println!("\n=== DEADLOCK SCENARIO ===");
    
    // Thread 1 will attempt to lock in the order of recipe then extras
    let recipe1 = Arc::clone(&recipe);
    let extra1 = Arc::clone(&extra_ingredients);
    let thread1 = thread::spawn(move || {
        // First get recipe lock
        println!("Thread 1: Attempting to get recipe lock");
        let mut  recipe_list = recipe1.lock().unwrap();
        println!("Thread 1: Got recipe lock");
        
        recipe_list.push("Captain Crunch".to_string());
        // Timer that ensures thread 2 has already grabbed the extra ingredients lock before thread 1, ensuring deadlock
        thread::sleep(Duration::from_millis(100));
        
        // Try to get extra ingredients while still holding recipe lock
        println!("Thread 1: Trying to lock extra ingredients (while holding recipe lock)");
        // This will deadlock if Thread 2 has the extra ingredients lock
        match extra1.try_lock() {
            //happens if there is no deadlock
            Ok(mut list) => {
                list.push("Vanilla".to_string());
                println!("Thread 1: Added Vanilla to extras");
            },
            Err(_) => {
                println!("Thread 1: DEADLOCK DETECTED - Could not get extra ingredients lock");
                // Potential deadlock detected by using try_lock instead of lock
            }
        }
    });
    
    // Thread 2 will attempt to lock in the order of extras then recipe
    let recipe2 = Arc::clone(&recipe);
    let extra2 = Arc::clone(&extra_ingredients);
    let thread2 = thread::spawn(move || {
        // Get extra ingredients lock first
        println!("Thread 2: Trying to lock extra ingredients");
        let _grab_extra = extra2.lock().unwrap();
        println!("Thread 2: Got extra ingredients lock");
        
        // Don't release extra lock! This will cause deadlock
        thread::sleep(Duration::from_millis(100));
        
        // Try to get recipe while still holding extra ingredients lock
        println!("Thread 2: Trying to lock recipe (while holding extra ingredients lock)");
        // This will deadlock if Thread 3 has the recipe lock
        match recipe2.try_lock() {
            Ok(mut list) => {
                list.push("Baking Powder".to_string());
                println!("Thread 2: Added Baking Powder to recipe");
                // No deadlock occurred
            },
            Err(_) => {
                println!("Thread 2: DEADLOCK DETECTED - Could not get recipe lock");
                // We detected potential deadlock by using try_lock instead of lock
            }
        }
    });
    
    // Wait for both threads to finish
    thread1.join().unwrap();
    thread2.join().unwrap();
    
    println!("\nFinal state after deadlock scenario:");
    println!("Recipe: {:?}", recipe.lock().unwrap());
    println!("Extras: {:?}", extra_ingredients.lock().unwrap());

    // Clear the collections for the next scenario
    recipe.lock().unwrap().clear();
    extra_ingredients.lock().unwrap().clear();
    
    //PART A PHASE 4 (Deadlock Resolution):
    println!("=== NORMAL SCENARIO (Deadlock Resolution) ===");
    
    // Thread 3
    let recipe3 = Arc::clone(&recipe);
    let extra3 = Arc::clone(&extra_ingredients);
    let thread3 = thread::spawn(move || {
        // First get recipe lock
        println!("Thread 3: Attempting to lock recipe");
        let mut recipe_list = recipe3.lock().unwrap();
        println!("Thread 3: Received recipe lock");
        
        recipe_list.push("Captain Crunch".to_string());
        println!("Thread 3: Added Captain Crunch to recipe");
        
        // Wait a bit to encourage concurrent operations are occurring
        thread::sleep(Duration::from_millis(100));
        
        // Release first lock before getting second
        println!("Thread 3: Releasing recipe lock");
        drop(recipe_list);
        
        // Now get extra ingredients lock
        println!("Thread 3: Attempting to lock extra ingredients");
        let mut extra_list = extra3.lock().unwrap();
        println!("Thread 3: Got extra ingredients lock");
        
        extra_list.push("Pinch of salt".to_string());
        println!("Thread 3: Added pinch of salt to extras");
    });
    
    // Second thread which will add different ingredients to both vectors 
    let recipe4 = Arc::clone(&recipe);
    let extra4 = Arc::clone(&extra_ingredients);
    let thread4 = thread::spawn(move || {
        // Wait to ensure the frist thread gets its lock
        thread::sleep(Duration::from_millis(50));
        
        // Get extra ingredients lock to begin with
        println!("Thread 4: Trying to lock extra ingredients");
        let mut extra_list = extra4.lock().unwrap();
        println!("Thread 4: Got extra ingredients lock");
        
        extra_list.push("Pinch of sugar".to_string());
        println!("Thread 4: Added pinch of sugar to extras");
        
        // Wait a bit to encourage concurrent operations are occurring
        thread::sleep(Duration::from_millis(100));
        
        // Release first lock before getting second
        println!("Thread 4: Releasing extra ingredients lock");
        drop(extra_list);
        
        // Now get recipe lock
        println!("Thread 4: Trying to lock recipe");
        let mut recipe_list = recipe4.lock().unwrap();
        println!("Thread 4: Got recipe lock");
        
        recipe_list.push("Sugar".to_string());
        println!("Thread 4: Added Sugar to recipe");
    });
    
    // Wait for both threads to finish
    thread3.join().unwrap();
    thread4.join().unwrap();
    
    // Print results
    println!("Recipe: {:?}", recipe.lock().unwrap());
    println!("Extras: {:?}", extra_ingredients.lock().unwrap());
    
}
