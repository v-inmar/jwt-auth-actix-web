# Actix-web (REST), Postgres, JWT

A small project showcasing how to use **Rust** as a backend REST API server using **Actix-web**. It is paired with **PostgreSQL** for persistence and **JSON Web Token** for **authorization/authentication**. **SQLx** is used for **asynchronous** communication with the database and for compile-time check. **Docker** is used to run the PostgreSQL container, however, it is still NOT utilizing **CI/CD**...yet(still working on this 😅).

## Objectives/Features
✅ Done/Completed/Implemented
⚠️ Need to implement/Still working on it

 - Actix-web (Rust)  as REST API backend ✅
 - Authorization and Authentication ✅
 - Communication with persistent storage ✅
 - Unit Testing ⚠️
 - Dockerize Actix-web + PostgreSQL ✅
 - CI/CD (partial - docker is already in use ⚠️)
 - Docs (including codeblock comments) ⚠️




# Partial Doc - How To Run
Must have .env file inside root directory that contains. Change the values as needed.
```sh
SERVER_HOST=0.0.0.0
SERVER_PORT=5000

DATABASE_URL=postgres://postgres:mysecretpassword@localhost:5432/appdb

JWT_ACCESS_SECRET=changeme123
JWT_REFRESH_SECRET=changemetoo123
ACCESS_TOKEN_EXPIRATION_MINUTES=5
REFRESH_TOKEN_EXPIRATION_DAYS=7
```

If running using **docker compose**, all the needed environment variables are inserted **docker-compose.yml** uses these credentials.

Also check notes.txt, it contains some useful commands

# How to run with docker
Make sure you have docker and docker compose installed
and run this command
```sh
docker compose up --build
```

to clean it up
```sh
docker compose down --volumes --remove-orphans
```

Docker will handle the networking between the too services (app and db)