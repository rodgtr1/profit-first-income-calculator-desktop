slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let ui = ui_handle.unwrap();
        match string.trim().parse::<f64>() {
            Ok(num) if num >= 0.0 => {
                let taxes: f64 = num * TAXPER;
                let owner: f64 = num * OWNERPER;
                let profit: f64 = num * PROFITPER;
                let opex: f64 = num * OPEXPER;
                let result = format!("Taxes: ${:.2}\nOwner: ${:.2}\nProfit: ${:.2}\nOpEx: ${:.2}", taxes, owner, profit, opex);
                ui.set_results(result.into());
            },
            Ok(_) => {
                ui.set_results("Please enter a positive number".into());
            },
            Err(_) => {
                ui.set_results("Please enter a valid number".into());
            }
        }
    });

    ui.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals_100() {
        let result = TAXPER + OWNERPER + PROFITPER + OPEXPER;
        let formatted = f64::trunc(result  * 100.0) / 100.0;
        assert_eq!(formatted, 1.00);
    }
}
