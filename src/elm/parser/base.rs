use combine::error::ParseError;
use combine::parser::char::{char, space, spaces, string};
use combine::{between, sep_by1, Parser, Stream};

use elm::syntax::modulename::ModuleName;

use super::tokens::type_name;

pub fn module_name<I>() -> impl Parser<Input = I, Output = ModuleName>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    sep_by1(type_name(), char('.'))
}

pub fn spaces1<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    space().with(spaces())
}

#[cfg(test)]
mod tests {

    use super::*;
    use combine::Parser;

    #[test]
    fn module_name_1() {
        assert_eq!(
            module_name().parse("Aaa.Bb.Cd"),
            Ok((
                vec!["Aaa".to_string(), "Bb".to_string(), "Cd".to_string()],
                ""
            ))
        );
    }
}

/*
module Elm.Parser.Base exposing (moduleName, typeIndicator)

import Combine exposing (Parser, sepBy1, string)
import Elm.Parser.Ranges exposing (withRange)
import Elm.Parser.State exposing (State)
import Elm.Parser.Tokens as Tokens
import Elm.Syntax.ModuleName exposing (ModuleName)


moduleName : Parser s ModuleName
moduleName =
    sepBy1 (string ".") Tokens.typeName


typeIndicator : Parser s ( ModuleName, String )
typeIndicator =
    let
        helper ( n, xs ) =
            Combine.choice
                [ string "."
                    |> Combine.continueWith Tokens.typeName
                    |> Combine.andThen (\t -> helper ( t, n :: xs ))
                , Combine.succeed ( n, xs )
                ]
    in
    Tokens.typeName
        |> Combine.andThen (\t -> helper ( t, [] ))
        |> Combine.map (\( t, xs ) -> ( List.reverse xs, t ))
*/