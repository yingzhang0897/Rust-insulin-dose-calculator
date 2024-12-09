use std::io;

struct UserData {
    current_glucose: f64,      // Current blood glucose level (mg/dL)
    target_glucose: f64,       // Target blood glucose level (mg/dL)
    carb_intake: f64,          // Estimated carbohydrate intake (grams)
    insulin_sensitivity: f64,  // Insulin sensitivity factor (mg/dL per unit)
    carb_ratio: f64,           // carb-to-Insulin ratio (grams per unit)
    dose_history: Vec<f64>,    // History of calculated doses
}

impl UserData {
    fn new() -> UserData {
        UserData {
            current_glucose: 0.0,
            target_glucose: 0.0,
            carb_intake: 0.0,
            insulin_sensitivity: 0.0,
            carb_ratio: 0.0,
            dose_history: Vec::new(),
        }
    }

    fn collect_data(&mut self) {
        self.current_glucose = read_input("Current blood glucose level (mg/dL): ");
        self.target_glucose = read_input("Target blood glucose level (mg/dL): ");
        self.carb_intake = read_input("Carbohydrate intake (grams): ");
        self.insulin_sensitivity = read_input("Insulin sensitivity factor (mg/dL per unit): ");
        self.carb_ratio = read_input("Insulin-to-carb ratio (grams per unit): ");
    }

    fn calculate_correction_dose(&self) -> f64 {
        let difference = self.current_glucose - self.target_glucose;
        if difference > 0.0 {
            difference / self.insulin_sensitivity
        } else {
            0.0 // No correction needed if glucose is below target
        }
    }

    fn calculate_carb_dose(&self) -> f64 {
        self.carb_intake / self.carb_ratio
    }

    fn calculate_total_dose(&mut self) -> f64 {
        let correction_dose = self.calculate_correction_dose();
        let carb_dose = self.calculate_carb_dose();
        let total_dose = correction_dose + carb_dose;

        // Store the calculated dose in the history
        self.dose_history.push(total_dose);
        total_dose
    }

    fn display_dose_history(&self) {
        println!("\nDose History:");
        for (index, dose) in self.dose_history.iter().enumerate() {
            println!("Dose {}: {:.2} units", index + 1, dose);
        }
    }
}

fn main() {
    println!("Welcome to the Insulin Dose Calculator!\n");

    let mut user_data = UserData::new();

    loop {
        user_data.collect_data();

        // Calculate insulin dose
        let correction_dose = user_data.calculate_correction_dose();
        let carb_dose = user_data.calculate_carb_dose();
        let total_dose = user_data.calculate_total_dose();

        // Display results
        println!("\nInsulin Dose Calculation:");
        println!("Correction dose: {:.2} units", correction_dose);
        println!("Carb dose: {:.2} units", carb_dose);
        println!("Total Insulin Dose: {:.2} units", total_dose);

        // Display dose history
        user_data.display_dose_history();

        println!("\nWould you like to calculate another doese? (yes or no)");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.trim().eq_ignore_ascii_case("no") {
            break;
        }
    }
    println!("\nConsult your doctor before using this program for medical decisions.");

}

fn read_input(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let _input: f64 = match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input, please enter a valid number.");
                continue;
            },
        };
    }
}
