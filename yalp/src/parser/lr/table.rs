use std::{collections::VecDeque, fmt::format};

use itertools::Itertools;

use crate::{parser::{rule::{ParserRuleSet, ParserRule}, traits::{ParserSymbol, ParserSymbolType, TerminalSymbol}}, lexer::traits::LexerSymbol};
use super::{state::LrParserState, action::{LrParserAction, LrParserOp}, goto::LrParserGoto};

#[derive(Clone)]
pub(super) struct LrParserTable<G: ParserSymbol>(Vec<LrParserState<G>>);

impl<G: ParserSymbol> FromIterator<LrParserState<G>> for LrParserTable<G> 
{
    fn from_iter<T: IntoIterator<Item = LrParserState<G>>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<G: ParserSymbol> LrParserTable<G> 
{
    pub fn get(&self, state: usize) -> Option<&LrParserState<G>> {
        self.0.get(state)
    }

    /// Generate the LrParserTable
    pub fn generate(rules: &ParserRuleSet<G>) -> Self {
        let item_sets_table = ItemSetTable::<'_, G>::build(rules);

        item_sets_table.iter().map(|s| {
            LrParserState::new_from_iterators(
                s.iter_actions(), 
                s.iter_gotos()
            )
        }).collect()
    }
}

impl<'a, G> Into<Item<'a, G>> for &'a ParserRule<G> where G: ParserSymbol + 'static  {
    fn into(self) -> Item<'a, G> {
        Item(self, 0)
    }
}

struct Item<'a, G>(&'a ParserRule<G>, usize) where G: ParserSymbol + 'static;
impl<'a, G> Clone for Item<'a, G> where G: ParserSymbol + 'static {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}
impl<'a, G> PartialEq for Item<'a, G> where G: ParserSymbol + 'static {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl<'a, G> Item<'a, G> where G: ParserSymbol + 'static {
    pub fn rule_id(&self) -> usize {
        self.0.id
    }

    pub fn shift(&mut self) {
        self.1 += 1;
    }

    pub(self) fn next_symbol(&self) -> Option<G::Type> {
        self.0.rhs.get(self.1).cloned()
    }   

    // Build the closure of the item.
    pub(self) fn close(self, rules: &'a ParserRuleSet<G>) -> ItemSet<'a, G> {
        let mut set: ItemSet<'_, G> = ItemSet::new();
        let mut stack: VecDeque<Item<'_, G>> = vec![self].into();
        
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
            .for_each(|item| stack.push_front(item));
        }

        set
    }
}

struct ItemSet<'a, G>(Vec<Item<'a, G>>) where G: ParserSymbol + 'static;
impl<'a, G> Clone for ItemSet<'a, G> where G: ParserSymbol + 'static {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<'a, G> PartialEq for ItemSet<'a, G> where G: ParserSymbol + 'static {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<'a, G> ItemSet<'a, G> where G: ParserSymbol + 'static {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn new_with_items<Itm: Into<Item<'a, G>>, It: IntoIterator<Item = Itm>>(items: It) -> Self {
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
    pub fn next_reachable_sets(&self, rules: &'a ParserRuleSet<G>) -> Vec<(G::Type, ItemSet<'a, G>)> {
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

    /// Push a new item in the set
    pub fn push(&mut self, item: Item<'a, G>) -> bool {
        let size = self.0.len();
        self.0.push(item);
        self.0.dedup();

        return size != self.0.len()
    }

    /// Append new items in the set
    pub fn append<I: IntoIterator<Item=Item<'a, G>>>(&mut self, items: I) {
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
    fn split_by_symbol(&self) -> Vec<(G::Type, ItemSet<'a, G>)> {
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
    fn close(&mut self, rules: &'a ParserRuleSet<G>) -> &mut Self {
        self.0
            .clone()
            .into_iter()
            .map(|item| item.close(rules))
            .for_each(|subset| self.append(subset.0));
        self
    }
}

struct ItemSetState<'a, G> where G: ParserSymbol + 'static {
    id:     usize,
    set:    ItemSet<'a, G>,
    next_states:   Vec<(G::Type, usize)>
}

impl<'a, G> ItemSetState<'a, G> where G: ParserSymbol + 'static {

    fn iter_terminal_symbols<'b>(&'b self) -> impl Iterator<Item=<G::Terminal as LexerSymbol>::Type> + 'b {
        self.next_states
        .iter()
        .map(|(sym_type, _)| sym_type)
        .cloned()
        .filter(|sym_type| sym_type.is_terminal())
        .map(|sym_type| sym_type.expect_terminal_type())
    }

    fn iter_terminal_transitions<'b>(&'b self) -> impl Iterator<Item=(<G::Terminal as LexerSymbol>::Type, usize)> + 'b {
        self.next_states
        .iter()
        .cloned()
        .filter(|(sym_type, _)| sym_type.is_terminal())
        .map(|(sym_type, next_state)| (sym_type.expect_terminal_type(), next_state))     
    }

    pub fn iter_gotos<'b>(&'b self) -> impl Iterator<Item=LrParserGoto<G>> + 'b {
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
    pub fn iter_actions<'b>(&'b self) -> Box<dyn Iterator<Item=LrParserAction<G>> + 'b> {
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

struct ItemSetTable<'a, G>(Vec<ItemSetState<'a, G>>) where G: ParserSymbol + 'static;

impl<'a, G> Default for ItemSetTable<'a, G> where G: ParserSymbol + 'static {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'a, G> ItemSetTable<'a, G> where G: ParserSymbol + 'static {
    /// Build the item set transition table
    pub fn build(rules: &'a ParserRuleSet<G>) -> Self {
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

    pub fn iter(&self) -> impl Iterator<Item=&ItemSetState<'a, G>> {
        self.0.iter()
    }

    /// Add the ItemSet as a new, or existing state.
    /// If the set is already bound to a state, does not create a new state. 
    /// Returns (inserted, state id)
    pub fn new_state(&mut self, set: ItemSet<'a, G>) -> usize {
        if let Some(state) = self.find_by_set(&set) {
            return state.id;
        }

        let id = self.0.len();
        let state = ItemSetState{id, set, next_states: vec![]};
        self.0.push(state);
        return id;
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut ItemSetState<'a, G>> {
        self.0.get_mut(id)
    }

    pub fn get(&self, id: usize) -> Option<&ItemSetState<'a, G>> {
        self.0.get(id)
    }

    /// Find a state by its item set.
    pub fn find_by_set(&self, set: &ItemSet<'a, G>) -> Option<&ItemSetState<'a, G>> {
        self.0.iter().find(|state| state.set == *set)
    }
}

