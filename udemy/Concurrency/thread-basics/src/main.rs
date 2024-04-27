//-------------------------------------
//          Thread Basics
//-------------------------------------


// Concurency:
//  When sections of code in our application runs parallel to other sections of code.
//
//  Also refered to as multithreading

// In most OS and Programs,
// theexecuted code is run in a process
//
// The OS manages multiple processes at once

// This feature that runs these independent parts is called threads



// Concurency AND Parallelism
//
// Concurency:  When multiple tasks which start run and complete 
//              in overlapping time periods, in no specific order.
// (Interesting from SW view)
//
// Parallelism: When multiple tasks or sub-tasks of the same task
//              literally run at the same time,
//              on a HW with multiple computing resources like multi-core processors.
// (Interesting from HW view)



// Threads are often associated with problems such as
//      - Race Conditions,
//          where threasds are accessing data or resources in an inconsistent order.
//      - Deadlocks,
//          where 2 threads are waiting for each other to finish using a resource
//          preventing both threads from continuing.
//      - Bugs,
//          which can only happen in certain situations and are hard to reproduce and fix reliably.


// Rust attempts to mitigate the negative effects of using threads.


use std::thread;
// use std::time::Duration;

fn main() {
    println!("This will be printed");
    println!("This will also be printed");
    println!("The concurrency will start after this line");

    let t = thread::spawn(|| {
        println!("Hello 1 from the thread");
        println!("Hello 2 from the thread");
        println!("Hello 3 from the thread");
        println!("Hello 4 from the thread");
        println!("Hello 5 from the thread");
        println!("Hello 6 from the thread");
        println!("Hello 7 from the thread");
    });
    
    // thread::sleep(Duration::from_nanos(1));

    // Calling join on the thread handler,
    // will halt or block the execution, until the thread is complete
    //
    // This means:
    //      - The thread will complete first
    //      - After that the remaining statements in the main will get a chance to execute
    t.join().expect("Error!!!");

    println!("Hello 1 from the main");
    println!("Hello 2 from the main");

    // If I move the call to join at the end of main function
    //
    // At first the thread and the main will keep on getting chances
    //
    // Then the main thread will wait because of the call to join,
    // and will not end until the spawned thread is finished.
    t.join().expect("Error!!!");

}
