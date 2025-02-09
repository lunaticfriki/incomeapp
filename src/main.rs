slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui: AppWindow = AppWindow::new()?;
    let ui_handle: slint::Weak<AppWindow> = ui.as_weak();

    ui.on_divide_income({
        move |string: slint::SharedString| {
            let ui: AppWindow = ui_handle.unwrap();
            let num: f64 = string.trim().parse().unwrap();

            let tax:f64 = num * TAXPER;
            let owner:f64 = num * OWNERPER;
            let profit:f64 = num * PROFITPER;
            let opex:f64 = num * OPEXPER;
            let result: String = format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpex: {:.2}", tax, owner, profit, opex);

            ui.set_results(result.into());
        }
    });

    ui.run()
}
