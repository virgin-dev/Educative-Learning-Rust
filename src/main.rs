fn main() {


}






//Кортеже - Tuples
fn tuples() {
    //Пример создания кортежа -> let tuple_name = ("Value1",'c',1);
    //Кортеж с указанием типа -> let tuple_name : (&str,char,i32) = ("Value1",'c',1);
    //Example ->
    let person_data = ("Alex",48,"35kg","6ft"); //Создание кортежа без указания типа
    let persons_data : (&str,i32,&str,&str) = ("Alex",48,"35kg","6ft"); //С указанием типов
    //Доступк к элементу кортежа производится через . прим. -> tuplename.indexvalue
}
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