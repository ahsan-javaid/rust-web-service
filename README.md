# rust-web-service
Rust web server implemented using tcp protocol without using any frameworks or libraries.

### Runnig the project
```
cargo run
```

### Dependencies
```
sqlite
serde
```

### API documentation
Base url: http://localhost:5000
```
GET /users
GET /users/{id}
POST /users
GET /books
GET /books/{id}
POST /books
```

### Models
```
User(id, name, email)
Book(id, title, author)
```