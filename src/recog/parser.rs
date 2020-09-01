#[derive(Parser)]
#[grammar = "grammar/llsp.pest"]
pub struct LLSPParser;

#[cfg(test)]
mod tests {
    use super::{LLSPParser, Rule};
    use pest::Parser;

    #[test]
    fn number() {
        parses_to! {
            parser: LLSPParser,
            input: "-114514",
            rule: Rule::Number,
            tokens: [
                Number(0, 7)
            ]
        }

        parses_to! {
            parser: LLSPParser,
            input: "1919710",
            rule: Rule::Number,
            tokens: [
                Number(0, 7)
            ]
        }
    }

    #[test]
    #[should_panic]
    fn failed_number() {
        parses_to! {
            parser: LLSPParser,
            input: "0x3F3F3F",
            rule: Rule::Number,
            tokens: []
        }
    }

    #[test]
    fn sym() {
        parses_to! {
            parser: LLSPParser,
            input: "HoshinoChanWatashiWaAnatanoFanDesuuuuuuuuuuuu!",
            rule: Rule::Symbol,
            tokens: [
                Symbol(0, 46)
            ]
        }
    }

    #[test]
    #[should_panic]
    fn failed_sym() {
        parses_to! {
            parser: LLSPParser,
            input: "Symbol cannot has WHITESPACE!",
            rule: Rule::Symbol,
            tokens: []
        }
    }

    #[test]
    fn list() {
        parses_to! {
            parser: LLSPParser,
            input: "'(quoted)",
            rule: Rule::Quoted,
            tokens: [
                Quoted(0, 9, [
                    list(1, 9, [
                        Literal(2, 8, [
                            Symbol(2, 8)
                        ])
                    ])
                ])
            ]
        }

        parses_to! {
            parser: LLSPParser,
            input: "(define (kusai _unused) 114514)",
            rule: Rule::list,
            tokens: [
                list(0, 31, [
                    Literal(1, 7, [
                        Symbol(1, 7)
                    ]),

                    Literal(8, 23, [
                        list(8, 23, [
                            Literal(9, 14, [
                                Symbol(9, 14)
                            ]),

                            Literal(15, 22, [
                                Symbol(15, 22)
                            ])
                        ])
                    ]),

                    Literal(24, 30, [
                        Number(24, 30)
                    ])
                ])
            ]
        }
    }

    #[test]
    fn literal() {
        parses_to! {
            parser: LLSPParser,
            input: r#""Hello, world!\n""#,
            rule: Rule::String,
            tokens: [
                String(0, 17)
            ]
        }

        parses_to! {
            parser: LLSPParser,
            input: r#"'\\'"#,
            rule: Rule::Char,
            tokens: [
                Char(0, 4)
            ]
        }
    }

    #[test]
    fn code() {
        let code = r#"(define (addOne (x ：i32)) (+ x 1))
(define (mulTwo x) (* x 2))      ;; x wasn't typed explicitly, so when someone invokes it, the compiler will automatically cast x into the most suitable type

(emit "(5 + 1) * 2 = {}" (stringify (mulTwo (addOne 5))))
"#;

        let pairs = LLSPParser::parse(Rule::llsp, code).unwrap();

        println!("{:#?}", pairs);
    }
}