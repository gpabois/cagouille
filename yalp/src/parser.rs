use crate::{symbol::Symbol, lexer};

pub mod traits {
    use crate::symbol::Symbol;
    pub trait Parser {
        type Error;
        type Symbol: Symbol;
        type Terminal: Symbol + Into<Self::Symbol>;
    }
}

#[derive(Clone)]
pub enum ParserOp {
    Shift(usize),
    Reduce(usize),
    Accept
}

#[derive(Clone)]
struct ParserAction<Parser: traits::Parser> {
    r#type: <Parser::Terminal as Symbol>::Type,
    pub op: ParserOp
}

#[derive(Clone)]
struct ParserGoto<Parser: traits::Parser> {
    r#type: <Parser::Symbol as Symbol>::Type,
    next_state: usize
}

#[derive(Clone)]
pub struct ParserState<Parser: traits::Parser> {
    actions: Vec<ParserAction<Parser>>,
    goto: Vec<ParserGoto<Parser>>
}

impl<Ctx: traits::Parser> ParserState<Ctx> {
    pub fn new() -> Self {
        Self{actions: Default::default(), goto: Default::default()}
    }

    pub fn reduce(&mut self, terminal: <Ctx::Terminal as Symbol>::Type, rule: usize) -> &mut Self {
        self.actions.push(ParserAction {
            r#type: terminal,
            op: ParserOp::Reduce(rule)
        });

        self
    } 

    pub fn shift(&mut self, terminal: <Ctx::Terminal as Symbol>::Type, next_state: usize) -> &mut Self {
        self.actions.push(ParserAction {
            r#type: terminal,
            op: ParserOp::Shift(next_state)
        });

        self
    } 

    pub fn goto(&mut self, symbol: <Ctx::Symbol as Symbol>::Type, next_state: usize) -> &mut Self {
        self.goto.push(ParserGoto {
            r#type: symbol,
            next_state
        });

        self
    } 

    pub(self) fn get_goto(&self, symbol: &<Ctx::Symbol as Symbol>::Type) -> Option<&ParserGoto<Ctx>> {
        self.goto.iter().find(|a| a.r#type == *symbol)  
    }

    pub(self) fn get_action(&self, terminal: &<Ctx::Terminal as Symbol>::Type) -> Option<&ParserAction<Ctx>> {
        self.actions.iter().find(|a| a.r#type == *terminal)
    }
}


#[derive(Clone)]
pub struct ParserTable<Ctx: traits::Parser>(Vec<ParserState<Ctx>>);

impl<Ctx: traits::Parser> ParserTable<Ctx> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn add<F: FnOnce() -> ParserState<Ctx>>(&mut self, f: F) -> &mut Self {
        self.0.push(f());
        self
    }
    
    pub fn get(&self, state: usize) -> Option<&ParserState<Ctx>> {
        self.0.get(state)
    }
}

pub type AnyRuleRunnerFunc<'a, Ctx> = &'a (dyn Fn(&mut ParserStack<Ctx>) -> Result<<Ctx as traits::Parser>::Symbol, <Ctx as traits::Parser>::Error> + Sync + Send);

#[derive(Clone)]
pub struct ParserRuleRunner<'a, Ctx>(AnyRuleRunnerFunc<'a, Ctx>)
where Ctx: traits::Parser;

impl<'a, Ctx> ParserRuleRunner<'a, Ctx> 
where Ctx: traits::Parser {
    pub fn execute(&self, stack: &mut ParserStack<Ctx>) -> Result<Ctx::Symbol, Ctx::Error> {
        self.0(stack)
    }
}

#[derive(Clone)]
pub struct ParserRulesRunners<'a, Ctx>(Vec<ParserRuleRunner<'a, Ctx>>)
where Ctx: traits::Parser;  

impl<'a, Ctx> ParserRulesRunners<'a, Ctx> 
where Ctx: traits::Parser {
    pub fn new() -> Self {
        Self(vec![])
    }
    pub fn add<F: Fn(&mut ParserStack<Ctx>) -> Result<Ctx::Symbol, Ctx::Error> + Sync + Send>(&mut self, f: &'a F) -> &mut Self {
        self.0.push(ParserRuleRunner(f));
        self
    }
    
    pub fn get(&self, rule: usize) -> Option<&ParserRuleRunner<'_, Ctx>> {
        self.0.get(rule)
    }
}

pub struct ParserStack<Ctx: traits::Parser> {
    states: Vec<usize>,
    syms: Vec<Ctx::Symbol>,  
}

impl<Ctx: traits::Parser> ParserStack<Ctx> {
    pub fn new() -> Self {
        Self {
            states: vec![0],
            syms: vec![]
        }
    }
}

pub struct Parser<'table, 'rr, Ctx, Lexer> 
where Ctx: traits::Parser + 'static, 
        Lexer: lexer::traits::Lexer<Symbol = Ctx::Terminal>,
        Lexer::Error: Into<Ctx::Error>
{
    lexer: Lexer,
    stack: ParserStack<Ctx>,
    table: &'table ParserTable<Ctx>,
    rules_runners: &'rr ParserRulesRunners<'rr, Ctx>
}

impl<'table, 'rr, Ctx, Lexer> Parser<'table, 'rr, Ctx, Lexer> 
where Ctx: traits::Parser + 'static, 
        Lexer: lexer::traits::Lexer<Symbol = Ctx::Terminal>,
        Ctx::Error: From<Lexer::Error>
{
    pub fn new(lexer: Lexer, table: &'table ParserTable<Ctx>, rules_runners: &'rr ParserRulesRunners<'rr, Ctx>) -> Self {
        Self{lexer, stack: ParserStack::new(), table, rules_runners}
    }

    pub fn parse(&mut self) -> Result<Ctx::Symbol, Ctx::Error> {

        for tok in &mut self.lexer {
            let tok: Ctx::Terminal = tok.map_err(Ctx::Error::from)?;
            self.stack.syms.push(tok.clone().into());
            let state = self.table.get(*self.stack.states.last().unwrap()).unwrap();
            let action = state.get_action(&tok.get_type()).unwrap();

            match action.op {
                ParserOp::Shift(next_state) => {
                    // Shift to the next state
                    self.stack.states.push(next_state);
                },
                ParserOp::Reduce(rule_id) => {
                    // Reduce the stack by the given rule
                    let sym = self.rules_runners.get(rule_id).unwrap().execute(&mut self.stack)?;
                    self.stack.syms.push(sym.clone());

                    // fetch the next state to go to.
                    let state = self.table.get(*self.stack.states.last().unwrap()).unwrap();
                    let next_state = state.get_goto(&sym.get_type()).unwrap().next_state;
                    self.stack.states.push(next_state);
                },
                ParserOp::Accept => break,
            };
        }

        Ok(self.stack.syms.pop().unwrap())
    }
}

impl<Ctx> ParserStack<Ctx> 
where Ctx: traits::Parser + 'static
{
    pub fn pop(&mut self) -> Option<<Ctx as traits::Parser>::Symbol> {
        self.states.pop();
        self.syms.pop()
    }

    pub fn try_pop_into<F>(&mut self) 
    -> Option<Result<F, <Ctx as traits::Parser>::Error>> 
    where Ctx::Symbol: TryInto<F, Error = <Ctx as traits::Parser>::Error>
    {
        self
        .pop()
        .map(|sym |Ctx::Symbol::try_into(sym))
    }
}