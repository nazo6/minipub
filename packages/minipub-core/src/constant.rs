use once_cell::sync::OnceCell;

pub(crate) static DOMAIN: OnceCell<String> = OnceCell::new();
pub(crate) static BASE_URL: OnceCell<String> = OnceCell::new();
