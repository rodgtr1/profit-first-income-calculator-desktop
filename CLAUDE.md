# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust desktop application using the Slint UI framework for calculating profit allocation based on the Profit First methodology. The app divides income into four categories: Taxes (30%), Owner pay (55%), Profit (5%), and Operating expenses (10%).

## Architecture

- **Main application**: `src/main.rs` - Contains business logic and Slint integration
- **UI definition**: `ui/appwindow.slint` - Slint markup defining the user interface
- **Build script**: `build.rs` - Compiles the Slint UI during build process
- **Constants**: Percentage allocations are hardcoded constants in main.rs

The app uses a simple callback-based architecture where the UI triggers the `divide_income` callback with user input, and the Rust code calculates results and updates the UI.

## Common Commands

- **Run the application**: `cargo run`
- **Build the application**: `cargo build`
- **Run tests**: `cargo test`
- **Build for release**: `cargo build --release`
- **Bundle as macOS app**: `cargo bundle --release`

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

## Modifying Percentages

The allocation percentages are defined as constants at the top of `src/main.rs`:
- `TAXPER`: Tax percentage (0.30)
- `OWNERPER`: Owner percentage (0.55) 
- `PROFITPER`: Profit percentage (0.05)
- `OPEXPER`: Operating expenses percentage (0.10)

When modifying these, ensure they sum to 1.0 (the test will catch this).