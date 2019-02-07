# pgcopy

[![Latest Version](https://img.shields.io/crates/v/pgcopy.svg)](https://crates.io/crates/pgcopy)
[![Latest Version](https://docs.rs/pgcopy/badge.svg)](https://docs.rs/pgcopy)

Write data into a PostgreSQL `COPY WITH BINARY` format, somewhat faster way to insert a lot of entries into database
that plain text or CSV.

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
| ✔ | macaddr                  | `[u8; 6]` or [eui48::MacAddress](https://docs.rs/eui48/latest/eui48/struct.MacAddress.html)
| ✔ | macaddr8                 | `[u8; 6]`, `[u8; 8]` or [eui48::MacAddress](https://docs.rs/eui48/latest/eui48/struct.MacAddress.html)
| ✔ | uuid                     | `[u8; 16]` or [uuid::Uuid](https://docs.rs/uuid/latest/uuid/struct.Uuid.html)
|   | xml                      |
|   | json                     |
|   | jsonb                    |
|   | array                    |
