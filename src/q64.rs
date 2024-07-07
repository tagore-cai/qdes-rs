use base64::{prelude::BASE64_STANDARD, Engine as _};

pub  fn  decode_base64(s: String){
  BASE64_STANDARD.decode(s).unwrap();
}