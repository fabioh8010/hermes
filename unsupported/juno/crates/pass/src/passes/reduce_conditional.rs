/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

//! Pass for optimizing conditional expressions where the conditional is a literal.
//!
//! For example, transforms
//! ```js
//! true ? a : b
//! ```
//! into
//! ```js
//! a
//! ```

use crate::Pass;
use juno::ast::*;

#[derive(Default)]
pub struct ReduceConditional {}

impl ReduceConditional {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Pass for ReduceConditional {
    fn name(&self) -> &'static str {
        "Reduce conditional"
    }
    fn description(&self) -> &'static str {
        "Transforms literal conditionals into whichever branch is taken"
    }
    fn run<'gc>(
        &mut self,
        gc: &'gc GCLock<'_, '_>,
        node: &'gc Node<'gc>,
    ) -> TransformResult<&'gc Node<'gc>> {
        VisitorMut::call(self, gc, node, None)
    }
}

impl<'gc> VisitorMut<'gc> for ReduceConditional {
    fn call(
        &mut self,
        gc: &'gc GCLock<'_, '_>,
        node: &'gc Node<'gc>,
        _parent: Option<&'gc Node<'gc>>,
    ) -> TransformResult<&'gc Node<'gc>> {
        if let Node::ConditionalExpression(ConditionalExpression {
            test: Node::BooleanLiteral(BooleanLiteral { value, .. }),
            consequent,
            alternate,
            ..
        }) = node
        {
            let reduced = if *value { consequent } else { alternate };
            return node.replace_with_existing(reduced, gc, self);
        }
        node.visit_children_mut(gc, self)
    }
}
