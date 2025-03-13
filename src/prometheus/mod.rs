#[cfg(test)]
mod tests;

mod parsers;

mod mediamtx_annotator;

pub use parsers::parse_prometheus;
pub use parsers::parse_mediamtx_prometheus;
