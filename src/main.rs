use std::collections::VecDeque;
use::std::io;
use::std::mem;
use rand::prelude::SliceRandom;
use std::{thread,time};

/*
CÓDIGO HECHO POR IVÁN ISRAEL HURTADO LOZANO
ESTE CÓDIGO HECHO EN RUST PERMITE AL USUARIO CREAR LISTAS DE PROCESOS Y RESOLVERLOS POR EL MÉTODO DE TABLA DE REEMPLAZO DE PÁGINAS FIFO O LRU
LA RESOLUCIÓN SE MUESTRA GRÁFICAMENTE EN CONSOLA MEDIANTE UNA TABLA SENCILLA
24/04/2024
*/

fn main() {
    
    let mut lista_procesos:Vec<i32>;

// -------------------- SECCIÓN INICIAL, CREACIÓN DE LA LISTA DE PROCESOS VIA USUARIO O ALEATORIA
    loop {

        let mut input = String::new();

        println!(".:MENU:.\n1. Lista de procesos creada aleatoriamente \n2. Lista de procesos creada por usuario\n3. Salir\nIngresa la opción deseada ---> ");
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        
        // asignando un valor entero a una variable, se puede cambiar el tipo
        let opc:i32 = match input.trim().parse(){
            Ok(n) => n,
            Err(_) => {
                println!("Ingresa un número válido");
                return;
            }
        };

        match opc {
            1 => {
                let mut input = String::new();
                println!("Ingresa el número de procesos ->");
                io::stdin().read_line(&mut input).expect("Error al leer la línea"); // Using the ? operator to propagate errors
                let procesos: i32 = input.trim().parse().expect("Ingresa un número");
                
                let mut input = String::new();
                println!("Ingresa el número de páginas ->");
                io::stdin().read_line(&mut input).expect("Error al leer la línea"); // Using the ? operator to propagate errors
                let paginas: i32 = input.trim().parse().expect("Ingresa un número");

                lista_procesos = crear_lista_procesos_random(procesos as usize, paginas as usize);
            },
            2 => {
                let mut input = String::new();
                println!("Ingresa el número de páginas ->");
                io::stdin().read_line(&mut input).expect("Error al leer la línea"); // Using the ? operator to propagate errors
                let paginas: i32 = input.trim().parse().expect("Ingresa un número");

                lista_procesos = crear_lista_procesos_input(paginas as usize);
            },
            3 => {
                println!("Cerrando programa :)");
                thread::sleep(time::Duration::from_secs_f32(1.5));
                break
            },
            _ => {
                println!("Opción inválida");
                return;
            }
        }

// -------------------- SEGUNDA SECCIÓN, OPERACIÓN DE LAS LISTAS DE PROCESOS, MEDIANTE EL MÉTODO FIFO O LRU
        loop {
            let mut input = String::new();
            println!(".:MENU:.\n1. Reemplazo de páginas con método FIFO\n2. Reemplazo de páginas con método LRU\n3. Salir\nIngresa el método de tabla de reemplazo de páginas deseado ---> ");
            io::stdin().read_line(&mut input).expect("Error al leer la entrada");
            
            // asignando un valor entero a una variable, se puede cambiar el tipo
            let opc:i32 = match input.trim().parse(){
                Ok(n) => n,
                Err(_) => {
                    println!("Ingresa un número válido");
                    return;
                }
            };
    
            match opc {
                1 => {
                    let mut input2 = String::new();
                    println!("Ingresa el número de marcos ---> ");
                    io::stdin().read_line(&mut input2).expect("Error al leer la entrada");
    
                    // asignando un valor entero a una variable, se puede cambiar el tipo
                    let marcos: i32 = match input2.trim().parse() {
                        Ok(n) => n,
                        Err(_) => {
                            println!("Ingresa un número válido");
                            return;
                        }
                    };
    
                    reemplazo_fifo(&lista_procesos, marcos);
                },
                2 => {
                    let mut input2 = String::new();
                    println!("Ingresa el número de marcos ---> ");
                    io::stdin().read_line(&mut input2).expect("Error al leer la entrada");
    
                    // asignando un valor entero a una variable, se puede cambiar el tipo
                    let marcos: i32 = match input2.trim().parse() {
                        Ok(n) => n,
                        Err(_) => {
                            println!("Ingresa un número válido");
                            return;
                        }
                    };
    
                    reemplazo_lru(&lista_procesos, marcos);
                },
                3 => {
                    println!("Terminando ejecución de la lista de procesos actual");
                    thread::sleep(time::Duration::from_secs_f32(1.5));
                    break
                },
                _ => {
                    println!("Opción inválida");
                    return;
                }
            }
            
        }
    }

}

// -------------------- FUNCIONES

fn reemplazo_fifo(lista_procesos:&Vec<i32>,marcos:i32) -> () {
    let mut cola:VecDeque<i32> = VecDeque::with_capacity(marcos as usize);
    
    let mut fallos:i32 = 0;
    let cols = lista_procesos.len();
    let mut registro_fallos:Vec<i8> = vec![0;cols as usize]; // arreglo inicial del tamaño cols lleno de solamente 0's
    let mut tabla_ind:Vec<i32> = Vec::new(); // tabla de una sola iteracion
    let mut tabla: Vec<Vec<i32>> = vec![vec![-1; cols as usize];marcos as usize]; // declaración de arreglo bidimensional, lleno de -1's, primero columnas y luego filas

    for i in 0..cols{
        if !cola.contains(&lista_procesos[i]){
            if cola.len()<marcos as usize{
                tabla_ind.push(lista_procesos[i]);
            }
            else {

                // sacando el valor más viejo de la cola
                let value = match cola.pop_front() {
                    Some(value) => value,
                    None => panic!("VecDeque is empty"), // Handle the case where VecDeque is empty (optional)
                };
                
                println!("{value}");
                
                let index = tabla_ind.iter().position(|&r| r == value).unwrap(); // buscando el índice donde está el número que del proceso que acabamos de sacar de la cola
                
                tabla_ind[index] = lista_procesos[i];
                // println!("{index}");
            }
            cola.push_back(lista_procesos[i]);
            registro_fallos[i] = 1;
            fallos+=1;
        }
        println!("\nIteración: {}",i+1);
        println!("Proceso: {}",lista_procesos[i]);
        println!("Cola: {:?}",cola);
        println!("Tabla ind: {:?}",tabla_ind);

        for j in 0..tabla_ind.len(){
            tabla[j][i] = tabla_ind[j];
        }
    }
    
    imprimir_resultados(tabla, lista_procesos, registro_fallos, fallos);
}

fn reemplazo_lru(lista_procesos:&Vec<i32>,marcos:i32) -> () {
    let mut cola:VecDeque<i32> = VecDeque::with_capacity(marcos as usize);
    
    let mut fallos:i32 = 0;
    let cols = lista_procesos.len();
    let mut registro_fallos:Vec<i8> = vec![0;cols as usize];
    let mut tabla_ind:Vec<i32> = Vec::new(); // tabla de una sola iteracion
    let mut tabla: Vec<Vec<i32>> = vec![vec![-1; cols as usize];marcos as usize];

    for i in 0..cols{
        if !cola.contains(&lista_procesos[i]){
            if cola.len()<marcos as usize{
                tabla_ind.push(lista_procesos[i]);
            }
            else {
                // sacando el valor más menos usado de la cola
                let value = match cola.pop_front() {
                    Some(value) => value,
                    None => panic!("VecDeque is empty"), // Handle the case where VecDeque is empty (optional)
                };
                
                println!("{value}");
                
                let index = tabla_ind.iter().position(|&r| r == value).unwrap(); // buscando el índice donde está el número que del proceso que acabamos de sacar de la cola
                
                tabla_ind[index] = lista_procesos[i];
                // println!("{index}");
            }
            cola.push_back(lista_procesos[i]);
            registro_fallos[i] = 1;
            fallos+=1;
        }
        else {
            //recorrer el orden del proceso en la cola
            let index = cola.iter().position(|&r| r == lista_procesos[i]).unwrap(); // buscando el índice en la cola donde está el proceso actual, que ya se encuentra dentro de la cola
            // se hace para recorrerlo

            for i in index..cola.len(){
                if i<cola.len()-1{
                    let mut x = cola[i];
                    let mut y = cola[i+1];
                    mem::swap(&mut x, &mut y);
                    cola[i] = x;
                    cola[i+1] = y;
                }
            }
        }
        println!("\nIteración: {}",i+1);
        println!("Proceso: {}",lista_procesos[i]);
        println!("Cola: {:?}",cola);
        println!("Tabla ind: {:?}",tabla_ind);

        for j in 0..tabla_ind.len(){
            tabla[j][i] = tabla_ind[j];
        }
    }
    
    imprimir_resultados(tabla, lista_procesos, registro_fallos, fallos)
    
}

fn crear_lista_procesos_random(procesos: usize, paginas: usize) -> Vec<i32>{
    let mut rng = rand::thread_rng();
    let mut correcto:bool = false;
    let lista_procesos: Vec<usize> = (1..=procesos).collect();
    let mut lista_final:Vec<i32> = Vec::new();

    // println!("{:?}",lista_procesos);

    let mut lista_prueba:Vec<i32> = Vec::new();

    for _ in 0..paginas/2{
        for i in 0..lista_procesos.len(){
            lista_prueba.push(lista_procesos[i].try_into().unwrap());
        }
    }

    // println!("{:?}",lista_prueba);

    //revisamos que cada proceso se encuentre al menos una vez en el subgrupo de la lista de procesos, sino, se volveran a mezclar los valores del vector (shuffle)
    while correcto==false {
        correcto=true;
        lista_prueba.shuffle(&mut rng);
        // println!("{:?}",lista_prueba);
        
        let subgrupo: Vec<i32> = lista_prueba[0..12].to_vec();
        // println!("{:?}",subgrupo);
        
        for i in 1..=procesos{
            if subgrupo.iter().any(|&j| j == i.try_into().unwrap()){
            }
            else {
                correcto = false;
            }
        }
        lista_final = subgrupo.clone();
    }
    // println!("correcto");
    
    println!("Lista a utilizar: {:?}",lista_final);
    lista_final
}

fn crear_lista_procesos_input(paginas: usize) -> Vec<i32> {
    let mut lista_procesos: Vec<i32> = Vec::new();
    for i in 0..paginas {
        let mut input = String::new();
        println!("Ingresa un proceso del índice {i} ->");
        io::stdin().read_line(&mut input).expect("Error al leer la línea"); // Using the ? operator to propagate errors
        let val: i32 = input.trim().parse().expect("Ingresa un número");
        lista_procesos.push(val);
    }
    println!("Lista a utilizar: {:?}",lista_procesos);
    lista_procesos
}

fn imprimir_resultados(tabla:Vec<Vec<i32>>, lista_procesos:&Vec<i32>, registro_fallos:Vec<i8>, fallos:i32) -> () {
    println!("\nTabla resultante:\n");
    for i in lista_procesos{
        print!("{i} ")
    }
    println!();
    for i in 0..tabla.len(){
        for j in 0..tabla[i].len(){
            if tabla[i][j] == -1{
                print!("  ")
            }
            else {
                print!("{} ",tabla[i][j])
            }
        }
        println!("");
    }

    for i in registro_fallos{
        if i == 1{
            print!("F ")
        }
        else {
            print!("  ")
        }
    }
    // println!("Registro fallos\n {:?}", registro_fallos);

    // println!("{}",tabla_ind.len());

    println!("\n\nFallos totales {fallos}\n");
}
