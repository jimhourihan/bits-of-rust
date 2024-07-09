use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calc); // synthesized by LALRPOP
// mod etree;
mod value;
mod context;
mod ltree;
mod etree;
use etree::convert;

fn main () {
    let expr = calc::ExprParser::new()
        .parse("add( 1 + -5, 2 ^ 4 * 3 )")
        .unwrap();
    let e = convert(expr);
    match e {
	Ok(etree) => {
	    let v = etree.eval_root();
	    dbg!(v);
	}
	Err(msg) => {
	    println!("ERROR: {msg}");
	}
    }
}
