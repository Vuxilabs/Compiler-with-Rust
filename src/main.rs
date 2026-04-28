use std::io;

fn main() {
    // let nama = "davingm";
    // let umur = 16;
    // let tipe = "Rust";

    // println!("Halo Nama saya {} saat ini sedang belajar {} ", nama, tipe);

    let mut todos: Vec<String> = Vec::new();

    loop {
        println!("\n ==== todo aplikasi sederhana dengan rust");
        println!("1. Tambah todo");
        println!("2. Lihat todo");
        println!("3. Keluar");

        let mut pilihan = String::new();
        io::stdin().read_line(&mut pilihan).expect("Gagal membaca inputan");

        match pilihan.trim(){
            "1" => {
                println!("masukan todo : ");
                let mut todo = String::new();
                io::stdin().read_line(&mut todo).expect("Error");

                todos.push(todo.trim().to_string());
                println!("tambahkan todo");
            }

            "2" => {
                println!("Daftar todo :");
                for (i, item) in todos.iter().enumerate(){
                    println!("{}, {}", i + 1, item);
                }
            }

            "3" => {
                println!("keluar!");
                break;
            }

            _ => {
                println!("Pilihan tidak valid!");
            }
        }
    }

}
