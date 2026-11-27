pub mod codec;
pub mod error;

pub use codec::{decode_template, encode_template};
pub use error::Result;

/// The main entry point other crates will call.
#[allow(unused_variables)]
pub fn compile(source: &str) -> Result<String> {
    // let ast = syntax::parse(source)?;
    // let template = codegen::generate(&ast)?;
    // codec::encode_template(&template)
    todo!()
}