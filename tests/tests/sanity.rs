use jrsonnet_evaluator::{error::Result, throw, State, Val};
use jrsonnet_stdlib::StateExt;

mod common;

#[test]
fn assert_positive() -> Result<()> {
	let s = State::default();
	s.with_stdlib();

	let v = s.evaluate_snippet("snip".to_owned(), "assert 1 == 1: 'fail'; null")?;
	ensure_val_eq!(v, Val::Null);
	let v = s.evaluate_snippet("snip".to_owned(), "std.assertEqual(1, 1)")?;
	ensure_val_eq!(v, Val::Bool(true));

	Ok(())
}

#[test]
fn assert_negative() -> Result<()> {
	let s = State::default();
	s.with_stdlib();

	{
		let Err(e) = s.evaluate_snippet("snip".to_owned(), "assert 1 == 2: 'fail'; null") else {
			throw!("assertion should fail");
		};
		let e = s.stringify_err(&e);
		ensure!(e.starts_with("assert failed: fail\n"));
	}
	{
		let Err(e) = s.evaluate_snippet("snip".to_owned(), "std.assertEqual(1, 2)") else {
			throw!("assertion should fail")
		};
		let e = s.stringify_err(&e);
		ensure!(e.starts_with("runtime error: Assertion failed. 1 != 2"))
	}

	Ok(())
}