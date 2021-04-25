use std::io;

fn main() {
    println!("Welcome to Data Structures and Alogrithms\n\nMenu:\n1. Run an insertion sort - ascending\n2. Run an insertion sort - descending\n3. Selection Sort\n\nctrl+c to exit...\n\nEnter a menu option");
	loop {
		let user_input = get_user_input();
		if user_input == 1 {
			do_insertion_sort();
		}else if user_input == 3 {
			do_selection_sort();
		}else {
			println!("Wrong input");
		}
	}		
}

fn do_selection_sort() {
	println!("Selection sort - ascending :: start-----------\n\n");
	let mut input_array = [34,12,3,9,13,5,2,1];
	
	for j in 0..input_array.len() {
		let mut key = input_array[j];
		let mut key_position = j;
		for sort_run in (j+1)..input_array.len() {
			if key < input_array[sort_run] {
				continue;
			}else{
				key = input_array[sort_run];
				key_position = sort_run;
			}		
		}
		let temp = input_array[j];
		input_array[j] = key;
		input_array[key_position] = temp;
	}
	
	println!("\tOutput\n");
	print!("\t");
	for element in input_array.iter() {
		print!("{},",element);
	}	
	println!("\n\nSelection sort - ascending :: finish-----------\n\n");
	
}

fn do_insertion_sort() {
	println!("Insertion sort - ascending :: start-----------\n\n");

	//let mut input_array = [2,5,6,3,5,1];
	let mut input_array = [34,12,3,9,13,5,2,1];

	if input_array.len() > 1 {	
		for sort_run in 1..input_array.len() {
			let current_element = input_array[sort_run];		
			for counter in (0..sort_run).rev() {
				if current_element < input_array[counter] {
					let temp = input_array[counter];
					input_array[counter+1] = temp;
					input_array[counter] = current_element;
				}else{
					break;
				}
			}	
		}		
	}
	println!("\tInsertion sort - ascending :: Output\n");
	print!("\t");
	for element in input_array.iter() {
		print!("{},",element);
	}	
	println!("\n\nInsertion sort - ascending :: finish-----------\n\n");
}

fn get_user_input() -> u64 {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer).expect("Invalid input");
	let buffer : u64 = buffer.trim().parse().expect("Invalid number");
	buffer
}