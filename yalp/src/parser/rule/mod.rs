pub mod runner;

use core::convert::Infallible;
use std::collections::VecDeque;
use itertools::Itertools;

use crate::parser::traits::ParserGrammar;

use super::traits::ParserSymbolType;

#[derive(Clone)]
struct Rule<G> where G: ParserGrammar + 'static {
    id: usize,
    lhs: G::SymbolType,
    rhs: Vec<G::SymbolType>, 
    reducer: runner::ParserRuleRunner<'static, G, Infallible>
}

impl<G> PartialEq for Rule<G> where G: ParserGrammar + 'static  {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'a, G> Into<Item<'a, G>> for &'a Rule<G> where G: ParserGrammar + 'static  {
    fn into(self) -> Item<'a, G> {
        Item(self, 0)
    }
}

struct Item<'a, G>(&'a Rule<G>, usize) where G: ParserGrammar + 'static;

impl<'a, G> Clone for Item<'a, G> where G: ParserGrammar + 'static {
    fn clone(&self) -> Self {
        Self(self.0, self.1.clone())
    }
}

impl<'a, G> PartialEq for Item<'a, G> where G: ParserGrammar + 'static {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl<'a, G> Item<'a, G> where G: ParserGrammar + 'static 
{
    pub fn shift(&mut self) {
        self.1 += 1;
    }

    pub(self) fn next_symbol(&self) -> Option<G::SymbolType> {
        self.0.rhs.get(self.1).cloned()
    }   

    // Perform the closure of the item.
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

            let sym = item.next_symbol().unwrap();

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

struct ItemSet<'a, G>(Vec<Item<'a, G>>) where G: ParserGrammar + 'static;

impl<'a, G> Clone for ItemSet<'a, G> where G: ParserGrammar + 'static {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'a, G> PartialEq for ItemSet<'a, G> where G: ParserGrammar + 'static {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'a, G> ItemSet<'a, G> where G: ParserGrammar + 'static 
{
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn new_with_items<Itm: Into<Item<'a, G>>, It: IntoIterator<Item = Itm>>(items: It) -> Self {
        Self(
            items.into_iter().map(|i| i.into()).collect::<Vec<_>>()
        )
    }

    /// Returns the next item sets reachable from this set, depending on the next symbol.
    pub fn next_reachable_sets(&self, rules: &'a ParserRuleSet<G>) -> Vec<(G::SymbolType, ItemSet<'a, G>)> {
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
    fn split_by_symbol(&self) -> Vec<(G::SymbolType, ItemSet<'a, G>)> {
        self
        .0
        .clone()
        .into_iter()
        .filter(|i| i.next_symbol().is_some())
        .group_by(|i| i.next_symbol().unwrap())
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

#[derive(Clone)]
pub struct ParserRuleSet<G>(Vec<Rule<G>>) where G: ParserGrammar + 'static;

impl<G> ParserRuleSet<G> where G: ParserGrammar {
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Add a new rule
    pub fn add<I: IntoIterator<Item=G::SymbolType>>(&mut self, lhs: G::SymbolType, rhs: I, reducer: runner::ParserRuleRunner<'static, G, Infallible>) -> &mut Self {
        let id = self.0.len();
        self.0.push(
            Rule {
                id,
                lhs,
                rhs: rhs.into_iter().collect(),
                reducer
            }
        );
        self
    }

    pub(self) fn iter_by_lhs<'a>(&'a self, lhs: G::SymbolType) -> impl std::iter::Iterator<Item=&'a Rule<G>> {
        self.0.iter().filter(move |r| r.lhs == lhs)
    }

    pub fn generate_table(&self) {
        let item_sets = vec![ItemSet::new_with_items([self.0.first().unwrap()])];
    }
}   
