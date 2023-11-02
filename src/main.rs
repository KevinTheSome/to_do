struct Todo{
    title: String,
    completed: bool,
    id: i32,
}

impl Todo{
    pub fn new(title: String, completed: bool, id: i32) -> Todo{
        Todo{
            title: title,
            completed: completed,
            id: id,
        }
    }
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    loop{
        println!("1 to add , 2 to delete, 3 to list, 4 to exit");

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).unwrap();


        match input.trim() {
            "1"=>{
                println!("Enter todo title");
                let mut title_input = String::new();
                let _ = std::io::stdin().read_line(&mut title_input).unwrap();
                println!("Enter todo state on completed (false/true)");
                let mut completed_input = String::new();
                let _ = std::io::stdin().read_line(&mut completed_input).unwrap();
                let mut todo = Todo::new(title_input.trim().to_string(), completed_input.trim().parse::<bool>().unwrap(), todos.len() as i32+1);
                todos.push(todo);
            }
            "2"=>{
                println!("Enter todo to delete (id)");
                let mut id_input = String::new();
                let _ = std::io::stdin().read_line(&mut id_input).unwrap();
                todos.remove(id_input.parse::<usize>().unwrap());
            }
            "3"=>{
                for todo in &todos{
                    println!("id: {} Title: {} completed:{}",todo.id , todo.title , todo.completed);
                }
            }
            "4"=>{
                println!("Have a good day!");
                break;
            }
            _ => {
                println!("Invalid input. Please enter 1 or 2.");
            }
        }
    }
}
