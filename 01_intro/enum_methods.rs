enum PaymentMode {
    Debit,
    Credit,
    Paypal,
}

// handle the payment of different modes
fn pay_by_credit(amt: usize) {
    println!("Processing credit payment of {}", amt);
}

fn pay_by_debit(amt: usize) {
    println!("Processing debit payment of {}", amt);
}

fn paypal_redirect(amt: usize) {
    println!("Redirecting to paypal for amount: {}", amt);
}

impl PaymentMode {
    fn pay(&self, amount: usize) {
        match self {
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Paypal => paypal_redirect(amount),
        }
    }
}

// return the saved payment mode per user
fn get_saved_payment_mode() -> PaymentMode {
    // returned possible mode is limited by a defined enum
    PaymentMode::Debit
}

fn main() {
    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(512);
}
