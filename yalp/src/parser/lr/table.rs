use std::{collections::VecDeque, fmt::Debug};

use itertools::Itertools;

use crate::{parser::{rule::{ParserRuleSet, ParserRule}, traits::ParserSymbolClass}, symbol::{traits::SymbolDefinition, Sym}};
use super::{state::LrParserState, action::{LrParserAction, LrParserOp}, goto::LrParserGoto};

#[derive(Clone)]
pub(super) struct LrParserTable<SymDef: SymbolDefinition>(Vec<LrParserState<SymDef>>);

impl<SymDef> Debug for LrParserTable<SymDef> where SymDef: SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("LrParserTable").field(&self.0).finish()
    }
}

impl<SymDef: SymbolDefinition> FromIterator<LrParserState<SymDef>> for LrParserTable<SymDef> 
{
    fn from_iter<T: IntoIterator<Item = LrParserState<SymDef>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<SymDef> LrParserTable<SymDef> 
where SymDef: SymbolDefinition, SymDef::Class: ParserSymbolClass
{
    /// Get the state by its id
    pub fn get(&self, state: usize) -> Option<&LrParserState<SymDef>> {
        self.0.get(state)
    }

    /// Generate the LrParserTable
    pub fn generate(rules: &ParserRuleSet<SymDef>) -> Self {
        let item_sets_table = ItemSetTable::<'_, >::build(rules);

        item_sets_table.iter().map(|s| {
            LrParserState::new_from_iterators(
                s.iter_actions(), 
                s.iter_gotos()
            )
        }).collect()
    }
}

impl<'a, G> Into<Item<'a, G>> for &'a ParserRule<G> where G: SymbolDefinition + 'static  {
    fn into(self) -> Item<'a, G> {
        Item(self, 0)
    }
}

struct Item<'a, G>(&'a ParserRule<G>, usize) where G: SymbolDefinition + 'static;

impl<'a, SymDef> Debug for Item<'a, SymDef> where SymDef: SymbolDefinition{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Item").field(&self.0).field(&self.1).finish()
    }
}

impl<'a, G> Clone for Item<'a, G> where G: SymbolDefinition + 'static {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}
impl<'a, G> PartialEq for Item<'a, G> where G: SymbolDefinition + 'static {
    fn eq(&self, other: &Self) -> bool {
        *self.0 == *other.0 && self.1 == other.1
    }
}
impl<'a, SymDef> Item<'a, SymDef> 
where SymDef: SymbolDefinition, SymDef::Class: ParserSymbolClass {
    pub fn new(rule: &'a ParserRule<SymDef>, cursor: usize) -> Self {
        Self(rule, cursor)
    }

    pub fn rule_id(&self) -> usize {
        self.0.id
    }

    pub fn shift(&mut self) {
        self.1 += 1;
    }

    pub(self) fn next_symbol(&self) -> Option<SymDef::Class> {
        self.0.rhs.get(self.1).cloned()
    }   

    // Build the closure of the item.
    pub(self) fn close(self, rules: &'a ParserRuleSet<SymDef>) -> ItemSet<'a, SymDef> {
        let mut set: ItemSet<'_, SymDef> = ItemSet::new();
        let mut stack: VecDeque<Item<'_, SymDef>> = vec![self].into();
        
        while let Some(item) = stack.pop_front() {
            // Add to the item set, if it returns false, we have already pushed the item in the set.
            if !set.push(item.clone()) {
                continue;
            }

            // No more symbols
            if item.next_symbol().is_none() {
                continue;
            }

            let sym = item.next_symbol().expect("item should have a symbol");

            // We found our terminal
            if sym.is_terminal() {
                continue;
            }

            // add our item in the stack for the next loop.
            rules
            .iter_by_lhs(sym)
            .map(|r| r.into())
            .for_each(|item| stack.push_back(item));
        }

        set
    }
}

struct ItemSet<'a, G>(Vec<Item<'a, G>>) where G: SymbolDefinition;

impl<'a, SymDef> Debug for ItemSet<'a, SymDef> where SymDef: SymbolDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ItemSet").field(&self.0).finish()
    }
}

impl<'a, SymDef> FromIterator<Item<'a, SymDef>> for ItemSet<'a, SymDef> where SymDef: SymbolDefinition {
    fn from_iter<T: IntoIterator<Item = Item<'a, SymDef>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a, G> Clone for ItemSet<'a, G> where G: SymbolDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<'a, G> PartialEq for ItemSet<'a, G> where G: SymbolDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<'a, SymDef> ItemSet<'a, SymDef> 
where SymDef: SymbolDefinition, SymDef::Class: ParserSymbolClass
{
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn new_with_items<Itm: Into<Item<'a, SymDef>>, It: IntoIterator<Item = Itm>>(items: It) -> Self {
        Self(
            items.into_iter().map(|i| i.into()).collect::<Vec<_>>()
        )
    }

    /// Check if the item set is reducing
    /// Returns the rule id by which the reduction occurs.
    pub fn is_reducing(&self) -> Option<usize> {
        self.0
        .iter()
        .find(|i| i.next_symbol().is_none())
        .map(|i| i.rule_id())
    }

    /// Returns the next item sets reachable from this set, grouped by the next symbol.
    pub fn next_reachable_sets(&self, rules: &'a ParserRuleSet<SymDef>) -> Vec<(SymDef::Class, ItemSet<'a, SymDef>)> {
        self
        .split_by_symbol()
        .into_iter()
        // Generate next item sets
        .map(|(sym, mut set)| {
            (sym, set.shift().close(rules).to_owned())
        })
        // Remove empty sets
        .filter(|(_, set)| !set.is_empty())
        .collect::<Vec<_>>()
    }

    /// Push a new item in the set, returns true if inserted
    pub fn push(&mut self, item: Item<'a, SymDef>) -> bool {
        if self.0.contains(&item) {
            return false;
        }
        self.0.push(item);
        return true;
    }

    /// Append new items in the set
    pub fn append<I: IntoIterator<Item=Item<'a, SymDef>>>(&mut self, items: I) {
        self.0.extend(items);
        self.0.dedup();
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Shift each item in the set
    fn shift(&mut self) -> &mut Self {
        self.0.iter_mut().filter(|i| i.next_symbol().is_some()).for_each(|i| i.shift());
        self
    }

    // Create a subset based on the next symbol for each item as group key.
    fn split_by_symbol(&self) -> Vec<(SymDef::Class, ItemSet<'a, SymDef>)> {
        self
        .0
        .clone()
        .into_iter()
        .filter(|i| i.next_symbol().is_some())
        .group_by(|i| i.next_symbol().expect("item should have a symbol"))
        .into_iter()
        .map(|(sym, group)| {
            (sym, ItemSet(group.collect()))
        }).collect::<Vec<_>>()
    }

    /// Close the item set, returns true if new item was added
    fn close(&mut self, rules: &'a ParserRuleSet<SymDef>) -> &mut Self {
        self.0
            .clone()
            .into_iter()
            .map(|item| item.close(rules))
            .for_each(|subset| self.append(subset.0));
        self
    }
}

struct ItemSetState<'a, G> where G: SymbolDefinition {
    id:     usize,
    set:    ItemSet<'a, G>,
    next_states:   Vec<(G::Class, usize)>
}

impl<'a, SymDef> ItemSetState<'a, SymDef> 
    where SymDef: SymbolDefinition, SymDef::Class: ParserSymbolClass {
    fn iter_terminal_symbols<'b>(&'b self) -> impl Iterator<Item=SymDef::Class> + 'b {
        self.next_states
        .iter()
        .map(|(sym_type, _)| sym_type)
        .cloned()
        .filter(|sym_type| sym_type.is_terminal())
        .map(|sym_type| sym_type)
    }

    fn iter_terminal_transitions<'b>(&'b self) -> impl Iterator<Item=(SymDef::Class, usize)> + 'b {
        self.next_states
        .iter()
        .cloned()
        .filter(|(sym_type, _)| sym_type.is_terminal())
        .map(|(sym_type, next_state)| (sym_type, next_state))     
    }

    pub fn iter_gotos<'b>(&'b self) -> impl Iterator<Item=LrParserGoto<SymDef>> + 'b {
        self.next_states
        .iter()
        .cloned()
        .filter(|(sym_type, _)| !sym_type.is_terminal())  
        .map(|(sym_type, next_state)| {
            LrParserGoto {
                r#type: sym_type,
                next_state
            }
        })
    }

    /// Iterate over action-based state transitions
    pub fn iter_actions<'b>(&'b self) -> Box<dyn Iterator<Item=LrParserAction<SymDef>> + 'b> {
        if let Some(rule_id) = self.set.is_reducing() {
            let it = self
            .iter_terminal_symbols()
            .map(move |term| {
                LrParserAction {
                    r#type: term,
                    op: LrParserOp::Reduce(rule_id)
                }
            });

            Box::new(it)
        } else {
            let it = self
            .iter_terminal_transitions()
            .map(|(sym_type, next_state)| {
                LrParserAction {
                    r#type: sym_type,
                    op: LrParserOp::Shift(next_state)
                }
            });

            Box::new(it)
        }
    }
}

struct ItemSetTable<'a, G>(Vec<ItemSetState<'a, G>>) where G: SymbolDefinition + 'static;

impl<'a, G> Default for ItemSetTable<'a, G> where G: SymbolDefinition + 'static {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'a, SymDef> ItemSetTable<'a, SymDef>     
where SymDef: SymbolDefinition, SymDef::Class: ParserSymbolClass 
{
    /// Build the item set transition table
    pub fn build(rules: &'a ParserRuleSet<SymDef>) -> Self {
        let mut table = Self::default();
        let mut stack: VecDeque<usize> = vec![
            table.new_state(ItemSet::new_with_items([rules.root().expect("missing root rule")]))
        ].into();
        
        while let Some(state_id) = stack.pop_back() {
            // Get the next states
            let next_states = table
            .get(state_id)
            .expect(&format!("missing state {state_id}"))
            .set
            .to_owned()
            .next_reachable_sets(rules)
            .into_iter()
            .map(|(sym, set)| {
                if let Some(state) = table.find_by_set(&set) {
                    return (sym, state.id);
                }

                let state_id = table.new_state(set);
                stack.push_front(state_id);
                return (sym, state_id);
            })
            .collect::<Vec<_>>();
            
            table.get_mut(state_id).expect(&format!("missing state {state_id}")).next_states = next_states;
        }

        table
    }

    pub fn iter(&self) -> impl Iterator<Item=&ItemSetState<'a, SymDef>> {
        self.0.iter()
    }

    /// Add the ItemSet as a new, or existing state.
    /// If the set is already bound to a state, does not create a new state. 
    /// Returns (inserted, state id)
    pub fn new_state(&mut self, set: ItemSet<'a, SymDef>) -> usize {
        if let Some(state) = self.find_by_set(&set) {
            return state.id;
        }

        let id = self.0.len();
        let state = ItemSetState{id, set, next_states: vec![]};
        self.0.push(state);
        return id;
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut ItemSetState<'a, SymDef>> {
        self.0.get_mut(id)
    }

    pub fn get(&self, id: usize) -> Option<&ItemSetState<'a, SymDef>> {
        self.0.get(id)
    }

    /// Find a state by its item set.
    pub fn find_by_set(&self, set: &ItemSet<'a, SymDef>) -> Option<&ItemSetState<'a, SymDef>> {
        self.0.iter().find(|state| state.set == *set)
    }
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;

    use crate::{parser::{traits::ParserSymbolClass, rule::ParserRuleSet, ParserError}, symbol::{traits::SymbolDefinition, Sym}};

    use super::{Item, ItemSet, LrParserTable};


    #[derive(Clone, Debug, PartialEq)]
    enum Class {
        S,
        E,
        B,
        Zero,
        One,
        Mult,
        Plus
    }

    impl ParserSymbolClass for Class {
        fn is_terminal(&self) -> bool {
            match self {
                Class::Zero => true,
                Class::One => true,
                Class::Mult => true,
                Class::Plus => true,
                _ => false
            }
        }
    }

    #[derive(Clone, Debug)]
    enum Value {
        S(E),
        E(E),
        B(B),
        Zero(Zero),
        One(One),
        Mult(Mult),
        Plus(Plus)
    }

    struct SymDef;

    impl SymbolDefinition for SymDef {
        type Class = Class;
        type Value = Value;
    }

    #[derive(Clone, Debug)]
    struct E {
        lhs: B,
        operations: Vec<(BinOp, B)>
    }

    impl Into<Value> for E {
        fn into(self) -> Value {
            Value::E(self)
        }
    }

    impl TryFrom<Sym<SymDef>> for E {
        type Error = ParserError;

        fn try_from(sym: Sym<SymDef>) -> Result<Self, Self::Error> {
            match sym.value {
                Value::E(e) => Ok(e),
                _ => Err(ParserError::unexpected_token(sym, vec![Class::E]))
            }
        }
    }

    #[derive(Clone, Debug)]
    struct B(usize);

    impl Into<Value> for B {
        fn into(self) -> Value {
            Value::B(self)
        }
    }

    impl TryFrom<Sym<SymDef>> for B {
        type Error = ParserError;

        fn try_from(sym: Sym<SymDef>) -> Result<Self, Self::Error> {
            match sym.value {
                Value::B(b) => Ok(b),
                _ => Err(ParserError::unexpected_token(sym, vec![Class::B]))
            }
        }
    }

    #[derive(Clone, Debug)]
    struct Zero;

    impl TryFrom<Sym<SymDef>> for Zero {
        type Error = ParserError;

        fn try_from(sym: Sym<SymDef>) -> Result<Self, Self::Error> {
            match sym.value {
                Value::Zero(z) => Ok(z),
                _ => Err(ParserError::unexpected_token(sym, vec![Class::Zero]))
            }
        }
    }

    #[derive(Clone, Debug)]
    struct One;

    impl TryFrom<Sym<SymDef>> for One {
        type Error = ParserError;

        fn try_from(sym: Sym<SymDef>) -> Result<Self, Self::Error> {
            match sym.value {
                Value::One(o) => Ok(o),
                _ => Err(ParserError::unexpected_token(sym, vec![Class::One]))
            }
        }
    }

    #[derive(Clone, Debug)]
    enum BinOp {
        Plus,
        Mult
    }

    #[derive(Clone, Debug)]
    struct Mult;
    
    #[derive(Clone, Debug)]
    struct Plus;

    lazy_static! {
        static ref RULES: ParserRuleSet<SymDef> = ParserRuleSet::new()
        .add( // S -> E
            Class::S, [Class::E],
            &|mut syms| {
                Ok(syms.remove(0).value)
            }
        )
        .add( // E → E * B 
            Class::E, [Class::E, Class::Mult, Class::B],
            &|mut syms| {
                let mut e: E = syms.remove(0).try_into()?;
                let b: B = syms.remove(2).try_into()?;
                e.operations.push((BinOp::Mult, b));
                Ok(e.into())
            }
        )
        .add( // E → E + B
            Class::E, [Class::E, Class::Plus, Class::B],
            &|mut syms| {
                let mut e: E = syms.remove(0).try_into()?;
                let b: B = syms.remove(2).try_into()?;
                e.operations.push((BinOp::Plus, b));
                Ok(e.into())
            }            
        ).add( // E → B
            Class::E, [Class::B],
            &|mut syms| {
                let b: B = syms.remove(0).try_into()?;
                Ok(E{lhs: b, operations: vec![]}.into())
            }         
        ).add( // B → 0
            Class::B, [Class::Zero],
            &|_| {
                Ok(B(0).into())
            }
        ).add( // B → 1
            Class::B, [Class::One],
            &|_| {
                Ok(B(1).into())
            }
        )
        .to_owned();
    }

    #[test]
    fn item_closure() {
        let r0 = RULES.root().unwrap();
        let item: Item<'_, SymDef> = r0.into();

        let closed_set = item.close(&RULES);
        let expected_set = ItemSet::from_iter([
            Item::new(RULES.get(0).unwrap(), 0),
            Item::new(RULES.get(1).unwrap(), 0),
            Item::new(RULES.get(2).unwrap(), 0),
            Item::new(RULES.get(3).unwrap(), 0),
            Item::new(RULES.get(4).unwrap(), 0),
            Item::new(RULES.get(5).unwrap(), 0)
        ]);

        assert_eq!(closed_set, expected_set);
    }

    #[test]
    fn table_generation() {
        let table = LrParserTable::generate(&RULES);
        println!("{:?}", table)
    }
}