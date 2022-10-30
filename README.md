## country_state_city_api

A Rest API to get countries, states for country, and cities for state.
Build with [Rust](https://www.rust-lang.org) for practicing.

### Installation

#### Database
1. Get the `world.sql` path from this repository.

2. Open MySQL client.

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
mysql> source path/to/world.sql
~~~