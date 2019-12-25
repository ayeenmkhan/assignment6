#[derive(Debug)]
struct Employee {
    name:String,
    salary:i32,
    age:i32,
}
fn main() {
    let first_user = Employee {
        name: String::from("Ayeen Khan"),
        salary: 80000,
        age: 27,
     };
     println!("********Printing first user Instance********");
     println!("First User information is :: {:#?}",first_user);

     println!("********Printing user Instance By element********");

     println!("User name is {}",first_user.name);
     println!("User salary is {}",first_user.salary);
     println!("User age is {}",first_user.age);

     
    let second_user= Employee{
        name:String::from("Imran Ali"),
        age:first_user.age,
        salary:first_user.salary,
    };

    println!("********Printing second user Instance********");
    println!("Second User information is :: {:#?}",second_user);

    let new_salary=90000;
    let new_age=25;
    
    let third_user=salary(new_salary,new_age);

    println!("********Printing user defined function Instance********");
    println!("Third user information is  :: {:#?}",third_user);
}


fn salary(salary:i32,age:i32)->Employee{

    let third_user=Employee{
        name:String::from("Usman Ali"),
        age:age,
        salary:salary
    };
    third_user

}
