use sqlite::Connection;
pub struct Buffer{
    connection: Connection
}
impl Buffer{
    pub fn new() -> Buffer {
        // let connection = sqlite::Connection::open("./buffer.sqlite");
        let connection = sqlite::open(":memory:").unwrap();

        Buffer { connection: connection }
    }
}
