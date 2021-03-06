#![feature(rustc_private,plugin)]
#![plugin(rustlex)]

#[allow(plugin_as_library)]
extern crate rustlex;

#[macro_use] extern crate log;

use std::io::BufReader;

use self::Token::{TokOuterStuff, TokInnerStuff};

#[derive(PartialEq,Debug)]
pub enum Token {
    TokOuterStuff(String),
    TokInnerStuff(String)
}

rustlex! ConditionLexer {
    let OPEN = '{';
    let CLOSE = '}';
    let STUFF = [^'{''}']*;
    INITIAL {
        STUFF => |lexer: &mut ConditionLexer<R>|
            Some(TokOuterStuff(lexer.yystr().trim().to_string()))
        OPEN => |lexer: &mut ConditionLexer<R>| -> Option<Token> {
            lexer.INNER();
            None
        }
    }
    INNER {
        STUFF => |lexer: &mut ConditionLexer<R>|
            Some(TokInnerStuff(lexer.yystr().trim().to_string()))
        CLOSE => |lexer: &mut ConditionLexer<R>| -> Option<Token> {
            lexer.INITIAL();
            None
        }
    }
}

#[test]
fn test_conditions() {
    let expected = vec!(TokOuterStuff("outer".to_string()),
                        TokInnerStuff("inner".to_string()));
    let str = "outer { inner }";
    let inp = BufReader::new(str.as_bytes());
    let lexer = ConditionLexer::new(inp);
    let mut iter = expected.iter();
    for tok in lexer {
        assert_eq!(iter.next().unwrap(), &tok);
    }
    assert_eq!(iter.next(), None);
}
