use std::collections::HashMap;

mod mall;

pub use mall::{
    Mall,
    Guard,
    Floor,
    Store,
    Employee,
};

pub fn biggest_store(mall: &Mall) -> Option<(&String, &Store)> {
    let mut best = None;
    let mut max = 0;

    for floor in mall.floors.values() {
        for (name, store) in floor.stores.iter() {
            if store.square_meters > max {
                max = store.square_meters;
                best = Some((name, store));
            }
        }
    }

    best
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&String, &Employee)> {
    let mut max = 0.0;

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for emp in store.employees.values() {
                if emp.salary > max {
                    max = emp.salary;
                }
            }
        }
    }

    let mut res = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, emp) in store.employees.iter() {
                if (emp.salary - max).abs() < 0.00001 {
                    res.push((name, emp));
                }
            }
        }
    }

    res
}

pub fn employees(mall: &mut Mall) -> HashMap<String, &mut Employee> {
    let mut res = HashMap::new();

    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for (name, emp) in store.employees.iter_mut() {
                res.insert(name.clone(), emp);
            }
        }
    }

    res
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut count = mall.guards.len();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            count += store.employees.len();
        }
    }

    count
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut total = 0;

    for floor in mall.floors.values() {
        total += floor.size_limit;
    }

    let needed = (total / 200) as usize;

    let mut guards = guards;

    for (name, guard) in guards.drain() {
        if mall.guards.len() >= needed {
            break;
        }

        mall.hire_guard(name, guard);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for emp in store.employees.values_mut() {
                let hours = emp.working_hours.1 - emp.working_hours.0;

                if hours >= 10 {
                    emp.salary *= 1.10;
                } else {
                    emp.salary *= 0.90;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut mall = Mall::new(
            "La Vie Funchal",
            [
                (
                    "John Oliver",
                    Guard {
                        age: 34,
                        years_experience: 7,
                    },
                ),
                (
                    "Bob Schumacher",
                    Guard {
                        age: 53,
                        years_experience: 15,
                    },
                ),
            ]
            .into(),
            [
                (
                    "Ground Floor",
                    Floor::new(
                        [
                            (
                                "Footzo",
                                Store::new(
                                    [
                                        (
                                            "Finbar Haines",
                                            Employee {
                                                age: 36,
                                                working_hours: (9, 14),
                                                salary: 650.88,
                                            },
                                        ),
                                        (
                                            "Sienna-Rose Penn",
                                            Employee {
                                                age: 26,
                                                working_hours: (9, 22),
                                                salary: 1000.43,
                                            },
                                        ),
                                    ]
                                    .into(),
                                    50,
                                ),
                            ),
                            (
                                "Swashion",
                                Store::new(
                                    [
                                        (
                                            "Abdallah Stafford",
                                            Employee {
                                                age: 54,
                                                working_hours: (8, 22),
                                                salary: 1234.21,
                                            },
                                        ),
                                        (
                                            "Marian Snyder",
                                            Employee {
                                                age: 21,
                                                working_hours: (8, 14),
                                                salary: 831.9,
                                            },
                                        ),
                                    ]
                                    .into(),
                                    43,
                                ),
                            ),
                        ]
                        .into(),
                        300,
                    ),
                ),
                (
                    "Supermarket",
                    Floor::new(
                        [(
                            "Pretail",
                            Store::new(
                                [
                                    (
                                        "Yara Wickens",
                                        Employee {
                                            age: 39,
                                            working_hours: (9, 14),
                                            salary: 853.42,
                                        },
                                    ),
                                    (
                                        "Indiana Baxter",
                                        Employee {
                                            age: 33,
                                            working_hours: (13, 20),
                                            salary: 991.71,
                                        },
                                    ),
                                    (
                                        "Jadine Page",
                                        Employee {
                                            age: 48,
                                            working_hours: (13, 20),
                                            salary: 743.21,
                                        },
                                    ),
                                    (
                                        "Tyler Hunt",
                                        Employee {
                                            age: 63,
                                            working_hours: (13, 20),
                                            salary: 668.25,
                                        },
                                    ),
                                    (
                                        "Mohsin Mcgee",
                                        Employee {
                                            age: 30,
                                            working_hours: (19, 24),
                                            salary: 703.83,
                                        },
                                    ),
                                    (
                                        "Antoine Goulding",
                                        Employee {
                                            age: 45,
                                            working_hours: (19, 24),
                                            salary: 697.12,
                                        },
                                    ),
                                    (
                                        "Mark Barnard",
                                        Employee {
                                            age: 53,
                                            working_hours: (19, 24),
                                            salary: 788.81,
                                        },
                                    ),
                                ]
                                .into(),
                                950,
                            ),
                        )]
                        .into(),
                        1000,
                    ),
                ),
            ]
            .into(),
        );

        // returns the biggest store
        println!("Biggest store: {:#?}", biggest_store(&mall));

        // returns the list with the highest paid employees
        println!("Highest paid employee: {:#?}", highest_paid_employee(&mall));

        // returns the number of employees
        println!("Number of employees: {}", nbr_of_employees(&mall));

        // checks if it is needed to add securities
        check_for_securities(
            &mut mall,
            [
                (
                    "Peter Solomons",
                    Guard {
                        age: 45,
                        years_experience: 20,
                    },
                ),
                (
                    "William Charles",
                    Guard {
                        age: 32,
                        years_experience: 10,
                    },
                ),
                (
                    "Leonardo Changretta",
                    Guard {
                        age: 23,
                        years_experience: 0,
                    },
                ),
                (
                    "Vlad Levi",
                    Guard {
                        age: 38,
                        years_experience: 8,
                    },
                ),
                (
                    "Faruk Berkai",
                    Guard {
                        age: 40,
                        years_experience: 15,
                    },
                ),
                (
                    "Christopher Smith",
                    Guard {
                        age: 35,
                        years_experience: 9,
                    },
                ),
                (
                    "Jason Mackie",
                    Guard {
                        age: 26,
                        years_experience: 2,
                    },
                ),
                (
                    "Kenzie Mair",
                    Guard {
                        age: 34,
                        years_experience: 8,
                    },
                ),
                (
                    "Bentley Larson",
                    Guard {
                        age: 33,
                        years_experience: 10,
                    },
                ),
                (
                    "Ray Storey",
                    Guard {
                        age: 37,
                        years_experience: 12,
                    },
                ),
            ]
            .map(|(n, d)| (n.to_owned(), d))
            .into(),
        );

        // raises or cuts the salary of every employee
        cut_or_raise(&mut mall);

        println!("{:#?}", mall);
    }
}
