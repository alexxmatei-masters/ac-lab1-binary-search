use std::io;

fn get_user_input_u16_value() -> u16 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number = match input.trim().parse::<u16>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return get_user_input_u16_value(); // Recursive call to retry input
        }
    };

    println!("The number you entered is: {}", number);
    number
}

fn main() {
    let my_array: &mut [u16] = &mut [3, 1, 8, 15, 6, 32];
    let mut low_index: u8 = 0;
    let mut high_index: u8 = (my_array.len() - 1) as u8;
    let mut middle_index: u8;
    println!("Please enter the target number [0, 65,535]:");
    let target_number: u16 = get_user_input_u16_value();
    println!("Target number: {}", target_number);
    println!("Initial array: {:?}", my_array);
    my_array.sort();
    println!("Sorted array: {:?}", my_array);

    println!("Starting binary search algorihm...\n");

    loop {
        print!("Searching the middle of the array...\n");
        middle_index = (low_index + high_index) / 2;
        let middle_number = my_array[middle_index as usize];
        println!("Found the number: {}", middle_number);
        if middle_number == target_number {
            println!("Target number found at index {}", middle_index);
            break;
        }
        if low_index == high_index {
            println!("Couldn't find target number in the given array.");
            break;
        }

        if target_number < middle_number {
            println!(
                "Found number is higher than target number: {}",
                target_number
            );
            println!("Creating a new array from the 1st half of the old array...");
            high_index = middle_index - 1;
        } else if target_number > middle_number {
            print!(
                "Found number is lower than than target number: {}\n",
                target_number
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
