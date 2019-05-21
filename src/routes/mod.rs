
pub mod static_examples;
pub use static_examples::get_test;
pub use static_examples::post_test;
pub use static_examples::redirect;

pub mod fs_examples;
pub use fs_examples::mock_get;
pub use fs_examples::mock_set;

pub mod db_examples;
pub use db_examples::db_get;
pub use db_examples::db_set;