use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::any::Any;

struct Loan<T> {
    loan_type: LoanType,
    loan_objective: LoanObjective,
    interest_rate: T,
    loan_amount: T,
    loan_period: T,
    present_value: T,
}

enum LoanType {
    Anniuty,
    Serial,
}

enum LoanObjective {
    Car,
    Housing,
    Consumer,
}

trait CalculateLoan {
    fn serial_loan(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

impl<T: Any> CalculateLoan for Loan<T> {
    // , <T as Div<f32>>::Output
    fn serial_loan(&self) -> String {
        let value = self.as_any();
        if value.is::<f32>() {
            std::any::type_name::<T>().to_string()
            // todo!()
        } else if value.is::<Decimal>() {
            std::any::type_name::<T>().to_string()
            // todo!()
        } else {
            std::any::type_name::<T>().to_string()
            // todo!()
        }
    }

    fn as_any(&self) -> &dyn Any {
        &self.interest_rate
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_f32_loan() {
        let new_f32_loan = Loan {
            loan_type: LoanType::Anniuty,
            loan_objective: LoanObjective::Car,
            interest_rate: 3.5 as f32,
            loan_amount: 5000 as f32,
            loan_period: 2 as f32,
            present_value: 0 as f32,
        };
        assert_eq!(&"f32".to_string(), &new_f32_loan.serial_loan())
    }

    #[test]
    fn test_decimal_loan() {
        let new_decimal_loan = Loan {
            loan_type: LoanType::Anniuty,
            loan_objective: LoanObjective::Car,
            interest_rate: dec!(3.5),
            loan_amount: dec!(5000),
            loan_period: dec!(2),
            present_value: dec!(0),
        };
        println!("{}", &new_decimal_loan.serial_loan());
        assert_eq!(
            &"rust_decimal::decimal::Decimal".to_string(),
            &new_decimal_loan.serial_loan()
        )
    }
}
