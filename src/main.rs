#[macro_use]
extern crate lazy_static;
use std::io::Repeat;
use regex::Regex;
use std::fs;

pub mod tests;

#[allow(unused_variables)]
#[allow(dead_code)]



enum ReportType {
    // will expand
    WrongAssignment,
    DeclarationAssignmentDisparity,
}

struct Report {
    t: ReportType,
    line: String,
    i: usize,
    symbol: String,
}

#[derive(Debug, PartialEq)]
enum PythonType {
    // imcomplete
    Int,
    Str,
    Bool,
    Float,
    Custom,
}

fn gettype(value: &str) -> Result<PythonType, ()> {
    lazy_static! {
        static ref REGEX_STR: Regex = Regex::new("('|\")").unwrap();    // rString doesnt work
        static ref REGEX_BOOL: Regex = Regex::new(r"(True|False)").unwrap();
        static ref REGEX_INT: Regex = Regex::new(r"^(\d|_)*$").unwrap();
        static ref REGEX_FLOAT: Regex = Regex::new(r"(\d|_)*\.(\d|_)").unwrap();
    }
    
    if REGEX_STR.is_match(value) {
        return Ok(PythonType::Str);
    }
    else if REGEX_BOOL.is_match(value) {
        return Ok(PythonType::Bool);
    }
    else if REGEX_INT.is_match(value) {
        return Ok(PythonType::Int);
    }
    else if REGEX_FLOAT.is_match(value) {
        return Ok(PythonType::Float);
    }
    Err(())
}

fn gettype_explicit(symbol: &str) -> Result<PythonType, ()> {
    match symbol {
        "int" => Ok(PythonType::Int),
        "str" => Ok(PythonType::Str),
        "bool" => Ok(PythonType::Bool),
        "float" => Ok(PythonType::Float),
        _ => Err(()),
    }
}

#[derive(Debug, PartialEq)]
enum Setting {
    Assignment(String, PythonType), // x = 2
    VariableDeclaration(String, PythonType), // x: int
    VariableDefinition(String, PythonType, PythonType), // x: int = 2
    FunctionDefinition(String, Vec<Setting>, Option<PythonType>), // def pow(x: int, y: int=2): -> int
}

fn tokenize(line: &str) -> Vec<String> {
    let mut out = vec![];
    let mut current_word = String::new();
    let mut instr = false;
    let mut esc_char = false;
    for char in line.chars() {
        match char {
            '\t' => {},
            ' ' => {
                if !instr {
                    out.push(current_word);
                    current_word = String::new();
                }
                else {
                    current_word.push(' ');
                }
            },
            ':'|'('|')'|','|'='  => {
                out.push(current_word);
                out.push(String::from(char));
                current_word = String::new();
            },
            '\"' | '\'' => {
                if !esc_char {
                    if instr {
                        instr = false;
                    }
                    else {
                        instr = true;
                    }
                }
                current_word.push('\'');
            },
            '\\' => {
                esc_char = true;
                current_word.push('\\');
            }
            '#' => break,
            _ => current_word.push(char),
        }
    }
    out.push(current_word);
    out.into_iter().filter( |x| !x.is_empty() ).collect()
}

fn interpret(tokens: Vec<String>) -> Option<Setting> {
    // function declaration
    if tokens.len() > 0 && tokens[0] == "def" {
        let mut args: Vec<Setting> = vec![];
        let mut ret_type = None;
        let mut i = tokens.iter().position(|r| r == "(").unwrap() + 1;
        while tokens[i] != ")" {
            if tokens[i] == ":" && tokens[i + 2] == "=" {
                args.push(Setting::VariableDefinition(tokens[i-1].to_owned(), gettype_explicit(&tokens[i+1]).unwrap(), gettype(&tokens[i+3]).unwrap() ));
            }
            else if tokens[i] == ":" {
                args.push( Setting::VariableDeclaration( tokens[i-1].to_owned(), gettype_explicit(&tokens[i+1]).unwrap() ) );
            }
            else if tokens[i] == "=" && tokens[i-2] != ":" {
                args.push(Setting::Assignment(tokens[i-1].to_owned(), gettype(&tokens[i+1]).unwrap() ));
            }
            i += 1;
        }
        if tokens[tokens.len()-3] == "->" {
            ret_type = Some(gettype_explicit(&tokens[tokens.len()-2]).unwrap());
        }
        return Some(Setting::FunctionDefinition(tokens[1].to_owned(), args, ret_type));
    }
    // variable definition
    else if tokens.len() > 4 && tokens[1] == ":" && tokens[3] == "=" {
        return Some(Setting::VariableDefinition(tokens[0].to_owned(), gettype_explicit(&tokens[2]).unwrap(), gettype(&tokens[4]).unwrap() ));
    }
    // variable assignment
    else if tokens.len() > 2 && tokens[1] == "=" {
        return Some(Setting::Assignment(tokens[0].to_owned(), gettype(&tokens[2]).unwrap() ));
    }
    None
}

fn check(code: &str) -> Vec<Report> {
    let mut reports = vec![];
    for (i, line) in code.split(&['\n', ';'][..]).enumerate() {
        let tokens = tokenize(line);
        if let Some(setting) = interpret(tokens) {
            match setting {
                Setting::VariableDefinition(name, tanot, tval) => {
                    if tanot != tval {
                        reports.push(Report { 
                            t: ReportType::DeclarationAssignmentDisparity, 
                            i: i+1,
                            line: String::from(line),
                            symbol: name, 
                        });
                    }
                }
                _ => {} // todo ,
            }
        }
    }
    reports
}

// TODO: add non primitive assignment support, like this: x: int = add(1, 2) or x: int = y
fn main() {
    let contents = fs::read_to_string("test.py").unwrap();
    for report in check(&contents) {
        match report.t {
            ReportType::DeclarationAssignmentDisparity => {
                println!("[line {}] {}", report.i, report.line);
                println!("--> {} has different type than assigned value", report.symbol);
            }
            _ => {}
        }
    }
    println!("Done!");
}

