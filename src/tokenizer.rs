pub mod tokenizer {
    struct Token {
        is_text: bool;
        text_content: String;
        optional: bool;
        type: char;
        length: u8;
        format: Vec<Token>;
    }

    enum State {
        STRING,
        ESCAPE_STRING,
        ESCAPE_OPTIONAL_FORMAT,
        IN_FORMAT,
        IN_OPTIONAL_TYPE,
        IN_OPTIONAL_FORMAT,
    }

    let ESCAPE = '\\';
    let FORMAT_START = '[';
    let FORMAT_END = ']';
    let OPTIONAL_START = '(';
    let OPTIONAL_END = ')';
    let OPTIONAL_DEF_END = ':';

    pub fn tokenize(input: &str, start_index: u32 = 0) -> Vec<Token>{
        let mut tokens: Vec<Token> = Vec::new();
        let mut state: State = State::STRING;
        let mut character_index: u32 = 0;

        let mut build = String::new();
        let mut is_token_optional = false;
        let mut token_type = '';
        let mut processing_character = '';

        for c in input.chars() {
            match state {
                State::ESCAPE_STRING => {
                    state = State::STRING;
                    build.push(c);
                },

                State::ESCAPE_OPTIONAL_FORMAT => {
                    state = State::IN_OPTIONAL_FORMAT;
                    // Add the escape as well, since the build is getting tokenized
                    // a second time and woudn't get escaped the second time then.
                    build.push(ESCAPE);
                    build.push(c);
                },

                State::STRING => {
                    if (c == ESCAPE) {
                        state = State::ESCAPE_STRING;
                    } else if (c == OPTIONAL_START || c == FORMAT_START) {
                        if (!build.is_empty()) {
                            tokens.push(Token {
                                is_text: true,
                                text_content: build.copy(),
                                optional: false,
                                type: '',
                                length: 0,
                                format: Vec::new(),
                            });
                        }
                        build.clear();

                        if (c == OPTIONAL_START) {
                            if (start_index > 0) {
                                // TODO: Throw error
                            }
                            state = State::IN_OPTIONAL_TYPE;
                            is_token_optional = true;
                        } else {
                            state = State::IN_FORMAT;
                            is_token_optional = false;
                        }
                    } else {
                        build.push(c);
                    }
                },

                State::IN_FORMAT => {
                    if (c == FORMAT_END) {
                        tokens.push(Token {
                            is_text: false,
                            text_content: "",
                            optional: false,
                            type: build.copy(),
                            format: Vec::new(),
                        });
                    } else {
                        build.push(c);
                    }
                },
            }

            current_index++;
        }

        return tokens;
    }

    fn create_token_from_type(input: String, optional: bool) -> Token {
        let type = input.trim();
        if (type.is_empty()) {
            // TODO: Throw error
        }

        let mut token = Token {
            is_text: false,
            text_content: "",
            optional: optional,
            type: type.chars()[0],
            length: type.len(),
            format: Vec::new(),
        };

        let mut maximal_length: u8 = 0;

        match token.type {
            'd' => {
                
            }
        }
    }
}