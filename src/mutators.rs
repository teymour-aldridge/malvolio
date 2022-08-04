//! Mutators.
use std::{borrow::Cow, collections::HashMap};

use fuzzcheck::{
    mutators::{
        alternation::AlternationMutator,
        integer_within_range::U8WithinRangeMutator,
        map::MapMutator,
        tuples::{Tuple2Mutator, TupleMutatorWrapper},
        vector::VecMutator,
    },
    DefaultMutator, Mutator,
};

fn valid_attr_mutator() -> impl Mutator<u8> {
    AlternationMutator::new(vec![
        U8WithinRangeMutator::new(0x61..=0x7a),
        U8WithinRangeMutator::new(0x41..=0x5a),
        U8WithinRangeMutator::new(0x30..=0x39),
    ])
}

/// note: `LEN` denotes the minimum size of the string
pub fn valid_attr_string_mutator<const LEN: usize>() -> impl Mutator<String> {
    MapMutator::new(
        VecMutator::new(valid_attr_mutator(), LEN..=usize::MAX),
        |from: &String| Some(from.as_bytes().to_vec()),
        |vec: &Vec<u8>| String::from_utf8(vec.clone()).expect("failed to decode string"),
        |_, c| c,
    )
}

/// Attribute mutator
pub fn attr_mutator() -> impl Mutator<HashMap<Cow<'static, str>, Cow<'static, str>>> {
    MapMutator::new(
        VecMutator::new(
            TupleMutatorWrapper::new(Tuple2Mutator::new(
                valid_attr_string_mutator::<1>(),
                valid_attr_string_mutator::<0>(),
            )),
            0..=usize::MAX,
        ),
        |from: &HashMap<Cow<'static, str>, Cow<'static, str>>| {
            Some(
                from.clone()
                    .into_iter()
                    .map(|(a, b)| (a.to_string(), b.to_string()))
                    .collect::<Vec<_>>(),
            )
        },
        |vec| {
            vec.clone()
                .into_iter()
                .map(|(a, b)| (Cow::Owned(a), Cow::Owned(b)))
                .collect()
        },
        |_, cplx| cplx,
    )
}

/// Cow mutator.
pub fn cow_mutator() -> impl Mutator<Cow<'static, str>> {
    MapMutator::new(
        String::default_mutator(),
        |cow: &Cow<'static, str>| Some(cow.to_string()),
        |string: &String| Cow::Owned(string.clone()),
        |_, c| c,
    )
}
