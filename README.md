# Actix Web - Async JSON Service Boilerplate

## Motivation
Learning Rust can be difficult. Providing an idiomatic example of a purely JSON based, asynchronous service. Hope it helps more projects get started.

## Goals
* Leverage `actix-web` to fullest
* Leverage `serde` for JSON
* Async for all routes using Futures
* Best libraries, practices and performance

## Opinions
* JSON services need sensisble defaults out of the box
* Experimental/Mock routes favor generic `Map<String, Value>` over serialize/deserialize and strict types

## Mocks Status
* Loading `.json` files using `use std::fs::File` vs. `actix-files::fs`
* JSON response from `GET` using Futures
* _TODO_ allow simple `POST` where body overwrites file
* _TODO_ create/update files from `/mocks` endpoint

## TODO
1. Middleware for JWT?
