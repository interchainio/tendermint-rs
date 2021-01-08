use std::{collections::HashMap, time::Duration};

use tendermint::Time;

use tendermint_light_client::{
    components::{
        io::{AtHeight, Io},
        scheduler,
        verifier::ProdVerifier,
    },
    errors::Error,
    light_client::{LightClient, Options},
    operations::ProdHasher,
    state::State,
    store::{memory::MemoryStore, LightStore},
    tests::{MockClock, MockIo},
    types::{Height, LightBlock, Status},
};

use tendermint_testgen::{
    light_block::{default_peer_id, TMLightBlock as TGLightBlock},
    Generator, LightChain,
};

use proptest::prelude::*;

fn testgen_to_lb(tm_lb: TGLightBlock) -> LightBlock {
    LightBlock {
        signed_header: tm_lb.signed_header,
        validators: tm_lb.validators,
        next_validators: tm_lb.next_validators,
        provider: tm_lb.provider,
    }
}

#[derive(Clone, Debug)]
struct TestCase {
    length: u32,
    chain: LightChain,
    target_height: Height,
    trusted_height: Height,
}

fn make(chain: LightChain, trusted_height: Height) -> (LightClient, State) {
    let primary = default_peer_id();
    let chain_id = "testchain-1".parse().unwrap();

    let clock = MockClock { now: Time::now() };

    let options = Options {
        trust_threshold: Default::default(),
        trusting_period: Duration::from_secs(60 * 60 * 24 * 10),
        clock_drift: Duration::from_secs(10),
    };

    let light_blocks = chain
        .light_blocks
        .into_iter()
        .map(|lb| lb.generate().unwrap())
        .map(testgen_to_lb)
        .collect();

    let io = MockIo::new(chain_id, light_blocks);

    let trusted_state = io
        .fetch_light_block(AtHeight::At(trusted_height))
        .expect("could not find trusted light block");

    let mut light_store = MemoryStore::new();
    light_store.insert(trusted_state, Status::Trusted);

    let state = State {
        light_store: Box::new(light_store),
        verification_trace: HashMap::new(),
    };

    let verifier = ProdVerifier::default();
    let hasher = ProdHasher::default();

    let light_client = LightClient::new(
        primary,
        options,
        clock,
        scheduler::basic_bisecting_schedule,
        verifier,
        hasher,
        io,
    );

    (light_client, state)
}

fn verify(tc: TestCase) -> Result<LightBlock, Error> {
    let (light_client, mut state) = make(tc.chain, tc.trusted_height);
    light_client.verify_to_target(tc.target_height, &mut state)
}

fn ok_test(tc: TestCase) -> Result<(), TestCaseError> {
    let target_height = tc.target_height;
    let result = verify(tc);

    prop_assert!(result.is_ok());
    prop_assert_eq!(result.unwrap().height(), target_height);

    Ok(())
}

// fn bad_test(tc: TestCase) -> Result<(), TestCaseError> {
//     let result = verify(tc);

//     prop_assert!(result.is_err());

//     Ok(())
// }

fn testcase(max: u32) -> impl Strategy<Value = TestCase> {
    (1..=max).prop_flat_map(move |length| {
        (1..=length).prop_flat_map(move |trusted_height| {
            (1..=trusted_height).prop_map(move |target_height| TestCase {
                chain: LightChain::default_with_length(length as u64),
                length,
                trusted_height: trusted_height.into(),
                target_height: target_height.into(),
            })
        })
    })
}

// fn mutate(tc: &mut TestCase) {
//     let trusted = &mut tc.chain.light_blocks[tc.trusted_height.value() as usize - 1];
//     if let Some(header) = trusted.header.as_mut() {
//         header.last_block_id_hash = None;
//     }
// }

proptest! {
    #![proptest_config(ProptestConfig::with_cases(5))]

    #[test]
    fn prop_target_equal_trusted_first_block(mut tc in testcase(100)) {
        tc.target_height = 1_u32.into();
        tc.trusted_height = 1_u32.into();
        ok_test(tc)?;
    }

    #[test]
    fn prop_target_equal_trusted_last_block(mut tc in testcase(100)) {
        tc.target_height = tc.length.into();
        tc.trusted_height = tc.length.into();
        ok_test(tc)?;
    }

    #[test]
    fn prop_target_equal_trusted(mut tc in testcase(100)) {
        tc.target_height = tc.trusted_height;
        ok_test(tc)?;
    }

    #[test]
    fn prop_two_ends(mut tc in testcase(100)) {
        tc.target_height = 1_u32.into();
        tc.trusted_height = tc.length.into();
        ok_test(tc)?;
    }

    #[test]
    fn prop_target_less_than_trusted(tc in testcase(100)) {
        ok_test(tc)?;
    }

    // #[test]
    // fn bad(mut tc in testcase(100)) {
    //     mutate(&mut tc);
    //     bad_test(tc)?;
    // }
}