use uuid::Uuid;

use crate::types::PgUuid;


impl PgUuid for Uuid {
    fn to_uuid(&self) -> &[u8] {
        self.as_bytes()
    }
}
