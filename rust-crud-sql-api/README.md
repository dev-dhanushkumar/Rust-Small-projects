# Rust Warp Cookie based Authentication Sample code

This boilerplate application offers the following endpoints, with JWT role-based validation on most of them:

| Path | Method |
|------|--------|
| /api/auth/register | POST |
| /api/auth/login | POST |
| /api/articles_home | GET |
| /api/articles | GET |
| /api/articles/{url} | GET |
| /api/articles/updateHomeView/{id} | GET |
| /api/articles | POST |
| /api/articles | PUT |
| /api/articles/{id} | DELETE |
| /api/articles/comments | POST |
| /api/articles/comments/{article_id}/{comment_id} | DELETE |
| /api/users | GET |
| /api/users/{id} | GET |
| /api/users/updateHomeView/{id} | GET |
| /api/users | POST |
| /api/users | PUT |
| /api/users/{id} | DELETE |
| /api/users/changePassword | PUT |

<br />

The **.env** file contains the mongodb connection details and encryption keys.

<br />


## Running the Application
Run the application with the command:

    cargo run

Alternatively, you can run the command below to relaunch at any changes to the given resources:

    cargo watch -w src -w Cargo.toml -w .env -x run

<br />

### **Building the application**

#### Install sqlx-cli cargo dependency:

    cargo install sqlx-cli

Generate sqlx schema file required for "offline" builds (without need to reach DB on build time). This step is only required if any table schema is changed.

    cargo sqlx prepare

#### Run cargo build

    cargo build --release
