# Binance API transaction example

This repo shows an example of how to interact with the Binance Spot API. In particular account information is retrieved and then, an order is placed and canceled.

This repository was made to support an article on my blog [Binance API crypto transaction with Rust: how to](https://tms-dev-blog.com/binance-api-crypto-transaction-with-rust-how-to)

WARNING: this code will try to place an actual order to buy LTC with BTC. Execute at your own risk!

## Setup

This code requires BINANCE_API_KEY and BINANCE_SECRET_KEY environment variables to run. Recommended is adding a `.env` file to the root of the project directory. This will get loaded in by the `dotenv` crate.

These keys can be generated in your Binance profile under API Management.

## Running the code

The program can be run simply using `cargo run`.
