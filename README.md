# Currency Representation in Rust

## Overview

This project demonstrates a modular implementation of a currency representation system in Rust. The system supports generic numeric types and provides safe arithmetic operations between currency values, ensuring that operations between different currencies are prohibited.

---

## Features

- **Generic Numeric Support**: Works with any numeric type (`f64`, `i32`, etc.).
- **Safe Arithmetic Operations**: Supports addition, subtraction, multiplication, and division while ensuring operations between different currencies are disallowed.
- **Modular Design**: Code is split across multiple files for better readability and maintainability.
- **Custom Display**: Outputs currency values in a readable format (e.g., `100.0 USD`).

---

## Code Structure

```
src/
├── main.rs         # Entry point of the program
├── currency.rs     # Contains the Currency struct and related methods
└── operations.rs   # Implements arithmetic operations for the Currency struct
```

---

## How to Run

### Prerequisites
- Rust (latest stable version)

### Steps
1. Clone the repository:
   ```bash
   git clone https://github.com/yousef2342K/currency-representation-with-rust.git
   cd currency-representation-with-rust
   ```

2. Run the program:
   ```bash
   cargo run
   ```

---

## Example Usage

### Code
```rust
let usd: Currency<f64> = Currency::new(100.0, "USD");
let eur: Currency<f64> = Currency::new(85.0, "EUR");

println!("USD: {}", usd);
println!("EUR: {}", eur);

let usd_add = usd.clone() + Currency::new(50.0, "USD");
println!("After adding 50 USD: {}", usd_add);

let usd_sub = usd.clone() - Currency::new(30.0, "USD");
println!("After subtracting 30 USD: {}", usd_sub);

let usd_mul = usd.clone() * 2.0;
println!("After multiplying USD by 2: {}", usd_mul);

let usd_div = usd.clone() / 2.0;
println!("After dividing USD by 2: {}", usd_div);
```

### Output
```
USD: 100.0 USD
EUR: 85.0 EUR
After adding 50 USD: 150.0 USD
After subtracting 30 USD: 70.0 USD
After multiplying USD by 2: 200.0 USD
After dividing USD by 2: 50.0 USD
```

---

## Future Enhancements

- Add currency conversion using exchange rates.
- Include error handling for invalid currency codes.
- Integrate localization for different formats (e.g., `1,000.00 USD` vs `1.000,00 USD`).

---

## License

This project is open-source and available under the MIT License.

---

## Author

- **Yousef**
- GitHub: [yousef2342K](https://github.com/yousef2342K)
```
