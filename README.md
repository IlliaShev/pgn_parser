# PGN Parser for Chess Games

## Overview

This is a simple PGN (Portable Game Notation) parser for chess games written in Rust. It allows you to parse PGN files and extract information about chess games.

## Crate

https://crates.io/crates/pgn_parser

## Usage

### Command-Line Interface (CLI)
The PGN parser comes with a CLI offering the following subcommands:

### Help
Display help information for the PGN parser CLI:

```
pgn_parser help
```

### Parse
Parse a PGN file and output game information. This subcommand requires two arguments:

-i or --input: Specify the input file.
-o or --output: Specify the output file for parsed information.

```
pgn_parser parse -i input.txt -o output.txt
```

If you don't specify an output file using the -o or --output option, the game information will be written directly to your command-line interface (CLI).

## Grammar

Represents a metadata block enclosed in square brackets, consisting of two adjacent words enclosed in double quotes.
```
metadata_block = { "[" ~ word ~ "\"" ~ word ~ "\"]" }
```

Represents a sequence of characters that is not "]" or """ (double quote), allowing any character except those. It captures one or more characters.
```
word = { (!("]" | "\"") ~ ANY)+ }
```

Represents a character from 'a' to 'h', inclusive, representing chess files.
```
files = { 'a'..'h' }
```

Represents a character from '1' to '8', inclusive, representing chess ranks.
```
ranks = { '1' .. '8' }
```

Represents a comment for a move enclosed in curly braces. It captures any characters except '}' (closing curly brace).
```
move_comment = { "{" ~ (!("}") ~ ANY)* ~ "}" }
```
Represents a chess piece abbreviation: "K" (king), "Q" (queen), "R" (rook), "B" (bishop), or "N" (knight). It is optional.
```
piece = { ("K" | "Q" | "R" | "B" | "N")? }
```

Represents a castling move: "O-O-O" (queenside castling) or "O-O" (kingside castling).
```
castle = { ("O-O-O" | "O-O") }
```

Represents a move in chess
```
move_piece_with_capture = { piece ~ (files)? ~ "x" ~ files ~ ranks }
move_piece_without_capture = { piece ~ (files){1,2} ~ ranks }
move_piece = { (move_piece_with_capture | move_piece_without_capture)}
complete_move = { (move_piece | castle) ~ ("+" | "#")? }
```

Represents a pair of moves in chess notation, including the move number and optional move comments.
```
move_pair = { (ASCII_DIGIT)+ ~ "." ~ complete_move ~ move_comment? ~ complete_move? ~ move_comment? }
```

Represents a list of move pairs.
```
move_list = { (move_pair)+ }
```

Represents the result of a chess game, either "1/2-1/2" (draw), "1-0" (white wins), or "0-1" (black wins).
```
game_result = { "1/2-1/2" | "1-0" | "0-1" 
```

Represents a complete chess game, starting with the start of input (SOI), followed by optional metadata blocks, a move list, a game result, and the end of input (EOI).
```
game = { SOI ~ (metadata_block)* ~ move_list ~ game_result ~ EOI}
```

## Example

### Input example
```
[Event "Rated Blitz game"]
[Site "https://lichess.org/GI7wh3rQ"]
[Date "2023.11.13"]
[White "Ilya_Sh"]
[Black "Jlquilo"]
[Result "1-0"]
[UTCDate "2023.11.13"]
[UTCTime "14:26:46"]
[WhiteElo "1886"]
[BlackElo "1886"]
[WhiteRatingDiff "+6"]
[BlackRatingDiff "-6"]
[Variant "Standard"]
[TimeControl "180+0"]
[ECO "B01"]
[Opening "Scandinavian Defense: Valencian Variation"]
[Termination "Normal"]
[Annotator "lichess.org"]

1. e4 d5 2. exd5 Qxd5 3. Nc3 Qd8 { B01 Scandinavian Defense: Valencian Variation } 4. d4 e6 5. Nf3 c6 6. Bd3 Be7 7. O-O b5 8. a3 a5 9. Ne4 Ba6 10. Nc5 Bc8 11. Re1 Nf6 12. Bg5 O-O 13. c3 Na6 14. Nxa6 Bxa6 15. Qc2 h6 16. Bxf6 Bxf6 17. Ne5 Qc7 18. Re3 Rfc8 19. Rh3 Bxe5 20. dxe5 Qxe5 21. g4 c5 22. Be4 Rab8 23. Rg3 Qg5 24. Kg2 b4 25. h4 Qxh4 26. g5 h5 27. Rh1 Qf4 28. Rxh5 Bb7 29. Bxb7 Rxb7 30. Qh7+ Kf8 31. Qh8+ Ke7 32. Qxc8 Qe4+ 33. Kh2 Rd7 34. Qxc5+ { Black resigns. } 1-0
```

### Output
```
Game Metadata
Event: Rated Blitz game
Site: https://lichess.org/GI7wh3rQ
Date: 2023.11.13
White: Ilya_Sh
Black: Jlquilo
Result: 1-0
UTCDate: 2023.11.13
UTCTime: 14:26:46
WhiteElo: 1886
BlackElo: 1886
WhiteRatingDiff: +6
BlackRatingDiff: -6
Variant: Standard
TimeControl: 180+0
ECO: B01
Opening: Scandinavian Defense: Valencian Variation
Termination: Normal
Annotator: lichess.org

List of moves
Move #1
White: e4   Black: d5  
Move #2
White: exd5   Black: Qxd5  
Move #3
White: Nc3   Black: Qd8  { B01 Scandinavian Defense: Valencian Variation }
Move #4
White: d4   Black: e6  
Move #5
White: Nf3   Black: c6  
Move #6
White: Bd3   Black: Be7  
Move #7
White: O-O   Black: b5  
Move #8
White: a3   Black: a5  
Move #9
White: Ne4   Black: Ba6  
Move #10
White: Nc5   Black: Bc8  
Move #11
White: Re1   Black: Nf6  
Move #12
White: Bg5   Black: O-O  
Move #13
White: c3   Black: Na6  
Move #14
White: Nxa6   Black: Bxa6  
Move #15
White: Qc2   Black: h6  
Move #16
White: Bxf6   Black: Bxf6  
Move #17
White: Ne5   Black: Qc7  
Move #18
White: Re3   Black: Rfc8  
Move #19
White: Rh3   Black: Bxe5  
Move #20
White: dxe5   Black: Qxe5  
Move #21
White: g4   Black: c5  
Move #22
White: Be4   Black: Rab8  
Move #23
White: Rg3   Black: Qg5  
Move #24
White: Kg2   Black: b4  
Move #25
White: h4   Black: Qxh4  
Move #26
White: g5   Black: h5  
Move #27
White: Rh1   Black: Qf4  
Move #28
White: Rxh5   Black: Bb7  
Move #29
White: Bxb7   Black: Rxb7  
Move #30
White: Qh7+  Black: Kf8  
Move #31
White: Qh8+  Black: Ke7  
Move #32
White: Qxc8   Black: Qe4+ 
Move #33
White: Kh2   Black: Rd7  
Move #34
White: Qxc5+ { Black resigns. } Black: 

Game Result
1-0
```