mod tests {
    use anyhow::anyhow;
    use pest::Parser;
    use pgn_parser::*;

    #[test]
    fn test_word() -> anyhow::Result<()> {
        let input = "word";
        let pair = PGNParser::parse(Rule::word, input)?
            .next()
            .ok_or_else(|| anyhow!("no metadata block"))?;

        assert_eq!(pair.as_str(), "word");
        Ok(())
    }

    #[test]
    fn test_word_panic() {
        let input = "\"word\"";
        let _pair = PGNParser::parse(Rule::word, input);
        assert!(_pair.is_err())
    }

    #[test]
    fn test_metadata_block() -> anyhow::Result<()> {
        let input = "[key \"Value\"]";
        let pair = PGNParser::parse(Rule::metadata_block, input)?
            .next()
            .ok_or_else(|| anyhow!("no metadata block"))?;

        assert_eq!(pair.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_move_comment() -> anyhow::Result<()> {
        let input = "{ Comment }";

        let pair = PGNParser::parse(Rule::move_comment, input)?
            .next()
            .ok_or_else(|| anyhow!("no comment was found"))?;

        assert_eq!(pair.as_str(), "{ Comment }");
        Ok(())
    }

    #[test]
    fn test_move_piece_without_capture() -> anyhow::Result<()> {
        let move_king = "Kf8";
        let move_knight = "Ndf8";
        let move_bishop = "Be5";
        let move_rook = "Rae1";
        let move_queen = "Qa2";
        let move_pawn = "e4";

        let pair_king = PGNParser::parse(Rule::move_piece_without_capture, move_king);
        assert!(pair_king.is_ok());

        let pair_knight = PGNParser::parse(Rule::move_piece_without_capture, move_knight);
        assert!(pair_knight.is_ok());

        let pair_bishop = PGNParser::parse(Rule::move_piece_without_capture, move_bishop);
        assert!(pair_bishop.is_ok());

        let pair_queen = PGNParser::parse(Rule::move_piece_without_capture, move_queen);
        assert!(pair_queen.is_ok());

        let pair_rook = PGNParser::parse(Rule::move_piece_without_capture, move_rook);
        assert!(pair_rook.is_ok());

        let pair_pawn = PGNParser::parse(Rule::move_piece_without_capture, move_pawn);
        assert!(pair_pawn.is_ok());
        Ok(())
    }

    #[test]
    fn test_complete_move() {
        let move_with_capture = "exf5";
        let castle = "O-O";
        let move_with_checkmate = "Qe8#";
        let move_with_check = "Qe8+";

        let pair_castle = PGNParser::parse(Rule::complete_move, castle);
        assert!(pair_castle.is_ok());

        let pair_check = PGNParser::parse(Rule::complete_move, move_with_check);
        assert!(pair_check.is_ok());

        let pair_checkmate = PGNParser::parse(Rule::complete_move, move_with_checkmate);
        assert!(pair_checkmate.is_ok());

        let pair_capture = PGNParser::parse(Rule::complete_move, move_with_capture);
        assert!(pair_capture.is_ok());
    }

    #[test]
    fn test_move_pair() {
        let move_pair = "1. e4 e5";
        let incorrect_move_pair = "e4 e5";

        let pair_move = PGNParser::parse(Rule::move_pair, move_pair);
        assert!(pair_move.is_ok());

        let incorrect_pair_move = PGNParser::parse(Rule::move_pair, incorrect_move_pair);
        assert!(incorrect_pair_move.is_err());
    }

    #[test]
    fn test_game_result() {
        let draw = "1/2-1/2";
        let white_win = "1-0";
        let black_win = "0-1";

        let incorrect_result = "0-0";

        let pair_draw = PGNParser::parse(Rule::game_result, draw);
        assert!(pair_draw.is_ok());

        let pair_white_win = PGNParser::parse(Rule::game_result, white_win);
        assert!(pair_white_win.is_ok());

        let pair_black_win = PGNParser::parse(Rule::game_result, black_win);
        assert!(pair_black_win.is_ok());

        let pair_incorrect_result = PGNParser::parse(Rule::game_result, incorrect_result);
        assert!(pair_incorrect_result.is_err());
    }

    #[test]
    fn test_game1() -> Result<(), pest::error::Error<Rule>> {
        let input = std::fs::read_to_string("games/game1.txt").expect("Unable to read file");

        let game = pgn_parser::parse_pgn(&input)?;

        assert_eq!(game.game_result, "1-0");
        assert_eq!(game.moves.len(), 34);
        assert_eq!(game.metadata.len(), 18);

        Ok(())
    }

    #[test]
    fn test_game2() -> Result<(), pest::error::Error<Rule>> {
        let input = std::fs::read_to_string("games/game2.txt").expect("Unable to read file");

        let game = pgn_parser::parse_pgn(&input)?;

        assert_eq!(game.game_result, "0-1");
        assert_eq!(game.moves.len(), 17);
        assert_eq!(game.metadata.len(), 18);

        Ok(())
    }

    #[test]
    fn test_game3() -> Result<(), pest::error::Error<Rule>> {
        let input = std::fs::read_to_string("games/game3.txt").expect("Unable to read file");

        let game = pgn_parser::parse_pgn(&input)?;

        assert_eq!(game.game_result, "1/2-1/2");
        assert_eq!(game.moves.len(), 43);
        assert_eq!(game.metadata.len(), 7);

        Ok(())
    }

    #[test]
    fn test_game4() -> Result<(), pest::error::Error<Rule>> {
        let input = std::fs::read_to_string("games/game4.txt").expect("Unable to read file");

        let game = pgn_parser::parse_pgn(&input)?;

        assert_eq!(game.game_result, "1-0");
        assert_eq!(game.moves.len(), 23);
        assert_eq!(game.metadata.len(), 7);

        Ok(())
    }

    #[test]
    fn test_game5() -> Result<(), pest::error::Error<Rule>> {
        let input = std::fs::read_to_string("games/game5.txt").expect("Unable to read file");

        let game = pgn_parser::parse_pgn(&input)?;

        assert_eq!(game.game_result, "1-0");
        assert_eq!(game.moves.len(), 44);
        assert_eq!(game.metadata.len(), 8);

        Ok(())
    }

    #[test]
    fn test_game6() -> Result<(), pest::error::Error<Rule>> {
        let input = std::fs::read_to_string("games/game6.txt").expect("Unable to read file");

        let game = pgn_parser::parse_pgn(&input)?;

        assert_eq!(game.game_result, "1-0");
        assert_eq!(game.moves.len(), 17);
        assert_eq!(game.metadata.len(), 8);

        Ok(())
    }
}
