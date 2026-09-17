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

### Orders and inventory reservations

* Atomically create an order from the authenticated user's cart
* Reserve stock for 15 minutes while an order awaits payment
* Keep immutable product names and prices in order items
* Cancel orders and release their reserved stock
* Expire abandoned reservations in a background worker
* Simulate payment during development

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

### Orders

| Method | Endpoint                         | Description                              |
| ------ | -------------------------------- | ---------------------------------------- |
| POST   | `/checkout`                      | Create an order and reserve cart items   |
| GET    | `/orders`                        | List the authenticated user's orders     |
| GET    | `/orders/{id}`                   | Get an order with its immutable items    |
| POST   | `/orders/{id}/cancel`            | Cancel an order and release its stock    |
| POST   | `/orders/{id}/simulate-payment`  | Simulate successful payment locally      |

`POST /checkout` accepts an optional `Idempotency-Key` header (1–128 ASCII
characters). Repeating the request with the same key returns the original order
without reserving stock twice. A pending order reserves stock for 15 minutes.

`POST /orders/{id}/simulate-payment` is intentionally a development-only stub.
It must be removed or replaced by payment-intent creation and a verified provider
webhook before real payments are introduced. A browser client must never be trusted
to mark an order as paid.

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
├── order/
│   ├── order_handler.rs
│   ├── order_repository.rs
│   ├── order_service.rs
│   └── mod.rs
├── inventory/
│   ├── inventory_repository.rs
│   └── mod.rs
├── products_models.rs
├── category_models.rs
├── cart_models.rs
├── order_models.rs
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
