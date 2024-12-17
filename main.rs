mod currency;
mod operations;

use currency::Currency;

fn main() {
    let usd: Currency<f64> = Currency::new(100.0, "USD");
    let eur: Currency<f64> = Currency::new(85.0, "EUR");

    println!("Initial amounts:");
    println!("USD: {}", usd);
    println!("EUR: {}", eur);

    let usd_add = usd.clone() + Currency::new(50.0, "USD");
    println!("After adding 50 USD: {}", usd_add);

    let usd_sub = usd_add.clone() - Currency::new(30.0, "USD");
    println!("After subtracting 30 USD: {}", usd_sub);

    let usd_mul = usd.clone() * 1.5;
    println!("After multiplying USD by 1.5: {}", usd_mul);

    let usd_div = usd.clone() / 2.0;
    println!("After dividing USD by 2: {}", usd_div);
}
