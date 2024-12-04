use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys;

mod inner {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::js_sys;

    #[wasm_bindgen(module = "/src/sql.js")]
    extern "C" {
        #[wasm_bindgen(js_name = Database)]
        pub type Database;

        #[wasm_bindgen]
        pub async fn initSql(data: js_sys::Uint8Array) -> Database;

        #[wasm_bindgen(method)]
        pub fn run(this: &Database, sql: &str);

        #[wasm_bindgen(method)]
        pub fn exec(this: &Database, sql: &str) -> JsValue;

        #[wasm_bindgen(method)]
        pub fn export(this: &Database) -> Vec<u8>;
    }
}

/// Sqlite db wrapper
#[wasm_bindgen]
pub struct SqlDatabase {
    db: inner::Database,
}

#[wasm_bindgen]
impl SqlDatabase {
    /// Init db with data, if data is empty just new an empty db
    pub async fn init_with_data(data: Vec<u8>) -> Result<SqlDatabase, JsValue> {
        let data_js: JsValue = data.into();
        let db = inner::initSql(js_sys::Uint8Array::new(&data_js)).await;

        Ok(SqlDatabase { db })
    }

    /// Run sql
    pub fn run(&self, sql: &str) {
        self.db.run(sql)
    }

    /// Exec sql and then return result
    pub fn exec(&self, sql: &str) -> JsValue {
        self.db.exec(sql)
    }

    /// Export all db to u8 array
    pub fn export(&self) -> Vec<u8> {
        self.db.export()
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn greet() {
    set_panic_hook();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    let db = SqlDatabase::init_with_data(Vec::new()).await.unwrap();
    // Test database
    log::info!("create table (id INTEGER PRIMARY KEY, name TEXT)");
    db.exec("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)");
    log::info!("insert table users (name) VALUES ('John')");
    db.exec("INSERT INTO users (name) VALUES ('John')");

    let result = db.exec("SELECT * FROM users");

    log::info!("exec select * from users: {:?}", result);
}
