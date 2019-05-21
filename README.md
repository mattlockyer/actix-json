# Actix Web - Async JSON Service Boilerplate

## Motivation
Learning Rust can be difficult. Providing an idiomatic example of a purely JSON based, asynchronous service. Hope it helps more projects get started.

## Current Stack
* Actix
* Serde / Mock JSON FS
* JSON Function helpers for returning Actix::HttpResponse
* Environment variables in `.json` file
* Threaded DB connection pool for handling multiple async requests at once 
* `future_ok!` macro for writing handlers
* `sql_struct!` macro to create structs that satisfy diesel's ORM and raw sql queries

## Goals
* Leverage `actix-web` to fullest
* Leverage `serde` for JSON
* Async for all routes using Futures
* Best libraries, practices and performance

## Opinions
* JSON services need sensisble defaults out of the box
* Experimental/Mock routes favor generic `Map<String, Value>` over serialize/deserialize and strict types
* Metaprogramming (macros) used to reduce repeated and often lengthy definitions / signatures

## Environment Variables
Create a file in the root (not in src)
```
// env.json
{
  "postgres":{
    "username": "xxxxx",
    "password": "xxxxx"
  }
}
```

## Mocks Status
* GET /mock_get/{filename} will return the data from the file if it exists and 404 if it doesn't
* POST /mock_set/{filename} will create/overwrite a file and set contents to body of request

## TODO
1. Middleware for JWT?

## DB Setup (Postgres)
Database examples expect the following:
* There is a table schema called `test`
* There is a table called `item`
* item has field `id` that is type `serial`
* item has field `data` that is type `text`

Notes on setting up PostgresSQL

Start service

`# sudo service postgresql start`

Enter console without switching

`# sudo -u postgres psql`

Switch to postgres user

`# sudo i -u postgres`

Create new user

`sudo -u postgres createuser --interactive`
(default connect to db by username so maybe use `actix`?)

Create database

`# create database actix;`

Connect to database

`\c actix`

Create schema test and table w/ serial id

`# create schema test;`

`# create table test.item(id serial, data text);`

Grant permissions to schema / database
```
# GRANT USAGE ON SCHEMA database_name TO username;
# GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA database_name TO username;
# GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA schema_name TO username;
# GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA schema_name TO username;
# GRANT ALL PRIVILEGES ON DATABASE database_name TO username;
```