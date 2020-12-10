use gumdrop::Options;
use serde::{Deserialize, Serialize};
use simple_error::*;
use crate::{
    validator::{
        Validator as Validator,
        generate_validators,
    },
    Generator,
    helpers::*
};
use tendermint::{validator};

#[derive(Debug, Options, Serialize, Deserialize, Clone)]
pub struct ValidatorSet {
    #[options(
        parse(try_from_str = "parse_as::<Vec<Validator>>"),
        help = "validators (required)"
    )]
    validators: Option<Vec<Validator>>,
}

impl ValidatorSet {
    pub fn new(ids: Vec<&str>) -> Self {
        let validators = ids
            .iter()
            .map(|v| Validator::new(v))
            .collect::<Vec<Validator>>();
        Self{
            validators: Some(validators),
        }
    }
}

impl std::str::FromStr for ValidatorSet {
    type Err = SimpleError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let validator_set = match parse_as::<ValidatorSet>(s) {
            Ok(input) => input,
            Err(_) => ValidatorSet::new(vec![s]),
        };
        Ok(validator_set)
    }
}

impl Generator<validator::Set> for ValidatorSet {
    fn merge_with_default(self, default: Self) -> Self {
        ValidatorSet {
            validators: self.validators.or(default.validators),
        }
    }

    fn generate(&self) -> Result<validator::Set, SimpleError> {
        let vals = generate_validators(&self.validators.as_ref().unwrap())?;
        Ok(
            validator::Set::new_simple(vals)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_set() {
        let valset1 = ValidatorSet::new(vec!["a", "b", "c"]).generate().unwrap();
        let vals1 = vec![
            Validator::new("a"),
            Validator::new("b"),
            Validator::new("c"),
            ];
        let valset2 = validator::Set::new_simple(generate_validators(&vals1).unwrap());

        assert_eq!(valset1.hash(), valset2.hash());

        let valset3 = ValidatorSet::new(vec!["b", "c", "a"]).generate().unwrap();

        assert_eq!(valset1.hash(), valset3.hash());

        let valset4 = ValidatorSet::new(vec!["c", "a"]).generate().unwrap();

        assert_ne!(valset4.hash(), valset3.hash());

        let vals2 = vec![
            Validator::new("a").voting_power(100),
            Validator::new("b"),
            Validator::new("c"),
        ];
        let valset5 = validator::Set::new_simple(generate_validators(&vals2).unwrap());
        assert_ne!(valset2.hash(), valset5.hash());
    }

}