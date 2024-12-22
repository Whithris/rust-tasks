// Используя хеш-карту и векторы, создайте текстовый интерфейс позволяющий пользователю добавлять имена сотрудников к названию отдела компании. Например, "Add Sally to Engineering" или "Add Amir to Sales". Затем позвольте пользователю получить список всех людей из отдела или всех людей в компании, отсортированных по отделам в алфавитном порядке.
use std::collections::HashMap;
use std::io;

fn main() {
    let mut department_structure: HashMap<&str, Vec<&str>> = HashMap::new();
    println!("Input command ('Add (employee_name) to (department_name)', 'Get employees from (department_name)', 
    'Get department structure' or 'Stop' to exit): ");
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("Error during input");
        if input.trim() == "Stop" {
            break;
        }
        // Get department structure 
        else if input.trim() == "Get department structure" {
            for (key, value) in &department_structure {
                print!("{}: ", key);
                for empl in value {
                    print!("{} ", empl);
                }
                println!("");
            }
        }
        // Get employees from {department_name}
        else if input.trim().split(" ").nth(0).unwrap() == "Get" {
            let department_name = input.trim().split(" ").nth(3).unwrap();
            for employee in department_structure.get_mut(department_name).unwrap() {
                println!("{}", employee);
            }
        }
        // Add {employee_name} to {department_name}
        else if input.trim().split(" ").nth(0).unwrap() == "Add" {
            let employee_name = input.trim().split(" ").nth(1).unwrap();
            let department_name = input.trim().split(" ").nth(3).unwrap();
            department_structure.get_mut(department_name).unwrap().push(employee_name);
        }
    }
}
