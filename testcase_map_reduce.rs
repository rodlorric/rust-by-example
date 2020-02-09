use std::thread;

// This is the `main` thread.
fn main() {
    // This is our data to process.
    // We will calculate the sum of all digits via a thread map-reduce algorithm.
    // Each whitespace separated chunk will be handled in a different thread.
    // 
    // TODO: see what happens to the output if you insert spaces!
    let data = "86967897737416471853297327050364959
    1186132257556        4 7 2 3 9 6 3297542624962850
    70856234701860851907960690014725639
    38397966707106 094172783238747669219
    52380795257888236525459303330302837
    584953271357440 41048897885734297812
    6992021643898087      3548808413720956532
    16278424637452589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    /**************************************************************************
     * "Map" phase
     *
     * Divide our data into segments, and apply initial processing
     *************************************************************************/

    // Split our data into segments for individual calculation,
    // each chunk will be a reference (&str) into the actual data
    let chunked_data = data.split_whitespace();

    // Iterate over the data segments.
    // .enumerate() adds the current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is the immediately
    // "destructured" into two variables, "i" and "data_segment" with a 
    // "destructuring assigment"
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // Process each data segment in a separate thread
        //
        // spawn() returns a handle to the new thread,
        // which we MUST keep to access the returned value
        //
        // 'move || -> u32' is syntax for a closure that:
        // * takes no arguments ('||')
        // * takes ownership of its captured variables ('move') and
        // * returns an unsigned 32-bit integer ('-> u32')
        //
        // Rust is smart enough to infer the '-> u32' from
        // the closure itself so we could have left that out.
        //
        // TODO: try removing the 'move' and see what happens
        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of the segment:
            let result = data_segment
                // iterate over the characters of our segment...
                .chars()
                // ... convert the text-characters to their number value...
                .map(|c| c.to_digit(10).expect("should be a digit"))
                // ... and sum the resulting iterator of numbers.
                .sum();
            
            // println! locks stdout, so no text-interleaving occurs
            println!("processed segment {}, result={}", i, result);

            // "return" not needed, because Rust is an "expression language", the
            // last evaluated expression in each block is automatically its value.
            result
        }));
    }

    /**************************************************************************
     * "Reduce" phase
     *
     * Collect our intermediate results, and combine them into a final result
     *************************************************************************/

    // Collect each thread's intermediate results into a new Vec
    let mut intermediate_sums = vec![];
    for child in children {
        // collect each child thread's return-value
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    // Combine all intermediate sums into a single final sum.
    //
    // we user the "turbofish" ::<> to provide sum() with a type hint.
    //
    // TODO: try without the turbofish, by instead explicitly
    // specifying the type of final_result.
    let final_result = intermediate_sums.iter().sum::<u32>();

    println!("Final sum result: {}", final_result);
}
