mod employee_mut_ref;
mod employee_serde;
mod employee_struct;
mod loops;
mod mutex_stats;
mod rwlock_stats;

fn main() {
    loops::run();
    employee_struct::run();
    employee_serde::run();
    employee_mut_ref::run();
    mutex_stats::run();
    rwlock_stats::run();
}
