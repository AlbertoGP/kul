use std::iter::FromIterator;

use kul::{
    Parser, Datum, Text as _,
    parser::CharClassifier,
    common::inmem
};


pub struct CustomCharClassifier;

impl CharClassifier for CustomCharClassifier {
    #[inline]
    fn is_nest_start(&self, c: char) -> bool {
        '⟪' == c
    }

    #[inline]
    fn is_nest_end(&self, c: char) -> bool {
        '⟫' == c
    }

    #[inline]
    fn is_nest_escape(&self, c: char) -> bool {
        '␛' == c
    }

    #[inline]
    fn is_whitespace(&self, c: char) -> bool {
        c.is_whitespace()
    }
}


fn main() {
    let input = r#"
Using non-default delimiters:

⟪⟪source-code Rust⟫
    use kul::common::inmem::parse_str;

    fn main() {
        let input = "Escaped the {bold non-default} delimiters: ␛⟪, ␛⟫, ␛␛";
        dbg!(parse_str(input));
    }
⟫
"#;
    let mut parser = Parser {
        classifier: CustomCharClassifier,
        allocator: inmem::DatumAllocator::<'_, ()>::default(),
        bindings: inmem::OperatorBindings::<'_, _, ()>::default(),
    };
    let ast = parser.parse(inmem::Text::from_str(input).iter()).collect::<Vec<_>>();
    dbg!(&ast);

    if let Ok(Datum::Combination{operands, ..}) = &ast[1] {
        if let Datum::List{elem, ..} = &**operands {
            if let Datum::Text(text) = &**elem {
                let selected = String::from_iter(text.chars());
                println!("\nSelected text:\n{}", selected);
            }
        }
    }
}
