/*
            Una necesidad muy común en cualquier lenguaje de programación es la manipulación de archivos. Es decir, tener la posibilidad de capturar y leer el contenido de archivos de configuración u otro tipo de información almacenado en un JSON, CSV, TXT, etc.

            Lectura de archivos en Rust
            La lectura de un archivo en Rust es una tarea muy simple. Basta con importar fs desde std. Recordemos que std permite acceder al sistema operativo y realizar diversas funcionalidades.

            use std::{fs};
            const FILENAME: &str = "my_file.csv";

            fn main() {
                let content: String = fs::read_to_string(FILENAME).unwrap();
                println!("{}", content);
            }
            Con fs::read_to_string podemos leer un archivo en la raíz del proyecto y siempre es necesario la utilización de unwrap() para capturar errores, ya que el archivo podría no existir.

            Lectura de archivos .csv
            Visualicemos el contenido de un archivo .csv:

            Colum1;Colum2;Colum3
            Bienvenido;a;Platzi
            Podemos capturar ese texto en una simple línea de código, pero hasta aquí tendríamos solo un simple String que debemos convertir a, en este caso, un CSV para capturar cada valor delimitado por punto y coma.

            Para esto, importamos la dependencia csv en el archivo Cargo.toml:

            [dependencies]
            csv = "1.1.6"
            La importamos y utilizamos de la siguiente manera:

            use csv::{ReaderBuilder};
            use std::{fs};
            const FILENAME: &str = "my_file.csv";

            fn main() {
                let content: String = fs::read_to_string(FILENAME).unwrap();
                let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());
            }
            ReaderBuilder permite leer el contenido capturado previamente con fs, indicando el tipo de delimitador y el tipo de dato como binario. Convertimos el String en un Vector que contiene el contenido de nuestro archivo.

            Solo resta recorrer el contenido de nuestro archivo con un ciclo for de la siguiente forma:

            // ...
            fn main() {
                let content: String = fs::read_to_string(FILENAME).unwrap();
                let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(content.as_bytes());
                for result in rdr.records() {
                    println!("{}", result.unwrap().get(0).unwrap().trim());
                }
            }
            Devuelve el vector de registros con records() y accede a la columna del CSV que necesites con get(1) en cada iteración. Siempre asegúrate de utilizar unwrap() para contener los errores.

            Es muy frecuente necesitar convertir datos leídos desde un archivo en un JSON o en un Excel. Explora la manipulación de diferentes tipos de archivos en Rust para estar preparado para cualquier necesidad.


            ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            Las aplicaciones que desarrolles en Rust comenzarán a tener necesidades avanzadas y vas a requerir más herramientas para resolver diversos escenarios. Veamos varias utilidades de Rust para resolver problemas comunes a la hora de programar.

        Valores por defecto en los errores
        Para evitar que tu aplicación se detenga por un error y que continúe ejecutándose, puedes colocar un valor por defecto utilizando unwrap_or().

        fn main() {
            println!("Ingrese su edad: ");
            let mut edad: String = String::new();
            std::io::stdin().read_line(&mut edad).unwrap();
            let edad_int: u8 = edad.trim().parse().unwrap_or(18);

            println!("Tienes {} años: ", edad_int);	
        }
        Si el usuario ingresa una letra en lugar de un número cuando se le solicita su edad, la conversión del tipo de dato fallará, pero el unwrap_or() devolverá un valor establecido por defecto y tu aplicación continuará operando.

        Estructuras de datos
        Crear estructuras de datos para almacenar, dentro de una misma variable, atributos pertenecientes a una misma cosa. Puedes crear estructuras que tendrán la forma de tus datos de la siguiente manera:

        struct Person {
            nombre: String,
            apellido: String,
            edad: i32,
            pais: String,
        }

        fn main() {
            let persona = Person {
                nombre: "Kevin".to_string(),
                apellido: "Fiorentino".to_string(),
                edad: 27,
                pais: "Argentina".to_string(),
            };

            println!("{}", persona.nombre);
            println!("{}", persona.apellido);
            println!("{}", persona.edad);
            println!("{}", persona.pais);
        }
        Con la palabra clave struct, declaras las propiedades de tu estructura y puedes crear una variable que almacene estos datos y acceder a ellos mediante un punto “.”.

        NOTA: Las estructuras utilizan CamelCase para nombrar a las mismas a diferencia de las variables o funciones que utilizan snack_case.

        

        Implementaciones de estructuras
        Una estructura puede extenderse e implementar funciones para realizar determinada lógica, como crear un nuevo objeto de ese tipo y realizar algún cálculo.

        struct Person {
            nombre: String,
            apellido: String,
            edad: i32,
            pais: String,
        }

        impl Person {
            fn new_person(nombre: String, apellido: String, edad: i32, pais: String) -> Person {
                return Person { nombre, apellido, edad, pais };
            }
        }

        fn main() {
            let persona: Person = Person::new_person("Kevin".to_string(), "Fiorentino".to_string(), 27, "Argentina".to_string());

            println!("{}", persona.nombre);
        }
        Utilizando la palabra reservada impl, la función new_person creará un objeto Person pasándole como parámetro los datos que necesita y devolviendo el mismo para su posterior utilización. Has el llamado a estas funciones implementadas en una estructura con Person::XXXXX.

        Almacenamiento clave/valor
        Otra manera de almacenar información, además de las estructuras y los vectores, son los denominados HashMap. Los mismos son “diccionarios” de datos, del tipo clave/valor, donde para acceder a un elemento, en lugar de utilizar el índice del mismo como en un vector, se utiliza la Clave, que puede ser un string o un número, para colocarle un nombre al Valor y obtenerlo.

        use std::collections::{HashMap};

        fn main() {
            let mut diccionario: HashMap<&str, &str> = HashMap::new();

            diccionario.insert("Manzana", "La manzana es roja.");
            diccionario.insert("Banana", "La banana es amarilla.");
            diccionario.insert("Naranja", "La naranja es... naranja.");

            println!("{}", diccionario["Manzana"]);			// La manzana es roja.
        }
        De esta forma, puedes guardar un nuevo valor con insert() y acceder al mismo a través de su clave.

        Explora estas nuevas estructuras y tipos de datos para resolver diversas situaciones donde se vuelve algo más complicado manipular la información y mantener la claridad y prolijidad en tu código.
        
            */


use std::{fs, collections::HashMap, io}; //file system
use csv::{ReaderBuilder,StringRecord};


//Mi archivo
const FILENAME:&str = "history.csv";
const ONETAG:&str = "LUZ";

//TIPO ,TAG ,TEXTO ,VIDA
#[derive(Debug)]
struct DatesHistory{
    tipo:String,
    tag:String,
    texto:String,
    vida:i32,
    options:Vec<DatesHistory>,
}

/*
1. Se usa impl en su declaracion.
2. Su nombre es igual al tipo de dato al que le quieras a agregar funcionabilidades.
3. Se declaran todas las funciones dentro
4. Para llamar una funcion de esta implementacion queda algo como **TipoDeDato::**_funcion(parametros)_
Es como la programación orientada a objetos de rust, las funciones impl, son como los métodos de una clase
*/
impl DatesHistory {
    fn new(row: StringRecord) -> DatesHistory {
        let vida =  row.get(3).unwrap().trim();
        let vida:i32 =  vida.parse().unwrap_or(0);
        return DatesHistory {
            tipo:row.get(0).unwrap().trim().to_string(),
            tag:row.get(1).unwrap().trim().to_string(),
            texto:row.get(2).unwrap().trim().to_string(),
            vida,
            options:vec![]
        };
    }
}

fn read_to_csv(csv:&str){
    let content = fs::read_to_string(csv).unwrap();
    //print!("{}",content);
    delimiter_csv(content.as_str());
}

fn delimiter_csv(csv:&str){
    let mut heal = 100;
    let mut tag_actual = ONETAG;

    let mut last_record = "".to_string();

    let mut dates_history:HashMap<String,DatesHistory> = HashMap::new();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(csv.as_bytes());

    for result in rdr.records(){
        let result = result.unwrap();
        let date = DatesHistory::new(result);

        if date.tipo == "SITUACION" {
            let record_tag = date.tag.clone(); 
            dates_history.insert(record_tag.clone(), date);
            last_record = record_tag;
        }else if date.tipo == "OPCION" { 
            if let Some(dates) = dates_history.get_mut(&last_record){
                (*dates).options.push(date); 
            }
        }   
    }       
    
    loop{
        println!("Vida player: {:?}",heal);

    if let Some(dates) = dates_history.get(tag_actual){
        println!("{}", dates.texto);

            for (index,option) in dates.options.iter().enumerate() {
                println!("[{}],{}",index,option.texto);
            }

            let mut option_index = String::new();
            io::stdin().read_line(&mut option_index).unwrap();
            let option_index = option_index.trim().parse::<usize>().unwrap_or(99);

            if let Some(option_aplit) = &dates.options.get(option_index){
                    tag_actual = &option_aplit.tag;
            }else{
                println!("Comando no valido.");
            }
            heal += dates.vida;
            println!("");
        }else{
            break;
        }
        if heal <= 0{
            println!("YOU DEAD");
            break;
        }
    }
    println!("{:?}",dates_history["DERECHA"]);
}


fn main() {
    read_to_csv(FILENAME);
}
