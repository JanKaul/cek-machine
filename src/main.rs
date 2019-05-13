use std::collections::HashMap;

type Var = String;

#[derive(Clone,Debug)]
enum Exp {
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
            // if its a constant, done
            Value::Constant(_) => (Exp::Var(x),e1,Continuation::Done)
        },
        // case: Control == Application -> put Lambda term into control and put Argument into EvalArg continuation
        (Exp::App(t1,t2),e,k) => (*t1,e.clone(),Continuation::EvalArg(*t2,e,Box::new(k))),
        // case: Control == Lambda && Cont == EvalArg -> put lambda expression into Call continuation and evaluate expression from EvalArg continuation
        (Exp::Lam(x1,t1),e1,Continuation::EvalArg(t2,e2,k)) => (t2,e2,Continuation::Call(Exp::Lam(x1,t1),e1,k)),
        // case: Control == Lambda && Cont == Call ->   set the argument of the Call continuation equal to the Closure of the Lambda expression and
        //                                              its environment inside the environment of the Call continuation
        (Exp::Lam(x2,t2),e2,Continuation::Call(Exp::Lam(x1,t1),mut e1,k)) => {e1.insert(x1,Value::Closure((Exp::Lam(x2,t2),e2)));(*t1,e1,*k)},
        // else
        (c,e,_) => (c,e,Continuation::Failed)
    }
}

fn main() {

    let env : Environment = HashMap::new();

    let e1 = Box::new(Exp::Lam(String::from("f"),Box::new(Exp::App(Box::new(Exp::Var(String::from("f"))),Box::new(Exp::Var(String::from("f")))))));
    let e2 = Box::new(Exp::Lam(String::from("y"),Box::new(Exp::Var(String::from("y")))));

    let mut state = (Exp::App(e1,e2),env,Continuation::Done);

    dbg!(&state);
    for _i in 1..12 {
        state = step(state);
        dbg!(&state);
    }
}
