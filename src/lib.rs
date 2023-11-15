use pest::error::ErrorVariant;
use pest::iterators::Pair;
use pest::{Parser, Span};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "pgn.pest"]
pub struct PGNParser;

pub struct ParsedGame {
    pub metadata: Vec<(String, String)>,
    pub moves: Vec<(Move, Option<Move>)>,
    pub game_result: String,
}

impl ToString for ParsedGame {
    fn to_string(&self) -> String {
        let metadata_str: String = self
            .metadata
            .iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<String>>()
            .join("\n");

        let moves_str: String = self
            .moves
            .iter()
            .enumerate()
            .map(|(index, (first_move, second_move))| {
                let first_move_str = first_move.to_string();
                let second_move_str = second_move
                    .as_ref()
                    .map(|mv| mv.to_string())
                    .unwrap_or_default();

                format!(
                    "Move #{}\nWhite: {} Black: {}",
                    (index + 1),
                    first_move_str,
                    second_move_str
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            "Game Metadata\n{}\n\nList of moves\n{}\n\nGame Result\n{}\n",
            metadata_str, moves_str, self.game_result
        )
    }
}

pub struct Move {
    piece_move: String,
    comment: Option<String>,
}

impl ToString for Move {
    fn to_string(&self) -> String {
        let piece_move_str = &self.piece_move;
        let comment_str = self.comment.as_ref().map_or("", |comment| comment);
        format!("{} {}", piece_move_str, comment_str)
    }
}

pub fn extract_pairs(pair: Pair<Rule>, rule: Rule) -> Option<Pair<Rule>> {
    pair.into_inner().find(|pair| pair.as_rule() == rule)
}

pub fn parse_pgn(to_parse: &str) -> Result<ParsedGame, pest::error::Error<Rule>> {
    let mut parsed_game = ParsedGame {
        metadata: Vec::new(),
        moves: Vec::new(),
        game_result: String::new(),
    };
    let parse_result = PGNParser::parse(Rule::game, to_parse);

    return match parse_result {
        Ok(pairs) => {
            let pgn_game = pairs
                .into_iter()
                .find(|pair| pair.as_rule() == Rule::game)
                .ok_or_else(|| {
                    pest::error::Error::<Rule>::new_from_span(
                        ErrorVariant::CustomError {
                            message: String::from("Game wasn't found"),
                        },
                        Span::new(to_parse, 0, to_parse.len()).unwrap(),
                    )
                })?;
            let move_list = extract_pairs(pgn_game.clone(), Rule::move_list).ok_or_else(|| {
                pest::error::Error::<Rule>::new_from_span(
                    ErrorVariant::CustomError {
                        message: String::from("Move list wasn't found"),
                    },
                    Span::new(to_parse, 0, to_parse.len()).unwrap(),
                )
            })?;
            let metadata_block: Vec<(String, String)> = pgn_game
                .clone()
                .into_inner()
                .filter_map(|pair| match pair.as_rule() {
                    Rule::metadata_block => {
                        let mut inner_pairs = pair.into_inner();
                        let key = inner_pairs.next().unwrap().as_str().to_string();
                        let value = inner_pairs.next().unwrap().as_str().to_string();
                        Some((key, value))
                    }
                    _ => None,
                })
                .collect();
            let game_result =
                extract_pairs(pgn_game.clone(), Rule::game_result).ok_or_else(|| {
                    pest::error::Error::<Rule>::new_from_span(
                        ErrorVariant::CustomError {
                            message: String::from("Game result wasn't found"),
                        },
                        Span::new(to_parse, 0, to_parse.len()).unwrap(),
                    )
                })?;

            let moves_list: Vec<(Move, Option<Move>)> = move_list
                .clone()
                .into_inner()
                .filter_map(|pair| match pair.as_rule() {
                    Rule::move_pair => {
                        let mut first_comment: Option<String> = None;
                        let mut second_comment: Option<String> = None;
                        let mut second_piece_move: Option<String> = None;
                        let mut inner_pairs = pair.into_inner();
                        let first_piece_move = inner_pairs.next().unwrap().as_str().to_string();
                        let second_lex = inner_pairs.next();
                        let third_lex = inner_pairs.next();
                        let fourth_lex = inner_pairs.next();
                        if let Some(pair) = second_lex {
                            if pair.as_rule() == Rule::move_comment {
                                first_comment = Some(pair.as_str().to_string());
                            } else {
                                second_piece_move = Some(pair.as_str().to_string());
                            }
                        }
                        if let Some(pair) = third_lex {
                            if pair.as_rule() == Rule::move_comment {
                                second_comment = Some(pair.as_str().to_string());
                            } else {
                                second_piece_move = Some(pair.as_str().to_string());
                            }
                        }

                        if let Some(pair) = fourth_lex {
                            second_comment = Some(pair.as_str().to_string());
                        }

                        let first_move = Move {
                            piece_move: first_piece_move,
                            comment: first_comment,
                        };
                        if let Some(piece_move) = second_piece_move {
                            let second_move = Move {
                                piece_move,
                                comment: second_comment,
                            };
                            Some((first_move, Some(second_move)))
                        } else {
                            Some((first_move, None))
                        }
                    }
                    _ => None,
                })
                .collect();
            parsed_game.game_result = game_result.as_str().to_string();
            parsed_game.moves = moves_list;
            parsed_game.metadata = metadata_block;
            return Ok(parsed_game);
        }
        Err(e) => Err(e),
    };
}
