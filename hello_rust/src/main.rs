// static NUMBER: i32 = 18;

// use std::thread::sleep;
// use std::time::Duration;

fn main() {

    

    // let mut i = 0;
    // let max = 5;

    // while i < max {
    //     println!("nilai: {i}");
    //     i += 1;

    //     sleep(Duration::from_secs(1));
    // }

    // let mut i = 0;
    // let max = 5;

    // while i < max {
    //     let mut j = 0;
    //     let max_inner = i;

    //     while j <= max_inner {
    //         print!("* ");
    //         j += 1;
    //     }

    //     println!();
    //     i += 1;
    // }

    // let max = 100.0;
    // let string_f = "nilai minimum kelulusan";
    // let result_f: f64 = if string_f == "nilai maksimum kelulusan" {
    //     max
    // } else {
    //     max * 3.0 / 4.0
    // };
    // println!("angka adalah {result_f}");

    // let number_d = 3;
    // let result_d = 
    //     if number_d == 2 {
    //         true
    //     } else {
    //         false
    //     };

    // println!("result_d adalah {result_d}")

    // let number_d = 3;
    // let result_d:bool;

    // if number_d ==2 {
    //     result_d = true
    // } else {
    //     result_d = false
    // }
    // println!("result_d adalah {result_d}")

    // let number_c = 3;
    // if number_c > 6 {
    //     println!("selamat, anda lulus dengan perolehan angka, {}", number_c);
        
    //     if number_c == 10 {
    //         println!("dengan nilai sempurna, {}",number_c );
    //     } else if number_c > 7 {
    //         println!("dengan nilai baik");
    //     } else {
    //         println!("dengan nilai cukup");
    //     }
    // } else {
    //     println!("anda tidak lulus");

    //     if number_c < 4 {
    //         println!("belajar lagi ya!");
    //     } else {
    //         println!("jangan malas belajar");
    //     }
    // }
    // let number_a = 3;
    // if number_a == 5 {
    //     println!("number_a adalah dibawah 5");
    // } else if number_a < 2 {
    //     println!("number_a adalah dibawah 2");
    // } else {
    //     println!("number_a adalah diatas 2")
    // }

    // let (bool_left, bool_right) = (true, false);
    // println!("AND result \t: {}", bool_left & bool_right);
    // println!("OR result \t: {}", bool_left || bool_right);

    // let (value_left, value_right) = (12, -12);
    // let res_one = -value_left == value_right;
    // let res_two = !(value_left == value_right);
    // println!("{res_one} {res_two}");

    // let res_one = 1 != 2;
    // println!("res_one: {res_one}");

    // let (num1, num2) = (12, 4);

    // let value_condition = num1 + num2;
    // println!("{} + {} = {}", num1, num2, value_condition);

    // println!("{}", NUMBER);
    // const LABEL: &str = "nilai konstanta";
    // const PI:f32 = 22.0/7.0;
    // println!("{}: {}", LABEL, PI);

    // let var5 = r#"
    //     {
    //         "nama": "baris satu",
    //         "alamat": "baris dua",
    //         "telepon": "baris tiga"
    //     }
    // "#;
    // println!("{}", var5);

    // let var6 = "
    //     {
    //         \"nama\": \"baris satu\",
    //         \"alamat\": \"baris dua\",
    //         \"telepon\": \"baris tiga\"
    //     }
    // ";
    // println!("{}", var6);
    
    // let var3 = "baris satu
    //     baris dua
    //     baris tiga";
    // println!("{}", var3);

    // let var1: &str = "hello";
    // println!("var1: {}", var1);

    // let ptr1: &i32 = &24;
    // println!("ptr1: {}", ptr1);

    // let c1 = 'n';
    // let c2 = '-';
    // let c3 = '2';
    // println!("{}, {}, {}", c1, c2, c3);

    // let min_fp1 = f32::MIN;
    // let max_fp1 = f32::MAX;
    // println!("f32 min: {}, f32 max: {}", min_fp1, max_fp1);

    // let fp1: f32 = 3.14;
    // let fp2: f64 = 3.141234512313133312231231;
    // println!("fp1: {}, fp2: {:.4}", fp1, fp2);

    // let min_i8 = i8::MIN;
    // let max_i8 = i8::MAX;
    // println!("i8 min: {}, i8 max: {}", min_i8, max_i8);

    // let numerik1: i32 =24;
    // let numerik2: i8 = 128;
    // let numerik3: i64 = 12;
    // println!("{}, {}, {}", numerik1, numerik2, numerik3);

    // let x = 5;
    // println!("The value of x is: {}", x);

    // let x = x + 1;
    // println!("The value of x is: {}", x);

    // let data1 = 24_i8;
    // println!("data1: {0}", data1);

    // let data1 = 24i8;
    // println!("data1: {0}", data1);

    // let (var5, mut var6, var7):(i8,i8,i8) = (23, 45, 67);
    // println!("var5: {0}", var5);
    // println!("var6: {0}", var6);
    // var6 = 24;
    // println!("var6: {0}", var6);
    // println!("var7: {0}", var7);

    // let (var3, var4):(i8,i8) = (23, 45);
    // println!("var3: {0}", var3);
    // println!("var4: {0}", var4);

    // let (var1, var2)= (23, "hello");
    // println!("var1: {0}", var1);
    // println!("var2: {0}", var2);

    // let message_number: i32;
    // message_number = 1;
    // println!("Message number {}", message_number);

    // let message_number = 1;
    // println!("Message number {}", message_number);

    // let mut message_number = 1;
    // let message1 = "hello";
    // println!("Message {}: {}", message_number, message1);

    // message_number = 2;
    // let message2  = "world";
    // println!("Message {}: {}", message_number, message2 );

    // message_number = 3;
    // let message3: i8 = 24;
    // println!("Message {1}: {0}", message3, message_number);
    
    // let nama_variable = "Hello, world!";
    // println!("{}",nama_variable);
    
    // println!("Hello, world!");
    // println!("How");
    // println!("are");
    // println!("you?");
}
