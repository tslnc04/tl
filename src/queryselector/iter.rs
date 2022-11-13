use std::marker::PhantomData;

use crate::{NodeHandle, Parser};

use super::{iterable::QueryIterable, Selector};

/// A query selector iterator that yields matching HTML nodes
pub struct QuerySelectorIterator<'a, 'b, Q: QueryIterable<'a>> {
    selector: Selector<'b>,
    collection: &'b Q,
    parser: &'b Parser<'a>,
    index: usize,
    len: usize,
    _a: PhantomData<&'a ()>,
}

impl<'a, 'b, Q: QueryIterable<'a>> Clone for QuerySelectorIterator<'a, 'b, Q> {
    fn clone(&self) -> Self {
        Self {
            selector: self.selector.clone(),
            collection: self.collection,
            parser: self.parser,
            index: self.index,
            len: self.len,
            _a: PhantomData,
        }
    }
}

impl<'a, 'b, Q: QueryIterable<'a>> QuerySelectorIterator<'a, 'b, Q> {
    /// Creates a new query selector iterator
    pub fn new(selector: Selector<'b>, parser: &'b Parser<'a>, collection: &'b Q) -> Self {
        Self {
            selector,
            collection,
            index: 0,
            len: collection.len(parser),
            parser,
            _a: PhantomData,
        }
    }
}

impl<'a, 'b, Q: QueryIterable<'a>> Iterator for QuerySelectorIterator<'a, 'b, Q> {
    type Item = NodeHandle;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.len {
            let node = self.collection.get(self.parser, self.index);
            self.index += 1;
            if let Some((node, id)) = node {
                let matches = self.selector.matches(node, self.parser);

                if matches {
                    return Some(id);
                }
            }
        }

        None
    }
}
