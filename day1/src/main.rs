use std::io::stdin;

fn fuel_required(mass:u32) -> u32
{
    let fuel = (mass/3).checked_sub(2).unwrap_or(0);
    return fuel;
}

fn read_mass_input(mass_num:usize) -> u32
{
    let index = mass_num + 1;
    println!("enter mass #{}: ", index);
    let mut s = String::new();
    stdin().read_line(&mut s).expect("did not enter correct string");
    
    let trimmed_string = s.trim();
    let mut mass :u32 = 0;

    match trimmed_string.parse::<u32>()
    {
        Ok(i) => mass = i,
        Err(..) => println!("this is not an integer: {}", trimmed_string),
    };
    
    return mass;
}

fn main() {

    println!("enter number of bodies: ");
    let mut s = String::new();
    stdin().read_line(&mut s).expect("did not enter correct string");

    let trimmed_string = s.trim();
    let mut number_of_masses :usize = 0;

    match trimmed_string.parse::<usize>()
    {
        Ok(i) => number_of_masses = i,
        Err(..) => println!("this is not an integer: {}", trimmed_string),
    };

    let mut mass_list: Vec<u32> = Vec::new();
    
    for i in 0..number_of_masses
    {
        let mass :u32 = read_mass_input(i);
        mass_list.push(mass);
    }

    let mut cumulative_fuel = 0;

    for mass in mass_list.iter()
    {
        cumulative_fuel += fuel_required(*mass);
    }
    println!("necessary fuel: {}", cumulative_fuel);
}
