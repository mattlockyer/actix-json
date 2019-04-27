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

## TODO
1. Add Mocks using `actix_web:fs`
2. Middleware for JWT?
