use json::{object, JsonValue};

pub struct Food {
    name: String,
    calories: (String, String),
    fats: f64,
    carbs: f64,
    proteins: f64,
    nbr_of_portions: f64
}

pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let mut c = 0.0;
    let mut p = 0.0;
    let mut f = 0.0;
    let mut cb = 0.0;

    for x in foods {
        let k = x
            .calories
            .1
            .trim_end_matches("kcal")
            .parse::<f64>()
            .unwrap();

        c += k * x.nbr_of_portions;
        p += x.proteins * x.nbr_of_portions;
        f += x.fats * x.nbr_of_portions;
        cb += x.carbs * x.nbr_of_portions;
    }

    object! {
        cals: (c * 100.0).round() / 100.0,
        carbs: (cb * 100.0).round() / 100.0,
        proteins: (p * 100.0).round() / 100.0,
        fats: (f * 100.0).round() / 100.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let foods = [
            Food {
                name: "big mac".to_owned(),
                calories: ("2133.84kJ".to_owned(), "510kcal".to_owned()),
                proteins: 27.,
                fats: 26.,
                carbs: 41.,
                nbr_of_portions: 2.,
            },
            Food {
                name: "pizza margherita".to_owned(),
                calories: ("1500.59kJ".to_owned(), "358.65kcal".to_owned()),
                proteins: 13.89,
                fats: 11.21,
                carbs: 49.07,
                nbr_of_portions: 4.9,
            },
        ];

        assert_eq!(
            calculate_macros(&foods),
            object! {
                cals: 2777.39,
                carbs: 322.44,
                proteins: 122.06,
                fats: 106.93,
            }
        );
    }
}
