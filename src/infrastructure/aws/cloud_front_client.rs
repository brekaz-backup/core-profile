use std::fs;
use cloudfront_sign::{get_signed_url, SignedOptions};
use lazy_static::lazy_static;


lazy_static! {
    static ref AWS_CLOUDFRONT_PK: String = fs::read_to_string(std::env::var("AWS_CLOUDFRONT_PK")
        .expect("Can't read AWS_CLOUDFRONT_PK")).expect("Can't read file");
    static ref AWS_CLOUDFRONT_ID: String = std::env::var("AWS_CLOUDFRONT_ID").expect("Can't read AWS_CLOUDFRONT_ID");
    static ref AWS_CLOUDFRONT_DOMAIN: String =
        std::env::var("AWS_CLOUDFRONT_DOMAIN").expect("Can't read AWS_CLOUDFRONT_DOMAIN");
}


pub struct CloudFrontSigner;


impl CloudFrontSigner {
    pub fn sing(key: Option<String>) -> Option<String> {
        let options = SignedOptions {
            key_pair_id: AWS_CLOUDFRONT_ID.clone(),
            private_key: AWS_CLOUDFRONT_PK.clone(),
            ..Default::default()
        };

        if let Some(file_key) = key {
            return get_signed_url(&format!("{}/{}", AWS_CLOUDFRONT_DOMAIN.clone(), file_key), &options).ok();
        }

        None
    }
}