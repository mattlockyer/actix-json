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
* GET /mock_get/{filename} will return the data from the file if it exists and 404 if it doesn't
* POST /mock_set/{filename} will create/overwrite a file and set contents to body of request

## TODO
1. Middleware for JWT?
