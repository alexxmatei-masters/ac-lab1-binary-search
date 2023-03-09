fn main() {
    let mut my_array: &mut [i32] = &mut [3, 1, 8, 15, 25, 26];
    let searched_element = 8;
    let mut element_found = false;
    my_array.sort();
    print!("{}", my_array[my_array.len() / 2]);
    while (my_array.len() > 1 && !element_found) {
        let element = my_array[my_array.len() / 2];
        if element == searched_element {
            print!("We found the number at index {} ", my_array.len() / 2);
            element_found = true;
        } else if element < searched_element {
            print!("Element is smaller than searched element, look in 2nd half.\n");
            let (_, right_half) = my_array.split_at_mut(my_array.len() / 2);
            my_array = right_half;
            print!("{:?}\n", my_array);
        } else if element > searched_element {
            print!("Element is bigger than searched element, look in 1st half.\n");
            let (left_half, _) = my_array.split_at_mut(my_array.len() / 2);
            my_array = left_half;
            print!("{:?}\n", my_array);
        }
    }
    if my_array[0] == searched_element {
        element_found = true;
        print!("We found the number at index {} ", 0);
    }
    if !element_found {
        print!("We didn't find the element.")
    }
}
