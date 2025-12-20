# Profit First Income Calculator Desktop

**NOTE: A new Tauri Desktop [version of this app](https://github.com/rodgtr1/profit-first-income-calculator-tauri) is available also.**

A Rust desktop application using the Slint UI framework for calculating profit allocation based on the Profit First methodology. The app divides income into four categories: Taxes (30%), Owner pay (55%), Profit (5%), and Operating expenses (10%).

## About

This application helps you get started with the Profit First methodology by automatically calculating how to divide your income across key business categories. It provides a simple desktop interface built with Slint for the user interface and demonstrates the integration between `.slint` UI markup and Rust code.

## Usage

### Prerequisites

1. Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the `rustc` compiler and the `cargo` build system installed in your path.

### Running the Application

```bash
cargo run
```

### Common Commands

- **Run the application**: `cargo run`
- **Build the application**: `cargo build`
- **Run tests**: `cargo test`
- **Build for release**: `cargo build --release`
- **Bundle as macOS app**: `cargo bundle --release`

## Default Percentages

The application uses the following default allocation percentages:

- **Tax percentage**: 30%
- **Owner percentage**: 55%
- **Profit percentage**: 5%
- **Operating expenses percentage**: 10%

*Note: There is a test to check if your numbers equal 100%*

## Architecture

- **Main application**: `src/main.rs` - Contains business logic and Slint integration
- **UI definition**: `ui/appwindow.slint` - Slint markup defining the user interface
- **Build script**: `build.rs` - Compiles the Slint UI during build process
- **Constants**: Percentage allocations are hardcoded constants in main.rs

The app uses a simple callback-based architecture where the UI triggers the `divide_income` callback with user input, and the Rust code calculates results and updates the UI.

## macOS App Bundling

This project uses cargo-bundle to create native macOS applications. The bundle configuration is already set up in `Cargo.toml`:

```toml
[package.metadata.bundle]
name = "Income Divider"
identifier = "io.github.rodgtr1.profit-first-income-calculator-desktop"
description = "Diving up income into tax, owner, profit, and opex amounts."
icon = ["./icon.png", "./icon@2x.png"]
```

To create a .app bundle, run `cargo bundle --release`. The resulting app will be in `target/release/bundle/osx/`.

## Key Implementation Details

- Business logic uses floating-point arithmetic with `.2` precision formatting
- The UI includes a curtain animation effect that reveals results
- Input validation includes proper error handling for invalid/negative numbers
- Results display includes dollar signs for currency formatting
- Test ensures percentage constants sum to 100%

## Dependencies

- `slint = "1.12.1"` - UI framework (updated from 1.0)
- `slint-build = "1.12.1"` - Build-time UI compilation (updated from 1.0)

## Customizing Percentages

To change the percentages to match your business model, modify the allocation percentages defined as constants at the top of `src/main.rs`:

- `TAXPER`: Tax percentage (0.30)
- `OWNERPER`: Owner percentage (0.55) 
- `PROFITPER`: Profit percentage (0.05)
- `OPEXPER`: Operating expenses percentage (0.10)

When modifying these, ensure they sum to 1.0 (the test will catch this).

## Development

We recommend using an IDE for development, along with our [LSP-based IDE integration for `.slint` files](https://github.com/slint-ui/slint/blob/master/tools/lsp/README.md). You can also load this project directly in [Visual Studio Code](https://code.visualstudio.com) and install our [Slint extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint).

## Next Steps

To learn more about the Slint APIs and the `.slint` markup language, check out the [online documentation](https://slint-ui.com/docs/rust/slint/).
