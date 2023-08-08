## country_state_city_api

A Rest API to get countries, states for country, and cities for state.
Build with [Rust](https://www.rust-lang.org).

### Installation

#### Prerequisites

* Install [Rust](https://www.rust-lang.org).
* Install [MySQL](https://www.mysql.com).

#### Database

1. Get the `world.sql` path from this repository.

2. Open MySQL.

~~~
mysql -u user -p password
~~~

3. Create a new database.

~~~
CREATE DATABASE databasename;
~~~

4. Use the created database,

~~~
USE databasename;
~~~

5. Run the SQL script.

~~~
source path/to/world.sql
~~~

### Configuration

#### Define environment variables

1. Create `.env` file at the root of this repository.

2. Define the following environment variables:
   * `DB_USER` Database username.
   * `DB_PASSWORD` Database user password.
   * `DB_DATABASE` Database name.
   * `PORT` Port where the app will run (Optional, default=8000).

### Running

1. Run the app with `cargo run`, it automatically will install all modules required.

2. Open in a browser `http://localhost:port_defined_in_dotenv_file`.
