use std::collections::HashMap;

type Var = String;

#[derive(Clone,Debug)]
enum Exp {
    Constant(Var),
    Var(Var),
    Lam(Var,Box<Exp>),
    App(Box<Exp>,Box<Exp>)
}

type Environment = HashMap<Var,Value>;

type Closure = (Exp,Environment);

#[derive(Clone,Debug)]
enum Value {
    Constant(Var),
    Closure(Closure),
}

#[derive(Debug)]
enum Continuation {
    Done,
    EvalArg(Exp,Environment,Box<Continuation>),
    Call(Exp,Environment,Box<Continuation>),
    Failed
}

type Control = Exp;

type State = (Control,Environment,Continuation);

fn step(state: State) -> State {
    match state {
        // case: Control == Variable -> Lookup variable in Environment
        (Exp::Var(x),e1,k) => match e1.get(&x).unwrap() {
            // if its a closure, use its environment and evaluate contained lambda expression
            Value::Closure((t,e2)) => (t.clone(),e2.clone(),k),
            // if its a constant, put constant in control
            Value::Constant(x2) => (Exp::Constant(x2.clone()),e1,k)
        },
        // case: Control == Application -> put Lambda term into control and put Argument into EvalArg continuation
        (Exp::App(t1,t2),e,k) => (*t1,e.clone(),Continuation::EvalArg(*t2,e,Box::new(k))),
        // case: Control == Lambda && Cont == EvalArg -> put lambda expression into Call continuation and evaluate expression from EvalArg continuation
        (Exp::Lam(x1,t1),e1,Continuation::EvalArg(t2,e2,k)) => (t2,e2,Continuation::Call(Exp::Lam(x1,t1),e1,k)),
        // case: Control == Lambda && Cont == Call ->   set the argument of the Call continuation inside the environment of the Call continuation
        //                                              equal to the Closure of the Lambda expression and its environment
        (Exp::Lam(x2,t2),e2,Continuation::Call(Exp::Lam(x1,t1),mut e1,k)) => {e1.insert(x1,Value::Closure((Exp::Lam(x2,t2),e2)));(*t1,e1,*k)},
        // case: Control == Constant && continuation == Call -> set argument of Call continuation to the constant in the environment of the call continuation
        (Exp::Constant(x2),_,Continuation::Call(Exp::Lam(x1,t1),mut e1,k)) => {e1.insert(x1,Value::Constant(x2));(*t1,e1,*k)},
        // case: Control == Constant && continuation == else -> Done
        (Exp::Constant(x2),e,k) => (Exp::Constant(x2),e,k),
        // else
        (c,e,_) => (c,e,Continuation::Failed)
    }
}

fn main() {

    let env : Environment = HashMap::new();

    // example 1
    let e1 = Box::new(Exp::Lam(String::from("f"),Box::new(Exp::App(Box::new(Exp::Var(String::from("f"))),Box::new(Exp::Var(String::from("f")))))));
    let e2 = Box::new(Exp::Lam(String::from("y"),Box::new(Exp::Var(String::from("y")))));

    //example 2
    let e3 = Box::new(Exp::Lam(String::from("f"),Box::new(Exp::App(Box::new(Exp::Var(String::from("f"))),Box::new(Exp::Constant(String::from("1")))))));
    let e4 = Box::new(Exp::App(Box::new(Exp::Lam(String::from("x"),Box::new(Exp::Lam(String::from("y"),Box::new(Exp::Var(String::from("x"))))))),Box::new(Exp::Constant(String::from("2")))));

    let mut state = (Exp::App(e3,e4),env,Continuation::Done);

    dbg!(&state);
    for _i in 1..12 {
        state = step(state);
        dbg!(&state);
    }
}
