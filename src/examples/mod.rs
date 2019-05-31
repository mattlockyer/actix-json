

pub mod tests;
pub use tests::api_test;

pub mod basic;
pub use basic::get_test;
pub use basic::post_test;
pub use basic::redirect;

pub mod proxy;
pub use proxy::proxy_test;
pub use proxy::proxy_streaming;

pub mod fs;
pub use fs::mock_get;
pub use fs::mock_set;

pub mod db;
pub use db::db_get;
pub use db::db_set;

pub mod mail;
pub use mail::mail_test;