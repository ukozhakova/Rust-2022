# Containerization

To work with this assignment, a basic knowledge of Docker / Podman is necessary. This summary helps you only with the basics.

We want to use the databases dynamically and have multiple instances ready at our disposal. Containerization helps us solve this issue.

For the provided `compose.yml` file to work correctly, you have to:

1. Install [Docker](https://docs.docker.com/get-docker/) / [Podman](https://podman.io/getting-started/installation). **You can also install Docker desktop to avoid working with the command line**.
2. Install the Compose for [Docker](https://docs.docker.com/compose/install/) / for [Podman](https://github.com/containers/podman-compose).
3. Run the compose file.

## Running the compose file

> Note: to run the Podman variant, simply change the `docker-` prefix to `podman-` in the following examples, and everything should work out of the box.

Change the directory to the root of the homework (`hw_01`) and run the following:

### Running for the first time:

This creates and then runs the container group. The `-d` parameter makes the container run in the background.

```sh
docker-compose up -d
```

### Starting the already existing compose:

```sh
docker-compose start
```

### Seeing whether the container group is running:

```sh
docker-compose ps -a
```

### Stopping the container group

```sh
docker-compose stop
```

Useful when you don't want the container to start running, for example, right after the system startup.

### Destroying the container group (mainly the database volume - remove persistent storage)

```sh
docker-compose down --remove-orphans --volumes
```

### I cannot get containerization to work

If, for whatever reason, you cannot get containerization to work on your machine, contact your tutor or [Tomáš Sedláček](mailto:tomas.sedlacek@mail.muni.cz).

As a last resort, you can use the faculty database. Containerization is forbidden by the faculty, so this is the only choice left. The faculty provides you with [one running instance of a Postgres database](https://fadmin.fi.muni.cz/auth/sys/ucty_db.mpl).

> Note: in this case, you will not be running the database locally at all. The database runs on faculty hardware. To manage that database, we suggest using `psql` and the appropriate `postgresql-client`.

## Managing the database

To peek into the database running in the container, you can use the provided `adminer` container, which is already defined in the compose.

The container is accessible via the `localhost:8080`, and if you don't change your environment values, you can connect to the database like so:

![Adminer connection](../database-homework/docs/adminer-config.png)
