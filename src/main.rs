use timpani::ast::{
    BinaryOperator,
    Expression,
    Function,
    Ident,
    Program,
    Statement
};

use timpani::parser::parseFunction;

fn main() {
    println!("Hello, world!");

    // function begin() {
    //   return 5;
    // }
    let begin1 = Function {
        name: Ident::from("begin"),
        parameters: vec![],
        declared_vars: vec![],
        body: Statement::Empty,
        return_expr: Expression::Int(5),
    };
    let prog1 = Program {
        functions: vec![begin1]
    };
    println!("program =  {:?}", prog1);

    // function add5(x) {
    //   return x + 5;
    // }
    // function begin() {
    //   return add5(7);
    // }
    let add5 = Function {
        name: Ident::from("add5"),
        parameters: vec![Ident::from("x")],
        declared_vars: vec![],
        body: Statement::Empty,
        return_expr: Expression::BinOp(BinaryOperator::Add,
            Box::new(Expression::Ident(Ident::from("x"))),
            Box::new(Expression::Int(5))),
    };
    let begin2 = Function {
        name: Ident::from("begin"),
        parameters: vec![],
        declared_vars: vec![],
        body: Statement::Empty,
        return_expr: Expression::FuncCall(Ident::from("add5"), vec![Box::new(Expression::Int(7))]),
    };
    let prog2 = Program {
        functions: vec![add5, begin2]
    };
    println!("program =  {:?}", prog2);

    println!("====================");

    let textProg = "function begin() { return -5; }";
    println!("parsing '{}'", textProg);
    let parseResult = parseFunction(textProg);
    println!(" result = {:?}", parseResult);
}
