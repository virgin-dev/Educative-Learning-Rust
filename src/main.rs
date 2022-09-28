fn main() {
nested_for();
}



fn nested_for() {
    for i in 1..5 {
        println!("Multiplication Table of : {}", i);
        for j in 1..5 {
            println!("{} * {} = {}", i, j, i * j);
        }
    }
}
//Цикл while вместе с continue
fn while_loop() {
    let mut var = 1;
    loop {
        var = var + 1;
        println!("{}",var);
        if var == 4 {
            println!("I encoutered continue statement");
            continue;
        }
        if var == 1000000 {
            println!("Stop IT!");
            break;
        }
        println!("I did not encounter continue statement");
    }
}
fn while_continue() {
    let mut var = 1;
    let mut found = false;

    while !found {
        var = var + 1;
        println!("{}",var);
        if var == 4 {
            println!("I encoutered a continue statement");
        }
        println!("I did not encoutered continue statement");

        if var == 10 {
            found = true;
        }
    }
}

// Использлвание continue вместе с циелом for
fn testing() {
    for var in 0..5 {
        if var == 2 {
            println!("Continue statement");
            continue;
        }
        println!("var: {}", var);
    }
}
//Цикл while
fn test () {
    let mut var = 1;
    let mut found = false;
    while !found {
        var=var+1;
        println!("{}",var);
        if var % 2 == 1 {
            found = true;
        }
        println!("Loop runs");
    }
}
//Цикл бесконечности
fn infinity() {
    let mut var = 1;
    loop {
        var = var + 1;
        println!("{}", var);
    }
}
//Побитовые операнды
//Example:
/*fn bytes_operand() {
    let a = 5;
    let b = 6;
    println!("Operand 1: {}, Operand 2: {}", a , b);
    println!("AND: {}", a & b);
    println!("OR: {}", a | b);
    println!("XOR: {}", a ^ b);
    println!("NOT a: {}", !a);
    println!("Left shift: {}", a << 2);
    println!("Right shift: {}", a >> 1);
}*/

/*
// Пример работы операндов
fn operands() {
    let a = true;
    let b = false;
    println!("Operand 1:{}, Operand 2:{}", a , b);
    println!("AND:{}", a && b);
    println!("OR:{}", a || b);
    println!("NOT:{}", ! a);
}*/

//Константы  - const
/*
fn consts() {
    //Константы - посстоянные не изменяемые переменные, синтаксис - const id:i32=001; В соглашении писать следует в капсе;
    const ID_1: i32 = 4;
    println!("ID:{}", ID_1);
}*/

//Кортежи - Tuples
/*
fn tuples() {
    //Пример создания кортежа -> let tuple_name = ("Value1",'c',1);
    //Кортеж с указанием типа -> let tuple_name : (&str,char,i32) = ("Value1",'c',1);
    //Example ->
    let person_data = ("Alex",48,"35kg","6ft"); //Создание кортежа без указания типа
    let persons_data : (&str,i32,&str,&str) = ("Alex",48,"35kg","6ft"); //С указанием типов
    //Доступк к элементу кортежа производится через . прим. -> tuplename.indexvalue
    //Чтобы получить отдельные значения из кортежа, мы можем использовать сопоставление с образцом, чтобы деструктурировать значение кортежа, например:
    let person_data = ("Alex",48,"35kg", "6ft");
    let (w,x,y,z) = person_data;
    println!("Name : {}", w);
    println!("Age : {}", x);
    println!("Weight : {}", y);
    println!("Height : {}", z);

    //Также мы можем изменять значение картежа, наравне с переменной можем добавить mut после слова let
    let mut mutable_data = ("Alex",48,"35kg", "6ft");
    println!("The value of the tuple at index 0 and index 1 are {} {}", mutable_data.0, mutable_data.1);
    mutable_data.0 = "John";
    println!("The value of the tuple at index 0 and index 1 are {} {}", mutable_data.0, mutable_data.1);
}
*/

//Массивы и Слайсы
/*
fn array_slices() {
    let arr:[i32;4] = [1,2,3,4];
    let arr1 = [2;4];
    println!("{:?}",arr1);
    println!("{}",arr.len());
    //Создаем срез массивов
    let slice_array:&[i32] = &arr;
    println!("Срез из полного массива:{:?}",slice_array);
    let slice_array1:&[i32] = &arr1[0..3];
    println!("Срез со значения 0 до 3:{:?}",slice_array1);
}*/