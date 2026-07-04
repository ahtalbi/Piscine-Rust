use std::collections::HashMap;

mod mall;

pub use mall::{Mall, Guard, Floor, Store, Employee};

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut best: Option<(String, Store)> = None;

    for floor in mall.floors.values() {
        for (name, store) in &floor.stores {
            match &best {
                None => best = Some((name.clone(), store.clone())),
                Some((_, b)) if store.square_meters > b.square_meters => {
                    best = Some((name.clone(), store.clone()));
                }
                _ => {}
            }
        }
    }

    best.unwrap()
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
        for store in &floor.stores {
            for (name, emp) in &store.1.employees {
                if (emp.salary - max).abs() < 0.00001 {
                    res.push((name, emp));
                }
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