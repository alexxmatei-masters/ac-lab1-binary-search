fn main() {
    let my_array: &mut [u16] = &mut [3, 1, 8, 15, 6, 32];
    let mut low_index: u8 = 0;
    let mut high_index: u8 = (my_array.len() - 1) as u8;
    let mut middle_index: u8;
    const TARGET_NUMBER: u16 = 7;
    println!("Target number: {}", TARGET_NUMBER);
    println!("Initial array: {:?}", my_array);
    my_array.sort();
    println!("Sorted array: {:?}", my_array);

    println!("Starting binary search algorihm...\n");

    loop {
        print!("Searching the middle of the array...\n");
        middle_index = (low_index + high_index) / 2;
        let middle_number = my_array[middle_index as usize];
        println!("Found the number: {}", middle_number);
        if middle_number == TARGET_NUMBER {
            println!("Target number found at index {}", middle_index);
            break;
        }
        if low_index == high_index {
            println!("Couldn't find target number in the given array.");
            break;
        }

        if TARGET_NUMBER < middle_number {
            println!(
                "Found number is higher than target number: {}",
                TARGET_NUMBER
            );
            println!("Creating a new array from the 1st half of the old array...");
            high_index = middle_index - 1;
        } else if TARGET_NUMBER > middle_number {
            print!(
                "Found number is lower than than target number: {}\n",
                TARGET_NUMBER
            );
            println!("Creating a new array from the 2nd half of the old array...");
            low_index = middle_index + 1;
        }

        println!(
            "New array: {:?}\n",
            &my_array[low_index as usize..(high_index + 1) as usize]
        )
    }
}
