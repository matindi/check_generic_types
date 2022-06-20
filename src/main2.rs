#![feature(type_alias_impl_trait)]
#![feature(associated_type_defaults)]

use rust_decimal::{prelude::FromPrimitive, Decimal};
use rust_decimal_macros::dec;
use rust_decimal::Decimal::log10;
use std::{
    cmp,
    ops::{Add, Mul, Sub, Div},
};

enum Priotity {
    Snowball,
    Avalanche,
    MinimumOnly,
}

#[derive(Clone)]
struct Loan<T> {
    name: String,
    balance_history: Vec<T>,
    payment_history: Vec<T>,
    remaining_history: Vec<T>,
    interest_rate: T,
    min_payment: T,
    principal: T,
    interest: T,
    principal_paid: T,
    interest_paid: T,
    year: T,
}



// impl<T> From<Loan<T>> for Loan<T> {
//     fn from(_: Loan<T>) -> Self {
//         Loan {
//             name: (),
//             balance_history: (),
//             payment_history: (),
//             remaining_history: (),
//             interest_rate: (),
//             min_payment: (),
//             principal: (),
//             interest: (),
//             principal_paid: (),
//             interest_paid: (),
//         }
//     }
// }

impl<T> Default for Loan<T>
where
    T: Default,
{
    fn default() -> Self {
        Loan {
            name: String::new(),
            balance_history: vec![],
            payment_history: vec![],
            remaining_history: vec![],
            interest_rate: T::default(),
            min_payment: T::default(),
            principal: T::default(),
            interest: T::default(),
            principal_paid: T::default(),
            interest_paid: T::default(),
            year: T::default(),
        }
    }
}

trait Calculate{
    fn get_num_remaining_payments(&self,payment: Option<T>); 
}

impl<T> Loan<T>
where
    T:    Add<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + Div<Output = T>
        + std::ops::Add
        + std::ops::Mul
        + std::ops::Div
        + std::ops::AddAssign
        + Clone
        + std::cmp::Ord
        + std::ops::Sub
        + std::ops::SubAssign,
{
    fn balance(&self) -> T {
        self.clone().interest + self.clone().principal
    }

    fn balance_paid(&self) -> T {
        self.clone().interest_paid + self.clone().principal_paid
    }

    fn compound(&mut self) -> T {
        let interest = self.clone().balance().clone() * self.clone().interest_rate.clone();
        self.interest += interest;
        self.clone().balance()
    }

    fn make_payment(&mut self, payment: Option<T>) -> T {
        match payment {
            None => self.clone().min_payment,
            Some(payment) => {
                let adjusted_payment = (cmp::min(payment.clone(), self.clone().balance())).clone();
                let refund = payment.clone() - adjusted_payment.clone();
                let interest_payment = cmp::min(adjusted_payment.clone(), self.clone().interest);
                let principal_payment = adjusted_payment.clone() - interest_payment.clone();
                self.interest_paid += interest_payment.clone();
                self.principal_paid += principal_payment.clone();
                self.interest -= interest_payment.clone();
                self.principal -= principal_payment;

                self.balance_history.push(self.balance().clone());
                self.payment_history.push((adjusted_payment).clone());
                // let num_payments_left =
                //
                refund
            }
        }
    }
}

impl Calculate for Loan<Decimal> {
    fn get_num_remaining_payments(&self,payment:Option<Decimal>) where {
        match payment {
            Some(payment) => {
                let monthly_rate = self.clone().interest_rate / self.clone().year;
                let interest = monthly_rate * self.clone().balance();
                let num_payments = -log10(dec![1]-interest/payment)/log10(1+monthly_rate)
            }
            None => {
                self.clone().min_payment
            }
        }
    }
}

impl<T> Calculate<T> for Loan<f32>{
    fn get_num_remaining_payments(&self, payment:Option<T>) {
        todo!()
    }
}

// impl From<Loan<f32>> for Loan<Decimal> {
//     fn from(loan: Loan<f32>) -> Self {
//         type T = Vec<Decimal>;
//         let mut balance_history: T = vec![];
//         let mut payment_history: T = vec![];
//         let mut remaining_history: T = vec![];

//         |_: T| {
//             for i in loan.balance_history {
//                 balance_history.push(Decimal::from_f32(i).unwrap());
//             }
//         };

//         |_: T| {
//             for i in loan.payment_history {
//                 payment_history.push(Decimal::from_f32(i).unwrap());
//             }
//         };

//         |_: T| {
//             for i in loan.remaining_history {
//                 remaining_history.push(Decimal::from_f32(i).unwrap());
//             }
//         };

//         Loan {
//             name: loan.name,
//             balance_history: balance_history,
//             payment_history: payment_history,
//             remaining_history: remaining_history,
//             interest_rate: Decimal::from_f32(loan.interest_rate).unwrap(),
//             min_payment: Decimal::from_f32(loan.min_payment).unwrap(),
//             principal: Decimal::from_f32(loan.principal).unwrap(),
//             interest: Decimal::from_f32(loan.interest).unwrap(),
//             principal_paid: Decimal::from_f32(loan.principal_paid).unwrap(),
//             interest_paid: Decimal::from_f32(loan.interest_paid).unwrap(),
//             year: Decimal::from_f32(loan.year).unwrap(),
//         }
//     }
// }

fn main() {

    // let number = dec!(-1.23);
}
