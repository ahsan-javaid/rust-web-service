# rust-web-service
Rust web server implemented using tcp protocol without using any frameworks or libraries.

### Runnig the project
```
cargo run .
```
or 

```
cargo r
```

### Dependencies
```
sqlite
serde
```

### Enviroment
Creat .env file in the root of the project
```
PORT=5000
DB=sqlite.db
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

### Architecture
Here is a light architecture diagram describing the relationship between modules in rust-web-service:

```
    +--main--+    +-routes-+   +--api---+   
    |        |--> +  GET   |-->| book   |   
    |  http  |    +  PUT   |   | users  |    
    | server |    |  POST  |   |        |
    |        |    | DELETE |   |        |
    +---+----+    +--------+   +--------+
        |       http listner      |
        |Roouting & Handler thread|
        +-------------------------+
                    |
                    |
                    |
    +-models-+   +--config--+    +----db----+
    |        |-->|  env     |    | queries  |
    |  book  |   +----------+ -->|-----------      
    |  user  |   |  db      |    | populate |
    |        |   | connect  |    |  models  |     
    +---+----+   +----------+    +----------+
        |                         |
        |    Database handling    |
        +-------------------------+
```

### Contribution
Pull requests are most welcome.

### Swagger docs

Add swagger docs at /docH- **Handlers .
-