use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calc); // synthesized by LALRPOP
// mod etree;
mod value;
mod context;
mod ltree;

fn main () {
    let expr = calc::ExprParser::new()
        .parse("add( 1 + -5, '(2 ^ x * 3) )")
        .unwrap();
    dbg!(expr);
}
