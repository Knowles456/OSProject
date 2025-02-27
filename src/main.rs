use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // Create two shared resources
    let recipe: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let extra_ingredients: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    //PART A PHASE 3 (Deadlock Scenario):
    println!("\n=== DEADLOCK SCENARIO ===");

    // Thread 1 will attempt to lock in the order of recipe then extras. CLone to give thread shared ownership
    let recipe1 = Arc::clone(&recipe);
    let extra1 = Arc::clone(&extra_ingredients);
    let thread1 = thread::spawn(move || {
        // First get recipe lock
        println!("Thread 1: Attempting to get recipe lock");
        let mut  recipe_list = recipe1.lock().unwrap();
        println!("Thread 1: Got recipe lock");

        recipe_list.push("Captain Crunch".to_string());
        println!("Thread 1: Added Captain Crunch to recipe");

        // Timer that ensures thread 2 has already grabbed the extra ingredients lock before thread 1, ensuring deadlock
        thread::sleep(Duration::from_millis(500));

        // Try to get extra ingredients while still holding recipe lock
        println!("Thread 1: Attempting to get extra ingredients lock");
        // Match the response of the try_lock function to see if deadlock occurs or
        match extra1.try_lock() {
            //happens if there is no deadlock
            Ok(mut list) => {
                list.push("Vanilla".to_string());
                println!("Thread 1: Added Vanilla to extras");
            },
            Err(_) => {
                println!("Thread 1: DEADLOCK DETECTED - Could not get extra ingredients lock");
                // Potential deadlock detected
            }
        }
    });

    // Thread 2 will attempt to lock in the order of extras then recipe
    let recipe2 = Arc::clone(&recipe);
    let extra2 = Arc::clone(&extra_ingredients);
    let thread2 = thread::spawn(move || {
        // Get extra ingredients lock first
        println!("Thread 2: Attempting to get extra ingredients lock");
        let mut grab_extra = extra2.lock().unwrap();
        println!("Thread 2: Got extra ingredients lock");

        grab_extra.push("Salt".to_string());
        println!("Thread 2: Added Salt to extras");

        thread::sleep(Duration::from_millis(500));

        // Try to get recipe while still holding extra ingredients lock
        println!("Thread 2: Trying to lock recipe (while holding extra ingredients lock)");
        // This will deadlock if Thread 1 has the recipe lock
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
    println!("\n=== NORMAL SCENARIO (Deadlock Resolution) ===");

    // Thread 3
    let recipe3 = Arc::clone(&recipe);
    let extra3 = Arc::clone(&extra_ingredients);
    let thread3 = thread::spawn(move || {
        // First get recipe lock with timeout
        println!("Thread 3: Attempting to get recipe lock");
        let recipe_list_result = try_lock_for_duration(&recipe3, Duration::from_millis(50));
        let mut recipe_list = match recipe_list_result {
            Ok(guard) => {
                println!("Thread 3: Got recipe lock");
                guard
            },
            // Exit thread on timeout
            Err(_) => {
                println!("Thread 3: TIMEOUT - Could not get recipe lock");
                return; 
            }
        };

        recipe_list.push("Captain Crunch".to_string());
        println!("Thread 3: Added Captain Crunch to recipe");

        // Wait a bit to encourage concurrent operations are occurring
        thread::sleep(Duration::from_millis(100));

        // Very explicitly release first lock before getting second
        println!("Thread 3: Releasing recipe lock");
        drop(recipe_list);

        // Now get extra ingredients lock with timeout
        println!("Thread 3: Attempting to lock extra ingredients");
        let extra_list_result = try_lock_for_duration(&extra3, Duration::from_millis(50));
        let mut extra_list = match extra_list_result {
            Ok(guard) => {
                println!("Thread 3: Got extra ingredients lock");
                guard
            },
            // Exit thread on timeout
            Err(_) => {
                println!("Thread 3: TIMEOUT - Could not get extra ingredients lock");
                return;
            }
        };

        extra_list.push("Pinch of salt".to_string());
        println!("Thread 3: Added Pinch of salt to extras");
    });

    // Thread 4. Adds ingredients to vectors in same sequence as Thread 3
    let recipe4 = Arc::clone(&recipe);
    let extra4 = Arc::clone(&extra_ingredients);
    let thread4 = thread::spawn(move || {
        // Wait to ensure the first thread gets its lock
        thread::sleep(Duration::from_millis(50));

        // Get extra ingredients lock to begin with with timeout
        println!("Thread 4: Trying to lock extra ingredients");
        let extra_list_result = try_lock_for_duration(&extra4, Duration::from_millis(50));
        let mut extra_list = match extra_list_result {
            Ok(guard) => {
                println!("Thread 4: Got extra ingredients lock");
                guard
            },
            // Exit thread on timeout
            Err(_) => {
                println!("Thread 4: TIMEOUT - Could not get extra ingredients lock");
                return;
            }
        };

        extra_list.push("Pinch of sugar".to_string());
        println!("Thread 4: Added Pinch of sugar to extras");

        // Wait a bit to encourage concurrent operations are occurring
        thread::sleep(Duration::from_millis(100));

        // Release first lock before getting second
        println!("Thread 4: Releasing extra ingredients lock");
        drop(extra_list);

        // Now get recipe lock with timeout
        println!("Thread 4: Attempting to lock recipe");
        let recipe_list_result = try_lock_for_duration(&recipe4, Duration::from_millis(50));
        let mut recipe_list = match recipe_list_result {
            Ok(guard) => {
                println!("Thread 4: Got recipe lock");
                guard
            },
            // Exit thread on timeout
            Err(_) => {
                println!("Thread 4: TIMEOUT - Could not get recipe lock");
                return; 
            }
        };

        recipe_list.push("Sugar".to_string());
        println!("Thread 4: Added Sugar to recipe");
    });

    // Wait for both threads to finish
    thread3.join().unwrap();
    thread4.join().unwrap();

    // Print results
    println!("\nRecipe: {:?}", recipe.lock().unwrap());
    println!("Extras: {:?}", extra_ingredients.lock().unwrap());

}

// Timeout function to prevent deadlock. Loops indefinitely until lock is acquired or timeout occurs. 'a included to ensure the mutex guard is valid for the lifetime of the mutex
// Returns ok from try_lock containing mutex guard or Err if timeout occurs
fn try_lock_for_duration<'a>(mutex: &'a Arc<Mutex<Vec<String>>>, timeout: Duration) -> Result<std::sync::MutexGuard<'a, Vec<String>>, ()> {
    let start_time = Instant::now();
    loop {
        match mutex.try_lock() {
            //got the go ahead on lock
            Ok(guard) => return Ok(guard),
            //timeout occurs
            Err(_) => {
                if start_time.elapsed() >= timeout {
                    return Err(()); 
                }
                // Short sleep before trying again/looping
                thread::sleep(Duration::from_millis(5)); 
            }
        }
    }
}