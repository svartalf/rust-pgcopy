# pgcopy

Write data into a PostgreSQL `COPY WITH BINARY` format, fastest way to insert a lot of entries into database.

## Supported data types

    | PostgreSQL type          | Rust equivalent
----|--------------------------|-----------------
[x] | smallint                 | `i16`
[x] | integer                  | `i32`
[x] | bigint                   | `i64`
[ ] | decimal                  |
[ ] | numeric                  |
[x] | real                     | `f32`
[x] | double                   | `f64`
[ ] | char                     | `char`
[x] | char varying             | `&str`
[x] | text                     | `&str`
[x] | bytea                    | `&[u8]`
[x] | timestamp                | [chrono::naive::NaiveDateTime](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html)
[x] | timestamp with time zone | [chrono::DateTime](https://docs.rs/chrono/latest/chrono/struct.DateTime.html)
[x] | date                     | [chrono::Date](https://docs.rs/chrono/latest/chrono/struct.Date.html) or [chrono::naive::NaiveDate](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html)
[x] | time                     | [chrono::naive::NaiveTime](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html)
[ ] | interval                 |
[x] | boolean                  | `bool`
[ ] | cidr                     |
[ ] | inet                     |
[ ] | macaddr                  |
[ ] | macaddr8                 |
[x] | uuid                     | [uuid::Uuid](https://docs.rs/uuid/latest/uuid/struct.Uuid.html)
[ ] | xml                      |
[ ] | json                     |
[ ] | jsonb                    |
[ ] | array                    |
