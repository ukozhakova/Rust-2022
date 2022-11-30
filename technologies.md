# Technologies to use

Only **required** tech to use is:

- [SQLx](https://crates.io/crates/sqlx): recommended crate for Rust-to-database communication

We do not want to pressure you into using a specific technology for everything (other than SQLx and Rust itself), here is the recommended list of technologies though:

- [PostgreSQL](https://www.postgresql.org/): open-source DBMS, featured in the [Postgres container image](https://hub.docker.com/_/postgres)
- [Docker](https://www.docker.com/)/[Podman](https://podman.io/): platform for containerization
  - Compose ([docker version](https://docs.docker.com/compose/)/[podman version](https://github.com/containers/podman-compose)): simple tools for basic container orchestration
- [dotenvy](https://crates.io/crates/dotenvy): loading the `.env` file
- [PlantUML](https://plantuml.com/ie-diagram): markup language for text representation of multiple diagrams, we will use it to create entity relationship diagrams
- [async\_trait](https://crates.io/crates/async-trait): adds the support of asynchronous traits.
- [chrono](https://crates.io/crates/chrono) (optionally, you can use [time](https://crates.io/crates/time) instead for similar functionality): library often used for datetime management
- [anyhow](https://crates.io/crates/anyhow):

  Allows working with multiple error types.
  
  **Note:** in code which is intended to be used as a library in other projects where the consuments are other developers, please, **do not use** anyhow to handle errors. Either use pre-defined ones, or create your own, which will undisputably cover various error cases that can occurr in your code. For the purpose of this iteration, it is fine, as error handling is not the point of this assignment, therefore we only care about "catching" the error, not about its origin or its "proper" handling.
