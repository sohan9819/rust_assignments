use colored::*;
use std::{io, str::FromStr};

fn get_input<T: FromStr>(prompt: &str) -> T {
    loop {
        println!("-------------------------------------");
        println!("{}", prompt.yellow());
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect(&"Failed to read line".red());

        if input.is_empty() {
            println!("{}", "Empty input is not allowed :(".red());
            continue;
        } else {
            match input.trim().parse::<T>() {
                Ok(val) => {
                    println!("-------------------------------------");
                    break val;
                }
                Err(_) => {
                    println!("{}", "Invalid input !!! Please try again.".red());
                    continue;
                }
            };
        }
    }
}

fn get_optional_input<T: FromStr>(prompt: &str) -> Option<T> {
    println!("{}", prompt.yellow());
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect(&"Failed to read line".red());

    if input.is_empty() {
        None
    } else {
        match input.trim().parse::<T>() {
            Ok(val) => {
                println!("-------------------------------------");
                Some(val)
            }
            Err(_) => {
                println!("{}", "Invalid input !!!".red());
                None
            }
        }
    }
}

#[derive(Debug)]
struct User {
    name: String,
    income: u32,
    age: u8,
    loan_amount: u32,
    co_applicant_name: Option<String>,
}

enum LoanEligibilityError {
    AgeError,
    IncomeError,
}

impl User {
    fn new() -> Self {
        let name: String = get_input("Please enter your name");
        let income: u32 = get_input("Please enter your income");
        let age: u8 = get_input("Please enter your age");
        let loan_amount: u32 = get_input("Please enter requested loan amount");
        let co_applicant_name: Option<String> =
            get_optional_input("Please enter co-applicant name");

        User {
            name,
            income,
            age,
            loan_amount,
            co_applicant_name,
        } // could be a heap object if Box::new(User{...})
    }

    fn show_user_info(&self) {
        println!("{}", "################################".green());
        println!("Your name : {}", &self.name);
        println!("Your income : {}", &self.income);
        println!("Your age : {}", &self.age);
        println!("Requested loan amount : {}", &self.loan_amount);
        // match &self.co_applicant_name {
        //     Some(name) => println!("Co-Applicant Name : {}", name),
        //     None => {}
        // }
        if let Some(name) = &self.co_applicant_name {
            println!("Co-Applicant Name : {}", name)
        }
        println!("{}", "################################".green());
    }

    fn check_loan_eligibility(&self) -> Result<(), LoanEligibilityError> {
        if self.age < 20 {
            Err(LoanEligibilityError::AgeError)
        } else if self.income < 100 {
            Err(LoanEligibilityError::IncomeError)
        } else {
            Ok(())
        }
    }
}

impl Drop for User {
    fn drop(&mut self) {
        println!("{}", "User got dropped !!!".bright_yellow());
    }
}

fn main() {
    println!("Hello, world!");

    let user = User::new();

    user.show_user_info();

    match user.check_loan_eligibility() {
        Ok(_) => println!("{}", "✅ You are eligible for the loan!".green()),
        Err(LoanEligibilityError::AgeError) => {
            println!("{}", "❌ Not eligible due to age.".yellow())
        }
        Err(LoanEligibilityError::IncomeError) => {
            println!("{}", "❌ Not eligible due to income.".yellow())
        }
    }
}
