pub fn tax(income: f64) -> f64 {
    // panic if income is negative
    if income < 0.0 {
        panic!("tax(): income cannot be negative");
    }

    // panic if income does not have integer value equivalent
    if income != income.floor() {
        panic!("tax(): income must be an integer");
    }
    
    // get the tax rate
    let mut tax_rate: u8 = 0;
    if income >= 0.0 && income < 10000.0 {
        tax_rate = 0;
    }
    else if income >= 10000.0 && income < 50000.0 {
        tax_rate = 10;
    }
    else if income >= 50000.0 && income < 100000.0 {
        tax_rate = 20;
    }
    else if income >= 100000.0 && income < 1000000.0 {
        tax_rate = 30;
    }
    else {
        tax_rate = 40;
    }

    let mut taxed_income: f64 = income - (income * (tax_rate as f64 / 100.0));

    return taxed_income;
}
