> Archived in favour of a monorepo: [clique-discord/clique](https://github.com/clique-discord/clique).

---

# Clique: Collector

This is the collector service for Clique. It is responsible for collecting
metadata on messages from Discord, and storing them in a PostgreSQL database.

## Setup

The service requires a PostgreSQL database to store messages in, and a Discord
bot token to recieve messages from Discord.

### 1. Set up the database

Install PostgreSQL if you haven't already, and create a database and user for
the service.

 1. [Install PostgreSQL.](https://www.postgresql.org/download/)
 2. Connect to the server as the `postgres` user (eg. `psql -U postgres`).
 3. Create a new database:

    `CREATE DATABASE clique;`
 4. Create a new user:

    `CREATE USER clique_collector WITH ENCRYPTED PASSWORD 'password-goes-here';`.
 5. Grant the user access to the database:

    `GRANT ALL ON DATABASE clique TO clique_collector;`

### 2. Get a Discord bot token

You'll need to create a Discord application with a bot in order for the service
to connect to Discord and recieve messages.

 1. [Go to the Discord Developer portal](https://discordapp.com/developers/applications)
    and press "New Application".
 2. Once you've created an application, go to "Bot" and create a bot.
 3. Copy the token.

### 3. Create the config file

The config file uses the INI format and contains the URL for the database, and
the token for the Discord bot.

 1. This should be a file named `config.ini`, in the directory where you will
    be running the service.
 2. Add the following to the file (replace the Discord token and Postgres
    password as appropriate):

    ```ini
    [clique-collector]
    discord_token = DISCORD.TOKEN.GOES.HERE
    postgres_url = postgres://clique_collector:password-goes-here-jpYU@localhost:5432/clique
    ```

### 4a. Download an executable

You can either download an executable (this step) or compile your own (the
next step). You may want to compile your own for security reasons, or if there
is not a pre-compiled executable for your operating system.

#### Linux

 1. Download the executable
    (or [click here](https://github.com/clique-discord/collector/releases/latest/download/clique-collector-linux)):

    `wget https://github.com/clique-discord/collector/releases/latest/download/clique-collector-linux -qO clique-collector`
 2. Make the file executable:

    `chmod +x clique-collector`

#### MacOS

 1. Download the executable
    (or [click here](https://github.com/clique-discord/collector/releases/latest/download/clique-collector-macos)):

    `wget https://github.com/clique-discord/collector/releases/latest/download/clique-collector-macos -qO clique-collector`
 2. Make the file executable:

    `chmod +x clique-collector`

#### Windows

 1. Download the executable with Power shell
    (or [click here](https://github.com/clique-discord/collector/releases/latest/download/clique-collector.exe)):

    `iwr -outf clique-collector.exe https://github.com/clique-discord/collector/releases/latest/download/clique-collector.exe`

### 4b. Compile an executable

If you don't want to use a pre-compiled executable, you can compile one using
Rust:

 1. [Install Rust](https://www.rust-lang.org/tools/install).
 2. Run `cargo build --release` in the same directory as this README.
 3. Your binary will be at `target/release/clique-collector`.

### 5. Run the service

This is as simple as executing the binary with your `config.ini` in the current
working directory, for example `./clique-collector`. You may also wish to set
it up with `systemd` or another process management system.

## Database Schema

The service will create a table name `messages` in the database, with the
following schema:

| Column      | Type     | Nullable? | Description                                               |
|:------------|:---------|:---:|:----------------------------------------------------------------|
| `id`        | `bigint` | no  | The Discord ID of the message. Primary key.                     |
| `guild`     | `bigint` | no  | The ID of the guild the message was sent in.                    |
| `author`    | `bigint` | no  | The ID of the author of the message.                            |
| `channel`   | `bigint` | no  | The ID of the channel the message was sent in.                  |
| `reply_to`  | `bigint` | yes | The ID of the author of the message this is a reply to, if any. |
| `timestamp` | `timestamp without time zone` | no | When the message was sent.                  |
