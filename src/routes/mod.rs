

pub mod tests;
pub use tests::api_test;

pub mod static_examples;
pub use static_examples::get_test;
pub use static_examples::post_test;
pub use static_examples::redirect;

pub mod proxy_examples;
pub use proxy_examples::proxy_test;
pub use proxy_examples::proxy_streaming;

pub mod fs_examples;
pub use fs_examples::mock_get;
pub use fs_examples::mock_set;

pub mod db_examples;
pub use db_examples::db_get;
pub use db_examples::db_set;

pub mod mail_examples;
pub use mail_examples::mail_test;