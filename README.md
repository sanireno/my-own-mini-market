# My Own Mini Market

A REST API for a small marketplace written in Rust.

The project is being developed as a backend development practice project using Rust, Axum and PostgreSQL.

## Features

### Products

* Create products
* Get all products
* Get product by ID
* Update products
* Delete products

### Categories

* Create categories
* Get all categories
* Get category by ID
* Update categories
* Delete categories
* Get products belonging to a category

### Authentication

* User registration
* User login
* Password hashing with Argon2
* JWT token generation and validation
* Role-based access control for catalogue management

### Cart

* Add products to a user-specific cart
* Get cart items with current product details and totals
* Update item quantity
* Remove cart items
* Validate quantities against available stock

### Database

* PostgreSQL
* SQLx
* Database migrations
* Async connection pool

## Tech Stack

* Rust
* Axum
* Tokio
* PostgreSQL
* SQLx
* Serde
* Argon2
* JSON Web Tokens (JWT)
* Chrono
* dotenvy

## API Endpoints

### Products

| Method | Endpoint         | Description       |
| ------ | ---------------- | ----------------- |
| GET    | `/products`      | Get all products  |
| POST   | `/products`      | Create a product  |
| GET    | `/products/{id}` | Get product by ID |
| PUT    | `/products/{id}` | Update a product  |
| DELETE | `/products/{id}` | Delete a product  |

### Categories

| Method | Endpoint                    | Description                  |
| ------ | --------------------------- | ---------------------------- |
| GET    | `/categories`               | Get all categories           |
| POST   | `/categories`               | Create a category            |
| GET    | `/categories/{id}`          | Get category by ID           |
| PUT    | `/categories/{id}`          | Update a category            |
| DELETE | `/categories/{id}`          | Delete a category            |
| GET    | `/categories/{id}/products` | Get products from a category |

### Authentication

| Method | Endpoint         | Description              |
| ------ | ---------------- | ------------------------ |
| POST   | `/auth/register` | Register a new user      |
| POST   | `/auth/login`    | Log in and receive a JWT |

### Cart

| Method | Endpoint           | Description                         |
| ------ | ------------------ | ----------------------------------- |
| GET    | `/cart`            | Get the authenticated user's cart   |
| POST   | `/cart/items`      | Add a product or increase quantity  |
| PATCH  | `/cart/items/{id}` | Replace an item's quantity          |
| DELETE | `/cart/items/{id}` | Remove an item from the user's cart |

All cart endpoints require a valid customer or administrator token. The user ID is
read from the JWT and is never accepted from the request body. Adding the same product
again increases its existing quantity. Quantities must be positive and cannot exceed
the product's current stock.

`GET` requests for products and categories are public. Creating, updating, and deleting
products or categories requires an administrator token in the `Authorization` header:

```text
Authorization: Bearer <token>
```

New users receive the `customer` role. To designate an administrator, update the user's
`role` to `admin` directly in the database during project setup.

## Project Structure

```text
src/
├── product/
│   ├── product_handler.rs
│   ├── product_repository.rs
│   └── mod.rs
├── category/
│   ├── category_handler.rs
│   ├── category_repository.rs
│   └── mod.rs
├── user/
│   ├── auth.rs
│   ├── password.rs
│   ├── user_handler.rs
│   ├── user_repository.rs
│   └── mod.rs
├── cart/
│   ├── cart_handler.rs
│   ├── cart_repository.rs
│   └── mod.rs
├── products_models.rs
├── category_models.rs
├── cart_models.rs
├── user_models.rs
└── main.rs
```

The project separates HTTP handlers, database access, models, authentication and password handling into separate modules.

## Running the Project

### Requirements

* Rust
* PostgreSQL

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://username:password@localhost/database
JWT_SECRET=your_secret_key
FRONTEND_ORIGIN=http://localhost:5173
```

`FRONTEND_ORIGIN` may contain a comma-separated allowlist. If it is omitted, the API
allows `http://localhost:5173` and `http://127.0.0.1:5173` for local Vite development.



Run the application:

```bash
cargo run
```

The API will be available at:

```text
http://127.0.0.1:3000
```

SQLx migrations are applied automatically when the application starts.

## Status

Work in progress.

Planned features include protected routes using JWT middleware, user-specific resources, and further marketplace functionality.
