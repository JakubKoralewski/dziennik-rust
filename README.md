<!---
  MIT License
  Copyright (c) 2019 Jakub Koralewski
-->
# dziennik-rust [![Deploy](https://www.herokucdn.com/deploy/button.png)](https://heroku.com/deploy)

A simple CRUD REST API backend application using Rust with actix-web, Diesel and PostgreSQL.

## Develop

Auto reload:
```
$ cargo run-script dev
```

## *Inspired* by

I've used these sources to learn these topics:

- actix:
  - json: https://github.com/actix/examples/blob/master/json/src/main.rs
  - diesel:
    - SQLite:
      - https://github.com/actix/examples/tree/master/diesel
    - PostgreSQL:
      - https://github.com/actix/examples/tree/master/actix_todo
        - actix, postgresql, tera for templates, diesel for ORM
  - full fetched project:
    - https://github.com/ryanmcgrath/jelly
- diesel:
  - official Getting Started Guide:
    - https://diesel.rs/guides/getting-started/
  - official README:
    - https://github.com/diesel-rs/diesel/blob/master/diesel_cli/README.md


## Entry points (CRUD)

### Postman Collection: [docs/dziennik_rust.postman_collection.json](docs/dziennik_rust.postman_collection.json)


`// Content-Type: application/json; charset=UTF-8`

- /api/
  - ### login
    - POST (login, password):
      - credentials for login 
  - ### students
    - GET:
      - get all students
      - array of Student object
    - POST body: (first_name, last_name):
      - check if student exists
      - add student
      - return new_student
    - /{id}
      - DELETE:
        - delete student
      - PUT body:(new_student):
        - edit existing student