# Rust Template HTTP API

Rust API Template using PostgreSQL, Redis, RabbitMQ, and Hexagonal Architecture

The following template provides a basic structure for developing a Rust API, utilizing the powerful combination of PostgreSQL as a database, Redis as a caching system, and RabbitMQ for asynchronous communication. This template follows the principles of Hexagonal Architecture, also known as Ports and Adapters, which promotes loose coupling, separation of concerns, and modularity.

By adopting the Hexagonal Architecture, this template separates the core business logic from the external dependencies, enabling easier testing, maintainability, and flexibility. The API's core functionalities are encapsulated within the hexagon, while the adapters handle the integration with external services such as databases, caching systems, and message queues.

Note: Before using this template, ensure that you have installed and properly configured PostgreSQL, Redis, RabbitMQ, and the necessary dependencies.

## Requirements

- Rust (>=1.66.0)

## Postgresql, redis and rabbitmq

Use docker-compose to start requirements resources

```bash
docker-compose up -d
```

## Environments

Create a .env file with this default envs

| Key               | Value                                   |
| ----------------- | --------------------------------------- |
| ADDR              | 0.0.0.0:5000                            |
| RUST_LOG          | debug                                   |
| RUST_BACKTRACE    | 1                                       |
| PAGE_SIZE_DEFAULT | 12                                      |
| PAGE_SIZE_MAX     | 120                                     |
| DATABASE_USER     | postgres                                |
| DATABASE_PASSWORD | postgres                                |
| DATABASE_NAME     | postgres                                |
| DATABASE_HOST     | localhost                               |
| DATABASE_POOL_MAX | 16                                      |
| AMQP_ADDR         | amqp://rabbitmq:rabbitmq@127.0.0.1:5672 |
| REDIS_URL         | redis://localhost/0                     |

## How to execute

```bash
cargo run
```

## Documentation

Swagger documentation

```bash
http://127.0.0.1:5000/docs
```

## Folder structure

### amqp

In this step, the resources such as exchanges and queues of RabbitMQ are configured to start listening to events received in the queue. When a message is received, it is processed by the domain layer where the business rules of the application reside.

### api

In this section, the HTTP access for the application is configured, where routes, middlewares, HTTP error handling, authentication, and other common features in HTTP APIs are defined.

### domain

In the proposed architecture, all modules containing the application's business rules are grouped together to be used by adapters (API, AMQP). Within each module folder, there is a file that defines the repository implementation (Traits), a file that defines the data models, and a resources folder that contains files with module-specific business rules. This structure helps maintain a clear and modular organization, facilitating code comprehension and maintenance.

### repository

In this folder, connections with the database, access to external services such as HTTP APIs using Reqwest, connections with Redis and RabbitMQ, among others, are defined. Additionally, the repositories required by the domain layer are implemented, following the trait defined in each module or submodule of the domain.

This allows for centralizing and organizing the configurations and integrations with the different services and external resources used by the application, ensuring a more modular, reusable, and easily maintainable code.
