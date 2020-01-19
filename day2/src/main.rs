fn opcode_process(arr: &mut [u32], index: usize)
{
    let operator_1_location = arr[index + 1];
    let operator_1 = arr[operator_1_location as usize];
    let operator_2_location = arr[index + 2];
    let operator_2 = arr[operator_2_location as usize];
    let result_location = arr[index + 3];
    
    match arr[index]
    {
        1 => arr[result_location as usize] = operator_1 + operator_2,
        2 => arr[result_location as usize] = operator_1 * operator_2,
        _=>(),
    }
}

fn main() 
{
    let mut opcodes: [u32; 12] = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    println!("Opcodes {:?}", opcodes);

    for i in 0..opcodes.len()
    {
        match opcodes[i]
        {
            1 | 2 => opcode_process(&mut opcodes, i),
            99 => break,
            _=>(),
        }
    }
    println!("Done!  Opcode result {:?}", opcodes);
}
