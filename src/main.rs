use std::collections::HashMap;

type Var = String;

#[derive(Clone,Debug)]
enum Exp {
    Var(Var),
    Lam(Var,Box<Exp>),
    App(Box<Exp>,Box<Exp>),
    Int(i32),
    Bool(bool),
    Let(Box<Exp>,Box<Exp>,Box<Exp>),
    If(Box<Exp>,Box<Exp>,Box<Exp>),
    Fix(Var,Box<Exp>,Box<Exp>),
}

type Environment = HashMap<Var,Value>;

type Closure = (Exp,Environment);

#[derive(Clone,Debug)]
enum Value {
    Closure(Closure),
    Int(i32),
    Bool(bool),
    Fix(Var,Box<Exp>,Box<Exp>),
}

#[derive(Debug)]
enum Continuation {
    Done,
    EvalArg(Exp,Environment,Box<Continuation>),
    Call(Exp,Environment,Box<Continuation>),
    Decide(Exp,Exp,Environment,Box<Continuation>),
    Failed(String),
}

type Control = Exp;

type State = (Control,Environment,Continuation);

fn step(state: State) -> State {
    match state {
        // case: Control == Variable -> Lookup variable in Environment
        (Exp::Var(x),mut e1,k) => match e1.get_mut(&x).unwrap() {
            // if its a closure, use its environment and evaluate contained lambda expression
            Value::Closure((t,e2)) => (t.clone(),e2.clone(),k),
            // if its a integer, put integer in control
            Value::Int(x2) => (Exp::Int(x2.clone()),e1,k),
            // if its a boolean, put boolean in control
            Value::Bool(x2) => (Exp::Bool(x2.clone()),e1,k),
            // if its a boolean, put boolean in control
            Value::Fix(_,n,l) => match **n {
                Exp::Int(mut i) => {i=i-1;if i>=0 {
                    (*l.clone(),e1,k)
                } else {
                    (Exp::Var(x),e1,Continuation::Failed(String::from("Too many recursions")))
                }},
                _ => (Exp::Var(x),e1,Continuation::Failed(String::from("No int in fix"))),
            },
        },
        // case: Control == Application -> put Lambda term into control and put Argument into EvalArg continuation
        (Exp::App(t1,t2),e,k) => (*t1,e.clone(),Continuation::EvalArg(*t2,e,Box::new(k))),
        // case: Control == Lambda && Cont == EvalArg -> put lambda expression into Call continuation and evaluate expression from EvalArg continuation
        (Exp::Lam(x1,t1),e1,Continuation::EvalArg(t2,e2,k)) => (t2,e2,Continuation::Call(Exp::Lam(x1,t1),e1,k)),
        // case: Control == Lambda && Cont == Call ->   set the argument of the Call continuation inside the environment of the Call continuation
        //                                              equal to the Closure of the Lambda expression and its environment
        (Exp::Lam(x2,t2),e2,Continuation::Call(Exp::Lam(x1,t1),mut e1,k)) => {e1.insert(x1,Value::Closure((Exp::Lam(x2,t2),e2)));(*t1,e1,*k)},
        // case: Control == Lambda && else -> no proper continuation in place
        (Exp::Lam(x,t),e,_) => (Exp::Lam(x,t),e,Continuation::Failed(String::from("Continuation is wrong for lambda"))),
        // case: Control == Int && continuation == Call -> set argument of Call continuation to the integer in the environment of the call continuation
        (Exp::Int(x2),_,Continuation::Call(Exp::Lam(x1,t1),mut e1,k)) => {e1.insert(x1,Value::Int(x2));(*t1,e1,*k)},
        // case: Control == Int && continuation == Call -> set argument of Call continuation to the integer in the environment of the call continuation
        (Exp::Bool(x2),_,Continuation::Call(Exp::Lam(x1,t1),mut e1,k)) => {e1.insert(x1,Value::Bool(x2));(*t1,e1,*k)},
        // case: Control == Let -> let a = t1 in t2 -> t1 is argument in call to lambda(a,t2)
        (Exp::Let(v,t1,t2),e,k) => match *v {
            Exp::Var(x) => (*t1,e.clone(),Continuation::Call(Exp::Lam(x,t2),e,Box::new(k))),
            _ => (Exp::Let(v,t1,t2),e,Continuation::Failed(String::from("No variable in let binding")))
        },
        // case: Control == If -> put if guard in Control and the branches into a Decide continuation
        (Exp::If(t1,t2,t3),e,k) => (*t1,e.clone(),Continuation::Decide(*t2,*t3,e,Box::new(k))),
        (c,e1,Continuation::Decide(t2,t3,e2,k)) => match c {
            Exp::Bool(true) => (t2,e2,*k),
            Exp::Bool(false) => (t3,e2,*k),
            _ => (c,e1,Continuation::Failed(String::from("No boolean in if clause")))
        },
        // case: Control == Fix -> evaluate term and put same fix in environment
        (Exp::Fix(f,n,l),mut e,k) => match *l.clone() {
            Exp::Lam(_x,t) => {e.insert(f.clone(),Value::Fix(f,n,l));(*t,e,k)},
            _ => (Exp::Fix(f,n,l),e,Continuation::Failed(String::from("No lambda in fix")))
        },
        // else: Done
        (Exp::Int(x),e,_) => (Exp::Int(x),e,Continuation::Done),
        (Exp::Bool(x),e,_) => (Exp::Bool(x),e,Continuation::Done)
    }
}

fn main() {

    let env : Environment = HashMap::new();

    // example 1
    let e1 = Box::new(Exp::Lam(String::from("f"),Box::new(Exp::App(Box::new(Exp::Var(String::from("f"))),Box::new(Exp::Var(String::from("f")))))));
    let e2 = Box::new(Exp::Lam(String::from("y"),Box::new(Exp::Var(String::from("y")))));

    //example 2
    let e3 = Box::new(Exp::Lam(String::from("f"),Box::new(Exp::App(Box::new(Exp::Var(String::from("f"))),Box::new(Exp::Int(1))))));
    let e4 = Box::new(Exp::App(Box::new(Exp::Lam(String::from("x"),Box::new(Exp::Lam(String::from("y"),Box::new(Exp::Var(String::from("x"))))))),Box::new(Exp::Int(2))));

    let mut state = (Exp::App(e3,e4),env,Continuation::Done);

    dbg!(&state);
    for _i in 1..12 {
        state = step(state);
        dbg!(&state);
    }
}
