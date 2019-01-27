# pgcopy

Write data into a PostgreSQL `COPY WITH BINARY` format, fastest way to insert a lot of entries into database.

## Supported data types

|   | PostgreSQL type          | Rust equivalent 
|---|--------------------------|-----------------
| ✔ | smallint                 | `i16`
| ✔ | integer                  | `i32`
| ✔ | bigint                   | `i64`
|   | decimal                  |
|   | numeric                  |
| ✔ | real                     | `f32`
| ✔ | double                   | `f64`
| ✔ | char                     | `char`
| ✔ | char varying             | `&str`
| ✔ | text                     | `&str`
| ✔ | bytea                    | `&[u8]`
| ✔ | timestamp                | [chrono::naive::NaiveDateTime](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html)
| ✔ | timestamp with time zone | [chrono::DateTime](https://docs.rs/chrono/latest/chrono/struct.DateTime.html)
| ✔ | date                     | [chrono::Date](https://docs.rs/chrono/latest/chrono/struct.Date.html) or [chrono::naive::NaiveDate](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html)
| ✔ | time                     | [chrono::naive::NaiveTime](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html)
|   | interval                 |
| ✔ | boolean                  | `bool`
|   | cidr                     |
|   | inet                     |
|   | macaddr                  |
|   | macaddr8                 |
| ✔ | uuid                     | [uuid::Uuid](https://docs.rs/uuid/latest/uuid/struct.Uuid.html)
|   | xml                      |
|   | json                     |
|   | jsonb                    |
|   | array                    |
