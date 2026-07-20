use std::collections::HashMap;

mod mall;

pub use mall::{Mall, Guard, Floor, Store, Employee};

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut best: Option<(String, Store)> = None;

    for floor in &mall.floors {
        for (name, store) in &floor.1.stores {
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

    for floor in &mall.floors {
        for store in &floor.1.stores {
            for employee in &store.1.employees {
                if employee.1.salary > max {
                    max = employee.1.salary;
                }
            }
        }
    }

    let mut highest: Vec<(&String, &Employee)> = Vec::new();

    for floor in &mall.floors {
        for store in &floor.1.stores {
            for (name, employee) in &store.1.employees {
                if (employee.salary - max).abs() < 0.00001 {
                    highest.push((name, employee));
                }
            }
        }
    }

    highest
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut number_employee: usize = mall.guards.len();

    for floor in &mall.floors {
        for store in &floor.1.stores {
            number_employee += store.1.employees.len();
        }
    }

    number_employee
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut total = 0;

    for floor in &mall.floors {
        total += floor.1.size_limit;
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
            for employee in store.employees.values_mut() {
                let working_hours = employee.working_hours.1 - employee.working_hours.0;

                if working_hours >= 10 {
                    employee.raise(employee.salary * 0.1);
                } else {
                    employee.cut(employee.salary * 0.1);
                }
            }
        }
    }
}