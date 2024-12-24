# rust-web-service
This project demonstrates how to build a basic web server in Rust from scratch, using only the standard library. It covers the implementation of a TCP server, request parsing, routing, and response handling. The goal is to provide a deeper understanding of how web servers work under the hood and to showcase Rust's capabilities in systems programming.

The server supports basic CRUD operations for `User` and `Book` models, with data stored in an SQLite database. The project is structured to separate concerns, making it easier to maintain and extend. Key components include:

- **HTTP Server**: Listens for incoming TCP connections and handles HTTP requests.
- **Routing**: Directs requests to the appropriate handler based on the URL and HTTP method.
- **Handlers**: Process requests, interact with the database, and generate responses.
- **Models**: Define the structure of the data and provide methods for database interaction.
- **Configuration**: Manages environment variables and database connections.

By following this project, you will gain insights into low-level web server development and Rust's concurrency model, which ensures safe and efficient handling of multiple requests.

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

Add swagger docs at /doc