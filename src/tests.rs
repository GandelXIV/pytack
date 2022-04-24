#[cfg(test)]

use super::*;


#[test]
fn test_gettype_str() {
    assert_eq!( gettype("\"hello\""), Ok(PythonType::Str) );
    assert_eq!( gettype("'hello'"), Ok(PythonType::Str) );
    assert_eq!( gettype("''"), Ok(PythonType::Str) );
    assert_ne!( gettype("12"), Ok(PythonType::Str) ) ;
    assert_ne!( gettype("varx"), Ok(PythonType::Str));
}

#[test]
fn test_gettype_bool() {
    assert_eq!( gettype("True"), Ok(PythonType::Bool) );
    assert_eq!( gettype("False"), Ok(PythonType::Bool) );
    assert_ne!( gettype("true"), Ok(PythonType::Bool));
}

#[test]
fn test_gettype_int() {
    assert_eq!( gettype("69"), Ok(PythonType::Int) );
    assert_eq!( gettype("12_000"), Ok(PythonType::Int) );
    assert_ne!( gettype("3.14"), Ok(PythonType::Int));
    assert_ne!( gettype("'3'"), Ok(PythonType::Int));
}

#[test]
fn test_gettype_explicit() {
    assert_eq!( gettype_explicit("int"), Ok(PythonType::Int) );
    assert_ne!( gettype_explicit("foobar"), Ok(PythonType::Int) );
}

#[test]
fn test_gettype_float() {
    assert_eq!( gettype("3.14"), Ok(PythonType::Float) );
    assert_ne!( gettype("13"), Ok(PythonType::Float) );
    assert_ne!( gettype("'3.14'"), Ok(PythonType::Float) );
}

#[test]
fn test_tokenize() {
    assert_eq!( tokenize("\tx = 4"), vec!["x", "=", "4"] );
    assert_eq!( tokenize("x: int = 4"), vec!["x", ":", "int", "=", "4"] );
    assert_eq!( tokenize("txt = 'Hello World!'"), vec!["txt", "=", "'Hello World!'"] );
    assert_eq!( tokenize("def add(x: int, y: int) -> int"), vec!["def", "add", "(", "x", ":", "int", ",", "y", ":", "int", ")", "->", "int"] );
    assert_eq!( tokenize("x=1 #hello"), vec!["x", "=", "1"] );
}

#[test]
fn test_interpret() {
    assert_eq!( interpret(vec!["x".to_string(), "=".to_string(), "2".to_string()]), Some(Setting::Assignment(String::from("x"), PythonType::Int)));
    assert_eq!( interpret( tokenize("x: int = 3")), Some( Setting::VariableDefinition("x".to_string(), PythonType::Int, PythonType::Int) ) );
    assert_eq!( interpret( tokenize("def pow(x: int, y: int=2, foo, bar=\"nothing\") -> int:") ),  Some( Setting::FunctionDefinition(
            "pow".to_string(),
            vec![
                Setting::VariableDeclaration(
                    "x".to_string(),
                    PythonType::Int,
                ),
                Setting::VariableDefinition(
                "y".to_string(),
                PythonType::Int,
                PythonType::Int,
                ),
                Setting::Assignment(
                "bar".to_string(),
                PythonType::Str,
                ),
            ],
            Some(PythonType::Int),
    )));
}
