use anyhow::{anyhow, Result};
use rambda::*;

fn main() -> Result<()> {
    let examples = [
        "(λx.x) y",                      // Identity function applied to y
        "(λf,x.f x) (λy.y) z",           // Self-application of identity function
        "λx,y.x y",                      // Apply first arg to second arg
        "(λx,y.x y) z w",                // Apply (λx.λy.x y) to z and w
        "(λn,f,x.f (n f x)) (λf,x.f x)", // Church numeral successor applied to one
        "(λx.(λy.x) y) z",               // No alpha reduction needed
        "(λx,y.x y) y",                  // Alpha reduction needed
        "(λx.λy.λy. x y) y",
        "(λx.λy.λx. x y) y",
    ];

    let mut lexer = Lexer::new();
    for example in examples {
        println!("{}", example);
        lexer.reload(example);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let parsed = parser
            .parse()
            .ok_or(anyhow!("Failed to parse expression"))?;

        let mut reduced = parsed;
        while !reduced.is_normal_form() {
            let (next_expr, reduction_type) = reduced.eval_step();
            println!("  ->{} {}", reduction_type, next_expr);
            reduced = next_expr;
        }
        let (simplified, reductions) = reduced.simplify_numbered_vars();
        let conversions: Vec<(String, String)> = reductions
            .iter()
            .filter_map(|red| {
                if let ReductionType::AlphaConversion(old, new) = red {
                    Some((old.clone(), new.clone()))
                } else {
                    None
                }
            })
            .collect();
        for (i, (from, to)) in conversions.iter().enumerate() {
            if i == 0 {
                print!("  ->α{{");
            }
            if i > 0 && i < conversions.len() - 1 {
                print!(", ");
            }
            print!("{} -> {}", from, to);
            if i == conversions.len() - 1 {
                println!("}} {}", simplified);
            }
        }
        println!("Final: {}", simplified);
        println!();
    }

    Ok(())
}
