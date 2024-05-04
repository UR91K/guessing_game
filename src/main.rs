use std::io;
use rand::Rng;

fn main() {
    let mut play_again: bool = true;

    while play_again {
        let random_numbers: [i32; 3] = generate_random_numbers();
        let mut correct_guesses: [bool; 3];
        let mut attempts_remaining: i32 = 3;

        println!("Welcome to the guessing game. You must guess three numbers between 1 and 5.");

        while attempts_remaining > 0 {
            println!("You have {} attempt(s) remaining.", attempts_remaining);
            let users_guesses: [i32; 3] = get_user_input();
            correct_guesses = compare_guesses(random_numbers, users_guesses);
            attempts_remaining -= 1;

            if correct_guesses[0] && correct_guesses[1] && correct_guesses[2] {
                println!("You guessed all the numbers correctly with {} attempt(s) remaining!", 3 - attempts_remaining);
                break;
            }
            if attempts_remaining == 0 {
                println!("You ran out of attempts. The treasure is sealed forever!");
            }
        }

        println!("The code was {:?}", random_numbers);

        let mut choice = String::new();

        loop {
            println!("Would you like to play again? (y/n)");
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line.");

            choice = choice.trim().to_lowercase();

            if choice == "y" {
                break;
            } else if choice == "n" {
                play_again = false;
                println!("Thank you for playing the guessing game!");
                break;
            } else {
                println!("Please enter 'y' or 'n'.");
                choice.clear();
            }
        }
    }
}

pub fn generate_random_numbers() -> [i32; 3] {
    let mut rng = rand::thread_rng();
    let lower_bound: i32 = 1;
    let upper_bound: i32 = 5;
    let mut random_numbers: [i32; 3] = [0, 0, 0];

    for i in 0..3 {
        random_numbers[i] = rng.gen_range(lower_bound..upper_bound);
    }

    random_numbers
}

fn compare_guesses(random_numbers: [i32; 3], guessed_numbers: [i32; 3]) -> [bool; 3] {
    let mut correct_guesses: [bool; 3] = [false, false, false];

    for i in 0..3 {
        if random_numbers[i] == guessed_numbers[i] {
            println!("{} is correct!", guessed_numbers[i]);
            correct_guesses[i] = true;
        } else {
            let mut is_duplicate : bool = false;
            for j in 0..3 {
                if j != i && random_numbers[j] == guessed_numbers[i] {
                    println!("{} is one of the numbers, but it's in the wrong position.", guessed_numbers[i]);
                    is_duplicate = true;
                    break
                }
            }
            if !is_duplicate {
                println!("{} is incorrect!", guessed_numbers[i])
            }
        }
    }

    correct_guesses
}

fn convert_string_to_array(input: &str) -> Result<Vec<i32>, String> {
    let mut numbers: Vec<i32> = Vec::new();

    for number_str in input.split_whitespace() {
        match number_str.parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => return Err(format!("Failed to parse {}", number_str)),
        }
    }
    Ok(numbers)
}

fn get_user_input() -> [i32; 3] {
    let mut input_buffer = String::new();

    loop {
        input_buffer.clear();
        println!("Input your guess as three numbers separated by spaces (e.g., \"1 2 3\"): ");

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line.");

        match convert_string_to_array(&input_buffer.trim()) {
            Ok(numbers) => {
                if numbers.len() == 3 {
                    return [numbers[0], numbers[1], numbers[2]];
                } else {
                    println!("You entered your numbers incorrectly!");
                }
            }
            Err(err) => {
                println!("{}, you entered your numbers incorrectly!", err);
            }
        }
    }
}