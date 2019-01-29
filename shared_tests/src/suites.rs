//! Suites of tests applied across multiple crates

use std::mem::replace;
use super::*;


/// Basic test suite that checks the basic syntax and forms and does not
/// exercise macros/combiners nor extra types.
pub fn test_suite0<DA>(mut p: Parser<DefaultCharClassifier,
                                     DA,
                                     EmptyOperatorBindings>)
    where DA: DatumAllocator,
          <DA::TT as Text>::Chunk: From<&'static str>,
          DA::DR: Debug,
          <DA::TT as TextBase>::Pos: Debug,
{
    use Datum::{Combination, EmptyNest, List, EmptyList};
    use Error::*;

    let text = |val| Datum::Text(ExpectedText(val));

    let comb = |rator, rands|
                   Combination{operator: dr(rator), operands: dr(rands)};

    let list = |elem, next| List{elem: dr(elem), next: dr(next)};
    let list1 = |e1| list(e1, EmptyList);
    let list2 = |e1, e2| list(e1, list1(e2));
    let list3 = |e1, e2, e3| list(e1, list2(e2, e3));
    let list4 = |e1, e2, e3, e4| list(e1, list3(e2, e3, e4));
    let list5 = |e1, e2, e3, e4, e5| list(e1, list4(e2, e3, e4, e5));

    macro_rules! test {
        ($input:expr => [$($expected:expr),*])
            =>
        {test!($input => (assert_eq p) [$($expected),*])};

        ($input:expr =>! [$($expected:expr),*])
            =>
        {test!($input => (assert_ne p) [$($expected),*])};

        ($input:expr => ($parser:expr) [$($expected:expr),*])
            =>
        {test!($input => (assert_eq $parser) [$($expected),*])};

        ($input:expr =>! ($parser:expr) [$($expected:expr),*])
            =>
        {test!($input => (assert_ne $parser) [$($expected),*])};

        ($input:expr => ($ass:ident $parser:expr) [$($expected:expr),*])
            =>
        {//dbg!($input);
         $ass!(expect(vec![$($expected),*]),
               parse_all(&mut $parser,
                         DA::TT::from_str($input).iter()));};
    }

    // Basics
    test!("" => []);
    test!(" " => [Ok(text(" "))]);
    test!("  " => [Ok(text("  "))]);
    test!("a" => [Ok(text("a"))]);
    test!("a " => [Ok(text("a "))]);
    test!(" a" => [Ok(text(" a"))]);
    test!(" a " => [Ok(text(" a "))]);
    test!("xyz" => [Ok(text("xyz"))]);
    test!("a b" => [Ok(text("a b"))]);
    test!("a  b" => [Ok(text("a  b"))]);
    test!("a b c" => [Ok(text("a b c"))]);
    test!("   a  b c    " => [Ok(text("   a  b c    "))]);

    test!("{b}" => [Ok(comb(text("b"), EmptyList))]);
    test!("{b }" => [Ok(comb(text("b"), EmptyList))]);
    test!("{bob}" => [Ok(comb(text("bob"), EmptyList))]);
    test!("{b o b}" => [Ok(comb(text("b"), list1(text("o b"))))]);
    test!("{ bo b }" => [Ok(comb(text("bo"), list1(text("b "))))]);
    test!(" c  d   { e  f   g    }     hi  j "
          => [Ok(text(" c  d   ")),
              Ok(comb(text("e"), list1(text(" f   g    ")))),
              Ok(text("     hi  j "))]);

    test!("{}" => [Ok(EmptyNest)]);
    test!("{}{}" => [Ok(EmptyNest),
                     Ok(EmptyNest)]);
    test!("{{}}" => [Ok(comb(EmptyNest, EmptyList))]);
    test!("{{}{}}" => [Ok(comb(EmptyNest, list1(EmptyNest)))]);
    test!("{{{}}}" => [Ok(comb(comb(EmptyNest, EmptyList), EmptyList))]);
    test!(" { } " => [Ok(text(" ")),
                      Ok(EmptyNest),
                      Ok(text(" "))]);
    test!("  { {  }   } " => [Ok(text("  ")),
                              Ok(comb(EmptyNest, list1(text("  ")))),
                              Ok(text(" "))]);
    test!("   {    {   {  } }  }  " => [Ok(text("   ")),
                                        Ok(comb(comb(EmptyNest, EmptyList),
                                                list1(text(" ")))),
                                        Ok(text("  "))]);

    test!(r"\\" => [Ok(text(r"\"))]);
    test!(r"\{" => [Ok(text("{"))]);
    test!(r"\}" => [Ok(text("}"))]);
    test!(r"\{\}" => [Ok(text("{}"))]);
    test!(r"\a" => [Ok(text("a"))]);
    test!(r"\a\b" => [Ok(text("ab"))]);
    test!(r"\" => [Ok(text(""))]);
    test!(r"a\" => [Ok(text("a"))]);
    test!(r"a\b\" => [Ok(text("ab"))]);

    test!(r"{b\ o b}" => [Ok(comb(text("b o"), list1(text("b"))))]);
    test!(r"{\ bo b }" => [Ok(comb(text(" bo"), list1(text("b "))))]);
    test!(r"{\ bo\ b }" => [Ok(comb(text(" bo b"), EmptyList))]);
    test!(r"{\ bo\ b  }" => [Ok(comb(text(" bo b"), list1(text(" "))))]);
    test!(r"{\ }" => [Ok(comb(text(" "), EmptyList))]);
    test!(r"{\  }" => [Ok(comb(text(" "), EmptyList))]);
    test!(r"{\   }" => [Ok(comb(text(" "), list1(text(" "))))]);
    test!(r"{\ \ }" => [Ok(comb(text("  "), EmptyList))]);
    test!(r"{yz\ }" => [Ok(comb(text("yz "), EmptyList))]);
    test!(r"{yz\ \ }" => [Ok(comb(text("yz  "), EmptyList))]);
    test!(r"{yz\ \  \ }" => [Ok(comb(text("yz  "), list1(text(" "))))]);
    test!(r"{y\\z}" => [Ok(comb(text(r"y\z"), EmptyList))]);
    test!(r"{yz\}}" => [Ok(comb(text("yz}"), EmptyList))]);
    test!(r"{yz\{}" => [Ok(comb(text("yz{"), EmptyList))]);
    test!(r"{y\{z}" => [Ok(comb(text("y{z"), EmptyList))]);
    test!(r"{\{ yz}" => [Ok(comb(text("{"), list1(text("yz"))))]);
    test!("{\\\n}" => [Ok(comb(text("\n"), EmptyList))]);
    test!("{\\\t}" => [Ok(comb(text("\t"), EmptyList))]);
    test!("{\\\t\\\n}" => [Ok(comb(text("\t\n"), EmptyList))]);

    test!("{" => [Err(MissingEndChar)]);
    test!("}" => [Err(UnbalancedEndChar(PosIgnore))]);
    test!("␛{" => [Ok(text("␛")),
                   Err(MissingEndChar)]);
    test!("␛}" => [Err(UnbalancedEndChar(PosIgnore))]);

    test!("a b{\n{cd}  { }   { {e\re  {\tf}}\t   g  }\t hi \n j \t\t}k\nλ{ m{{}\r\r}o}\n"
          => [Ok(text("a b")),
              Ok(comb(comb(text("cd"), EmptyList),
                      list5(text(" "),
                            EmptyNest,
                            text("   "),
                            comb(comb(text("e"), list2(text("e  "),
                                                       comb(text("f"), EmptyList))),
                                 list1(text("   g  "))),
                            text("\t hi \n j \t\t")))),
              Ok(text("k\nλ")),
              Ok(comb(text("m"), list2(comb(EmptyNest, list1(text("\r"))), text("o")))),
              Ok(text("\n"))]);

    // TODO: A lot more

    // Custom delimiters

    let mut c = custom_delim::parser(p, custom_delim::Spec {
        nest_start: vec!['⟪'],
        nest_end: vec!['⟫'],
        nest_escape: vec!['␛'],
        whitespace: vec!['-'],
    });
    test!("" =>(c) []);
    test!("{}" =>(c) [Ok(text("{}"))]);
    test!("{a}" =>(c) [Ok(text("{a}"))]);
    test!("⟪⟫" =>(c) [Ok(EmptyNest)]);
    test!("⟪ ⟫" =>(c) [Ok(comb(text(" "), EmptyList))]);
    test!("⟪a⟫" =>(c) [Ok(comb(text("a"), EmptyList))]);
    test!("⟪ a ⟫" =>(c) [Ok(comb(text(" a "), EmptyList))]);
    test!("⟪a⟫" =>(c) [Ok(comb(text("a"), EmptyList))]);
    test!("⟪-a⟫" =>(c) [Ok(comb(text("a"), EmptyList))]);
    test!("⟪--a⟫" =>(c) [Ok(comb(text("a"), EmptyList))]);
    test!("⟪a-⟫" =>(c) [Ok(comb(text("a"), EmptyList))]);
    test!("⟪a--⟫" =>(c) [Ok(comb(text("a"), list1(text("-"))))]);
    test!("⟪-a-⟫" =>(c) [Ok(comb(text("a"), EmptyList))]);
    test!("⟪a-b⟫" =>(c) [Ok(comb(text("a"), list1(text("b"))))]);
    test!("⟪--a---b--⟫" =>(c) [Ok(comb(text("a"), list1(text("--b--"))))]);
    test!("a-⟪b-c⟫d-" =>(c) [Ok(text("a-")),
                             Ok(comb(text("b"), list1(text("c")))),
                             Ok(text("d-"))]);
    test!("␛␛" =>(c) [Ok(text("␛"))]);
    test!("␛⟪" =>(c) [Ok(text("⟪"))]);
    test!("␛⟫" =>(c) [Ok(text("⟫"))]);
    test!("␛⟪␛⟫" =>(c) [Ok(text("⟪⟫"))]);
    test!(r"\\" =>(c) [Ok(text(r"\\"))]);
    test!(r"\⟪\⟫" =>(c) [Ok(text(r"\")),
                         Ok(comb(text(r"\"), EmptyList))]);
    test!(r"\⟪" =>(c) [Ok(text(r"\")),
                       Err(MissingEndChar)]);
    test!(r"\⟫" =>(c) [Err(UnbalancedEndChar(PosIgnore))]);

    let mut c = custom_delim::parser(c, custom_delim::Spec {
        nest_start: vec!['⟪', '⟦'],
        nest_end: vec!['⟫', '⟧'],
        nest_escape: vec!['␛', '⃠'],
        whitespace: vec!['.', ':'],
    });
    test!("⟪⟫" =>(c) [Ok(EmptyNest)]);
    test!("⟦⟧" =>(c) [Ok(EmptyNest)]);
    test!("⟪⟧" =>(c) [Ok(EmptyNest)]);
    test!("⟦.:..::⟫" =>(c) [Ok(EmptyNest)]);
    test!("␛⃠" =>(c) [Ok(text("⃠"))]);
    test!("⃠␛" =>(c) [Ok(text("␛"))]);
    test!("⃠⟪␛⟫" =>(c) [Ok(text("⟪⟫"))]);
    test!(r"\⟦" =>(c) [Ok(text(r"\")),
                       Err(MissingEndChar)]);
    test!(r"\⟧" =>(c) [Err(UnbalancedEndChar(PosIgnore))]);

    // Parsing modes for Operatives and Applicatives. (This doesn't really
    // exercise combiners/macros, just does the bare minimum with them to test
    // the core parser's fixed modes for them.)

    struct BasicCombiners{o: &'static str, a: &'static str};

    impl<DA> OperatorBindings<DA> for BasicCombiners
        where DA: DatumAllocator,
              <DA::TT as Text>::Chunk: From<&'static str>,
    {
        type OR = Box<OpFn<DA::TT, DA::ET, DA::DR, <DA::TT as TextBase>::Pos, ()>>;
        type AR = Box<ApFn<DA::TT, DA::ET, DA::DR, <DA::TT as TextBase>::Pos, ()>>;
        type CE = ();

        fn lookup(&mut self, operator: &DA::DR) -> Option<Combiner<Self::OR, Self::AR>>
        {
            let just_operands = |_operator, mut operands| {
                // Note: This `unwrap` won't ever fail because these datum
                // references are never shared.
                Ok(replace(DerefTryMut::get_mut(&mut operands).unwrap(),
                           EmptyNest))
            };

            if let Datum::Text(text) = &**operator {
                if text.partial_eq(&DA::TT::from_str(self.o)) {
                    return Some(Combiner::Operative(Box::new(just_operands)))
                } else if text.partial_eq(&DA::TT::from_str(self.a)) {
                    return Some(Combiner::Applicative(Box::new(just_operands)))
                }
            }
            None
        }
    }

    let mut c = Parser {
        classifier: DefaultCharClassifier,
        allocator: c.allocator,
        bindings: BasicCombiners{o: "oo", a: "aa"},
    };
    // Operatives get all the text to the end of the nest form unbroken
    // regardless if any of it looks like other nest forms.
    test!("{oo}" =>(c) [Ok(text(""))]);
    test!("{oo }" =>(c) [Ok(text(""))]);
    test!("{oo  }" =>(c) [Ok(text(" "))]);
    test!("{oo{}}" =>(c) [Ok(text("{}"))]);
    test!("{oo zab {zz} yo}" =>(c) [Ok(text("zab {zz} yo"))]);
    test!("{\n oo  {\n zab {{} yo}}}" =>(c) [Ok(text(" {\n zab {{} yo}}"))]);
    test!("{u {oo zab {zz} yo}}" =>(c) [Ok(comb(text("u"),
                                                list1(text("zab {zz} yo"))))]);
    test!("{oo {}" =>(c) [Err(MissingEndChar)]);
    test!("{oo {" =>(c) [Err(MissingEndChar)]);
    test!("{oo}}" =>(c) [Ok(text("")),
                         Err(UnbalancedEndChar(PosIgnore))]);
    // Applicatives get a list of the parsed operands.
    test!("{aa}" =>(c) [Ok(EmptyList)]);
    test!("{aa }" =>(c) [Ok(EmptyList)]);
    test!("{aa  }" =>(c) [Ok(list1(text(" ")))]);
    test!("{aa{}}" =>(c) [Ok(list1(EmptyNest))]);
    test!("{aa zab {zz} yo}" =>(c) [Ok(list3(text("zab "),
                                             comb(text("zz"), EmptyList),
                                             text(" yo")))]);
    test!("{\n aa  {\n zab {{} yo}}}"
          =>(c) [Ok(list2(text(" "), comb(text("zab"),
                                          list1(comb(EmptyNest,
                                                     list1(text("yo")))))))]);
    test!("{u {aa zab {zz} yo}}"
          =>(c) [Ok(comb(text("u"), list1(list3(text("zab "),
                                                comb(text("zz"), EmptyList),
                                                text(" yo")))))]);
    test!("{aa {}" =>(c) [Err(MissingEndChar)]);
    test!("{aa {" =>(c) [Err(MissingEndChar)]);
    test!("{aa}}" =>(c) [Ok(EmptyList),
                         Err(UnbalancedEndChar(PosIgnore))]);
}

// TODO: Suite for Parsers that provide character position.

// TODO: Suite for Parsers that provide UTF-8 byte position.
