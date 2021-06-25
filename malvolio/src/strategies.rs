use std::{borrow::Cow, collections::HashMap};

use proptest::prelude::*;

pub(crate) fn hashmap_strategy(
) -> impl Strategy<Value = HashMap<Cow<'static, str>, Cow<'static, str>>> {
    prop::collection::vec((".+", ".*"), 0..100).prop_map(|attrs| {
        attrs
            .into_iter()
            .map(|(a, b)| (Cow::Owned(a), Cow::Owned(b)))
            .collect()
    })
}
