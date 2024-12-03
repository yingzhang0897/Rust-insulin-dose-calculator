// use std::io;

// struct UserData {
//     current_glucose: f64,      // Current blood glucose level (mg/dL)
//     target_glucose: f64,     // Target blood glucose level (mg/dL)
//     carb_intake: f64,        // Estimated carbohydrate intake (grams)
//     insulin_sensitivity: f64, // Insulin sensitivity factor (mg/dL per unit)
//     carb_ratio: f64,         // Insulin-to-carb ratio (grams per unit)
// }

// fn main() {
//     println!("Welcome to the Insulin Dose Calculator!\n");

//     println!("Please provide the following information:");

//     // Collect user data
//     let user_data = UserData {
//         current_glucose: read_input("Current blood glucose level (mg/dL): "),
//         target_glucose: read_input("Target blood glucose level (mg/dL): "),
//         carb_intake: read_input("Carbohydrate intake (grams): "),
//         insulin_sensitivity: read_input("Insulin sensitivity factor (mg/dL per unit): "),
//         carb_ratio: read_input("Insulin-to-carb ratio (grams per unit): "),
//     };

//     // Calculate insulin dose
//     let correction_dose = calculate_correction_dose(
//         user_data.current_glucose,
//         user_data.target_glucose,
//         user_data.insulin_sensitivity,
//     );

//     let carb_dose = calculate_carb_dose(user_data.carb_intake, user_data.carb_ratio);

//     let total_dose = correction_dose + carb_dose;

//     // Display results
//     println!("\nInsulin Dose Calculation:");
//     println!("Correction Dose: {:.2} units", correction_dose);
//     println!("Carbohydrate Dose: {:.2} units", carb_dose);
//     println!("Total Insulin Dose: {:.2} units", total_dose);

//     println!("\nConsult your doctor before using this program for medical decisions.");
// }

// fn read_input(prompt: &str) -> f64 {
//     loop {
//         println!("{}", prompt);
//         let mut input = String::new();
//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read input");
//         //let input: f64 = input.trim().parse().expect("Invalid input. Please enter a valid number.")
//         match input.trim().parse::<f64>() {
//             Ok(value) => return value,
//             Err(_) => println!("Invalid input. Please enter a valid number."),
//         }
//     }
// }

// fn calculate_correction_dose(current_glucose: f64, target_glucose: f64, sensitivity: f64) -> f64 {
//     let difference = current_glucose - target_glucose;
//     if difference > 0.0 {
//         difference / sensitivity
//     } else {
//         0.0 // No correction needed if glucose is below target
//     }
// }

// fn calculate_carb_dose(carb_intake: f64, carb_ratio: f64) -> f64 {
//     carb_intake / carb_ratio
// }
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
     
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }    
    }
    println!("The secret number is: {secret_number}");
}
