use jrsonnet_evaluator::{function::builtin, IStr};

#[builtin]
pub fn builtin_md5(s: IStr) -> String {
	format!("{:x}", md5::compute(s.as_bytes()))
}

#[builtin]
pub fn builtin_sha256(s: IStr) -> String {
	use sha2::digest::Digest;
	format!("{:x}", sha2::Sha256::digest(s.as_bytes()))
}

#[builtin]
pub fn builtin_sha512(s: IStr) -> String {
	use sha2::digest::Digest;
	format!("{:x}", sha2::Sha512::digest(s.as_bytes()))
}
