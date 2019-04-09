use std::collections::HashMap;

mod tokenizer;

pub mod aevum {
    struct Time {
        positive: bool;
        milliseconds: u16;
        seconds: u8;
        minutes: u8;
        hours: u32;
    }

    struct OptimizedTime {
        positive: bool;
        milliseconds: {
            value: u16;
            above: u32;
        };
        seconds: {
            value: u8;
            above: u32;
        };
        minutes: {
            value: u8;
            above: u32;
        };
        hours: {
            value: u32;
        }
    }

    struct Aevum {
        optimized_tokens: HashMap<char, Vec<tokenizer::Token>>;
    }

    fn optimize_tokens(tokens: Vec<Token>) -> HashMap<char, Vec<tokenizer::Token>> {

    }

    impl Aevum {
        pub fn new(format: &str) {
            let tokens = tokenizer::tokenize(format, 0);
            Aevum {
                optimized_tokens: optimize_tokens(tokens);
            }
        }
    }
}