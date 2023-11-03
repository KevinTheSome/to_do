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

fn from_string_to_i32(i_string:String) -> i32{
    return i_string.trim().parse::<i32>().expect("int 32 expected");
}

fn from_string_to_bool(i_string:String) -> bool{
    return i_string.trim().parse::<bool>().expect("boolean expected");
}

fn main() {
    let mut todos: Vec<Todo> = Vec::new();
    loop{
        println!("1 to add , 2 to delete, 3 to list, 4 edit todo completed state , 5 to exit");

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
                let mut todo = Todo::new(title_input.trim().to_string(), from_string_to_bool(completed_input), todos.len() as i32+1);
                todos.push(todo);
            }
            "2"=>{
                println!("Enter todo to delete (id)");
                let mut id_input = String::new();
                let _ = std::io::stdin().read_line(&mut id_input).unwrap();
                todos.remove((from_string_to_i32(id_input)-1) as usize);
            }
            "3"=>{
                for todo in &todos{
                    println!("id: {} Title: {} completed:{}",todo.id , todo.title , todo.completed);
                }
            }
            "4"=>{
                println!("Enter todo to edit complited state (id)");
                let mut id_input = String::new();
                let _ = std::io::stdin().read_line(&mut id_input).unwrap();
                println!("Enter todo state on completed (false/true)");
                let mut completed_input = String::new();
                let _ = std::io::stdin().read_line(&mut completed_input).unwrap();
                todos[(from_string_to_i32(id_input)-1) as usize].completed = from_string_to_bool(completed_input);
            }
            "5"=>{
                println!("Have a good day!");
                break;
            }
            _ => {
                println!("1 to add , 2 to delete, 3 to list, 4 to exit");
            }
        }
    }
}
