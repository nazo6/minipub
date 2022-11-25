use const_format::formatcp;

pub(crate) const DOMAIN: &str = env!("DOMAIN");
pub(crate) const BASE_URL: &str = formatcp!("https://{DOMAIN}");
