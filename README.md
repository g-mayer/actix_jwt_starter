
# Getting Started with actix-web, Diesel, and JWts

1. Install Rust and Cargo
    - On Windows: Follow this [guide](https://homepage.cs.uri.edu/faculty/hamel/courses/home/csc301/rust-installation-windows.pdf) for Rust installation.
        - On Windows you might be required to use Rust Nightly builds to compile the code. 
            ```
            rustup default nightly
            cargo clean
            cargo build
            ```
    - On Mac: Refer to the [official Rust installation instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html) for macOS.

2. PostgreSQL Installation
    - Ensure that you have PostgreSQL version 14 installed on your machine.
        - On Windows: Download and install the PostgreSQL installer [from here](https://www.postgresql.org/download/windows/). Double check Windows Defender is allowing local access to port 5432 for your database.
        - On Mac: Use Homebrew to install PostgreSQL 14 with `brew install postgresql@14`.

3. OpenSSL Installation and Configuration
    - On Windows:
        - Install OpenSSL using Chocolatey with `choco install openssl`.
        - Set the `OPENSSL_DIR` environment variable to the appropriate directory path.
    - On Mac:
        - Install OpenSSL with Homebrew using `brew install openssl`.

4. Install `diesel-cli`
    - Install `diesel-cli` by running the following command:
        ```
        cargo install diesel_cli --no-default-features --features postgres
        ```
    - Learn about diesel and `diesel migration run` [here](https://diesel.rs/guides/getting-started)
5. Install `cargo-watch`
    - Install `cargo-watch` by running the following command:
        ```
        cargo install cargo-watch
        ```
    - This tool enables automatic server restarts when your code changes. To start the development server with auto-restart, use the following command:
        ```
        cargo watch -x run
        ```

To improve the syntax and clarity of the motivation section in your README, consider restructuring it for better readability and emphasis. Here's a revised version:

## Setting Environment Variables

Before running the application, set the following environment variables:

1. `ENVIRONMENT`: 
   - Set this to “development” to enable the seed database route and disable CORS. 
   - This variable is crucial for differentiating between production and development environments.

2. `JWT_SECRET`: 
   - This is the secret key used for generating JSON Web Tokens (JWTs). Ensure it's set to a secure, random value.

3. `DATABASE_URL`: 
   - Provide a PostgreSQL URL pointing to your production or development database, depending on the environment.

## Development Commands

1. **Run in Development Mode**:
   - Use `cargo watch` to automatically rebuild and restart the server upon code changes:
     ```
     cargo watch -x run
     ```
   - Learn more about `cargo-watch` [here](https://docs.rs/crate/cargo-watch/3.2.0).

2. **Building the Project**:
   - To compile the project, use:
     ```
     cargo build
     ```

3. **Running the Project**:
   - To run the project without live reloading, simply use:
     ```
     cargo run
     ```

5. **Running Docker for Production**:
   - Execute the following command to run Docker, optionally include `--build` to rebuild all files [learn more](https://docs.docker.com/compose/):
      ```
      docker-compose up
      ```
      
---

# Motivation

This project is an excellent starting point for anyone interested in building a server using Rust. It serves as a comprehensive example, showcasing the integration and usage of various technologies and concepts in a Rust-based API. Whether you're transitioning from a different tech stack like Node or enhancing your existing Rust skills, this project can be a valuable resource. Key features and technologies covered include:

- **JWTs (JSON Web Tokens):** Demonstrating secure authentication mechanisms.
- **Middleware:** Showcasing how to manage HTTP requests and responses effectively.
- **Actix-web:** Utilizing this powerful web framework for building efficient web services.
- **OpenAPI with SwaggerUI:** Integrating API documentation and testing tools.
- **Diesel and Diesel-cli:** Exploring ORM and database migrations. Use custom types in your database schema.
- **Custom API Errors:** Managing error handling gracefully.
- **Logging:** Tracking and recording application behavior.
- **Docker:** Facilitating easy deployment and environment consistency.

This project aims to provide a robust template and learning guide for building your own Rust API, especially if you're more familiar with other technologies like Node.js.

---

### Security

#### Current JWT Implementation
This API utilizes JSON Web Tokens (JWTs) for authentication. The current implementation is basic and serves as a starting point for securing the API. Key points to note:

- **Token Lifetime**: JWTs are set to expire after 30 days. This duration might not be ideal for all use cases, and should be adjusted based on your specific security requirements.
- **No Refresh Tokens**: At present, this implementation does not use refresh tokens. Once the JWT expires, users will need to re-authenticate to obtain a new token.

#### Recommendations for Enhanced Security

1. **Shorter Token Lifetimes**:
   - Consider reducing the JWT lifetime. Shorter lifetimes reduce the window of opportunity for token misuse, but require more frequent token renewal. A common practice is to use a shorter-lived access token (e.g., 15 minutes) and a longer-lived refresh token.

2. **Implement Refresh Tokens**:
   - Introduce refresh tokens to handle token renewal securely. Refresh tokens can be stored in a secure, HttpOnly cookie and used to issue new access tokens. This approach also allows for token revocation.

3. **Invalid Token Tracking**:
   - Implement an `invalid_tokens` table in your database. Store tokens that have been explicitly logged out or invalidated for other reasons. Check against this table to ensure a token is still valid.

4. **Access and Refresh Token Rotation**:
   - On each use of a refresh token, issue a new access and refresh token pair, and invalidate the old refresh token. This practice helps mitigate the risk of refresh token theft.

5. **Understand JWT Limitations**:
   - JWTs are stateless and cannot be invalidated or updated without some form of state management (like an `invalid_tokens` table). Understand the security implications of using JWTs, especially in high-security contexts.

6. **Secure Token Storage and Transmission**:
   - Ensure tokens are stored and transmitted securely. Use HTTPS to protect tokens in transit, and consider HttpOnly cookies for storing tokens to prevent access from JavaScript.

---

### Additional Notes
🌟 Please star this repo if it helped you! If it didn't help or you ran into problems let me know so I can update the project.
- Adjust the JWT configuration and authentication mechanisms to align with your application's security needs and user expectations. 
- Utoipa + SwaggerUI typically orders your routes based on when they're defined in your file. However, I've found that they are ordered alphabetically on the route path given to Utoipa.
- If you are on Windows you might need to debug your database connections and ensure your firewall isn't blocking the connections between the API and the database. This will typically cause an exit error (3) without any other logs or information.
- You can set `RUST_BACKTRACE=full` in your .env file to get better debugging information when using rust-analyzer in VScode.
- This API is designed to run behind an NGINX instance, however, you can configure OpenSSL yourself using actix-web.
- Instead of copying the .env file into the Docker container, environment variables are set in the `docker-compose.yml` file so any .env changes need to be reflected there for the production environment.
---
