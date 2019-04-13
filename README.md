# dziennik_php_rust

## Develop

Auto reload:
```
$ cargo run-script dev
```

## *Inspired* by

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


## Entry points

### Postman Collection: [docs/dziennik_rust.postman_collection.json](docs/dziennik_rust.postman_collection.json)


`// Content-Type: application/json; charset=UTF-8`

- /api/
  - ### login
    - POST (login, password):
      - credentials for login 
  - ### students
    - GET ():
      - get all students
    - POST (first_name, last_name):
      - check if student exists
      - add student
    - DELETE (id):
      - delete student
    - PUT (id, new_data):
      - edit existing student