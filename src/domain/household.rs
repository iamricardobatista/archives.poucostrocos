#![allow(dead_code)]
pub struct Household {
    pub id: Option<i32>,
    pub name: String,
}

impl Household {
    pub fn from(household_name: String) -> Self {
        Household {
            id: None,
            name: household_name,
        }
    }

    pub fn from_id_and_name(id: i32, household_name: String) -> Self {
        Household {
            id: Some(id),
            name: household_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_a_name_when_creating_a_new_household_then_a_household_with_name_and_none_id_should_be_returned() {
        // GIVEN
        let household_name = String::from("A household");
        // WHEN
        let new_household = Household::from(household_name.clone());
        // THEN
        assert_eq!(new_household.id, None);
        assert_eq!(new_household.name, household_name);
    }

    #[test]
    fn given_an_id_and_a_name_when_creating_a_new_household_then_a_househould_with_the_same_id_and_name_should_be_returned() {
        // GIVEN
        let household_id = 1;
        let household_name = String::from("A household");
        // WHEN
        let new_household = Household::from_id_and_name(household_id, household_name.clone());
        // THEN
        assert_eq!(new_household.id, Some(household_id));
        assert_eq!(new_household.name, household_name);

    }
}
