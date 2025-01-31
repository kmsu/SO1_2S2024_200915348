use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::Local;
use serde_json::json;
//use reqwest::Client;
use reqwest::blocking::Client;
//use std::error::Error;
//use tokio;

// CREACIÓN DE STRUCT

/* 
    El #[derive (macro...)] es una característica de Rust que permite a los desarrolladores
    agregar funcionalidades a sus estructuras de datos. En este caso, estamos agregando
    la capacidad de serializar y deserializar la estructura de datos a JSON que es parte de la librería
    serde.
*/

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    #[serde(rename = "Processes")]
    processes: Vec<Process>
}

/* 
    Además de esto, estamos implementando los traits Eq, Ord y PartialOrd para poder comparar
    los procesos en base a su uso de CPU y memoria.

    La estructura de datos representa un proceso en el sistema operativo, con los siguientes campos:
    - pid: El identificador del proceso.
    - name: El nombre del proceso.
    - cmd_line: La línea de comandos que se utilizó para ejecutar el proceso.
    - memory_usage: La cantidad de memoria que está utilizando el proceso.
    - cpu_usage: El porcentaje de uso de CPU que está utilizando el proceso.

    Serde nos deja implementar macros a acada campo de la estructura de datos para poder renombrar
    los campos en el JSON que se genere.
*/
#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Process {
    #[serde(rename = "PID")]
    pid: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Cmdline")]
    cmd_line: String,
    #[serde(rename = "MemoryUsage")]
    memory_usage: f64,
    #[serde(rename = "CPUUsage")]
    cpu_usage: f64,
    #[serde(rename = "Vsz")]
    vsz: u64,
    #[serde(rename = "Rss")]
    rss: u64,
}

#[derive(Debug, Serialize, Clone)]
struct LogProcess {
    pid: u32,
    container_id: String,
    name: String,
    vsz: u64,
    rss: u64,
    memory_usage: f64,
    cpu_usage: f64,
    action: String,
    timestamp: String
}

// IMPLEMENTACIÓN DE MÉTODOS

/* 
    Función para sobreescribir el campo cmd_line de cada proceso por el id del contenedor.
*/
impl Process {
    fn get_container_id(&self) -> &str {
        let parts: Vec<&str> = self.cmd_line.split_whitespace().collect();
        for (i, part) in parts.iter().enumerate() {
            if *part == "-id" {
                if let Some(id) = parts.get(i + 1) {
                    return id;
                }
            }
        }
        "N/A"
    }
}

// IMPLEMENTACIÓN DE TRAITS

/* 
    Contamos con 2 ordenamientos, el Ord y el PartialOrd. El primero es para poder comparar
    los procesos en base a su uso de CPU y memoria, mientras que el segundo es para poder
    comparar los procesos en base a su uso de CPU y memoria de manera parcial.

    ¿Por qué de manera parcial si todos los valores existen? 
        - Porque en el caso de que haya un valor NaN, la comparación no se puede hacer de manera total.
        - Por ejemplo, si un proceso tiene un uso de memoria de 10 y otro de NaN, no se puede comparar
          de manera total, pero sí de manera parcial.
        - Al manejar números decimales pueden existir valores NaN, por lo que es importante manejarlos.
*/

/* 
    Este trait no lleva ninguna implementación, pero es necesario para poder comparar ya que debe satisfacer
    la propiedad de reflexividad, es decir, que un proceso es igual a sí mismo.
*/
impl Eq for Process {}  


/* 
    Ord Trait:
    Funcionalidad: Proporciona una comparación total para dos instancias de Process. 
    Devuelve un std::cmp::Ordering que puede ser Less, Greater o Equal.
    Ejecución: Si partial_cmp devuelve Some(Ordering), sort usará el resultado de cmp para ordenar los elementos. 
    La implementación de cmp en Process compara primero el uso de CPU y, si es igual, compara el uso de memoria.
    
    ¿Qué significa esto?
        - Permite comparar procesos basándose en su uso de CPU y memoria.
        - Si el uso de CPU de un proceso es mayor que el de otro, el proceso con mayor uso de CPU es considerado mayor.
        - Si el uso de CPU de ambos procesos es igual, se comparan en base a su uso de memoria.
        - Si tanto el uso de CPU como el de memoria son iguales, los procesos se consideran iguales.

    Detalles de implementación:
        - Se utiliza unwrap_or para devolver std::cmp::Ordering::Equal en caso de que haya un valor NaN.
        - El método then_with se usa para comparar en base a la memoria si el uso de CPU es igual.
        - Los || no son necesarios aquí ya que unwrap_or maneja los valores NaN.

    Se pueden agregar más condiciones para comparar en base a otros campos si es necesario.
*/
impl Ord for Process {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cpu_usage.partial_cmp(&other.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| self.memory_usage.partial_cmp(&other.memory_usage).unwrap_or(std::cmp::Ordering::Equal))
            .then_with(|| self.vsz.partial_cmp(&other.vsz).unwrap_or(std::cmp::Ordering::Equal))
            .then_with(|| self.rss.partial_cmp(&other.rss).unwrap_or(std::cmp::Ordering::Equal))
    }
}

/* 
    PartialOrd Trait:

    Funcionalidad: Permite la comparación parcial de dos instancias de Process. Devuelve un Option<std::cmp::Ordering>, 
    que puede ser Some(Ordering) si la comparación es válida o None si no lo es (por ejemplo, si hay un valor NaN).
    Ejecución: La función sort primero intentará usar partial_cmp para comparar los elementos. Si partial_cmp devuelve None, la comparación falla.
    
    ¿Qué significa esto?
        - La comparación puede fallar si hay un valor NaN.
        - Por ejemplo, si un proceso tiene un uso de memoria de 10 y otro tiene NaN, la comparación fallará.

    Detalles de implementación:
        - Se delega la comparación al método cmp del trait Ord, envolviendo el resultado en Some.
*/
impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


// FUNCIONES

/* 
    Función para matar un contenedor de Docker.
    - id: El identificador del contenedor que se quiere matar.
    - Regresa un std::process::Output que contiene la salida del comando que se ejecutó.
*/
fn kill_container(id: &str) -> std::process::Output {
    let  output = std::process::Command::new("sudo")
        .arg("docker")
        .arg("stop")
        .arg(id)
        .output()
        .expect("failed to execute process");

    println!("Matando contenedor con id: {}", id);

    output
}

/* 
    Funcion para obtener el nombre del contenedor y verificar que no sea el de logs
*/
fn get_container_name(container_id: &str) -> String {
    let output = std::process::Command::new("sudo")
        .arg("docker")
        .arg("inspect")
        .arg("--format={{.Name}}")
        .arg(container_id)
        .output()
        .expect("failed to execute process");

    let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    name
}

fn analyzer( system_info:  SystemInfo) {


    // Creamos un vector vacío para guardar los logs de los procesos.
    let mut log_proc_list: Vec<LogProcess> = Vec::new();


    /* 
        Creamos un vector vacío para guardar los logs del sistema.
        En este caso, no se guardará nada, pero se puede modificar para guardar
        información del sistema.
    */
    let mut processes_list: Vec<Process> = system_info.processes;

    //Elimino el proceso del contenedor de logs antes de ordenar -> |process| es el nombre con el que itera la lista
    if let Some(pos) = processes_list.iter().position(|process| {
        let cont = get_container_name(&process.get_container_id());
        cont == "/log_container"
    }) {
        processes_list.remove(pos);
    }     

    /* 
        Cuando llamas a la función sort en un vector de Process, se ejecutarán los traits 
        Ord y PartialOrd en el siguiente orden y con la siguiente funcionalidad:


        La función sort del vector llama internamente a partial_cmp para comparar los elementos.
        partial_cmp delega la comparación a cmp del trait Ord.


        Comparación con cmp:

        cmp compara primero el uso de CPU (cpu_usage).
        Si el uso de CPU es igual, compara el uso de memoria (memory_usage).
        Si ambos son iguales, devuelve Ordering::Equal.
        Funcionalidad de los Traits
        PartialOrd: Permite la comparación parcial, necesaria para manejar casos donde los valores pueden ser NaN.
        Ord: Proporciona una comparación total, necesaria para ordenar completamente los elementos del vector.

        Cuando llamas a processes_list.sort(), el método sort usará partial_cmp y cmp para comparar y 
        ordenar los procesos en el vector processes_list basándose en el uso de CPU y memoria.
    */
    processes_list.sort();


    // Dividimos la lista de procesos en una lista bajo consumo y una lista alto consumo
    let mut index = 0;
    for (i, process) in processes_list.iter().enumerate() {
        if process.cpu_usage > 0.1 {
            index = i;
            break;
        }
    }
    let (lowest_list, highest_list) = processes_list.split_at(index);


    // Hacemos un print de los contenedores de bajo consumo en las listas.
    println!("Bajo consumo");
    for process in lowest_list {
        //println!("PID: {}, Name: {}, container ID: {}, Memory Usage: {}, CPU Usage: {}", process.pid, process.name, process.get_container_id(), process.memory_usage, process.cpu_usage);
        println!("PID: {}, Name: {}, container ID: {}, Memory Usage: {}, CPU Usage: {}, vsz: {}, rss: {}", process.pid, process.name, process.get_container_id(), process.memory_usage, process.cpu_usage, process.vsz, process.rss);
    }

    println!("------------------------------");

    println!("Alto consumo");
    for process in highest_list {
        //println!("PID: {}, Name: {}, Icontainer ID {}, Memory Usage: {}, CPU Usage: {}", process.pid, process.name,process.get_container_id(),process.memory_usage, process.cpu_usage);
        println!("PID: {}, Name: {}, container ID: {}, Memory Usage: {}, CPU Usage: {}, vsz: {}, rss: {}", process.pid, process.name, process.get_container_id(), process.memory_usage, process.cpu_usage, process.vsz, process.rss);
    }

    println!("------------------------------");

    /* 
        En la lista de bajo consumo, matamos todos los contenedores excepto los 3 primeros.
        antes 
        | 1 | 2 | 3 | 4 | 5 |

        después
        | 1 | 2 | 3 |
    */

    

    if lowest_list.len() > 3 {
        // Iteramos sobre los procesos en la lista de bajo consumo.
        for process in lowest_list.iter().skip(3) {
            let now = Local::now();
            let log_process = LogProcess {
                pid: process.pid,
                container_id: process.get_container_id().to_string(),
                name: process.name.clone(),
                vsz: process.vsz,
                rss: process.rss,
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage,
                action: "kill".to_string(),
                timestamp: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            };

            log_proc_list.push(log_process.clone());

            // Matamos el contenedor.
            let _output = kill_container(&process.get_container_id());

            // Enviar el JSON al servidor
            if let Err(e) = send_log_to_server(&log_process) {
                eprintln!("Error al enviar el log: {:?}", e);
            }
        }
    } 

    /* 
        En la lista de alto consumo, matamos todos los contenedores excepto los 2 últimos.
        antes 
        | 1 | 2 | 3 | 4 | 5 |

        después
                    | 4 | 5 |
    */
    if highest_list.len() > 2 {
        // Iteramos sobre los procesos en la lista de alto consumo.
        for process in highest_list.iter().take(highest_list.len() - 2) {
            let now = Local::now();
            let log_process = LogProcess {
                pid: process.pid,
                container_id: process.get_container_id().to_string(),
                name: process.name.clone(),
                vsz: process.vsz,
                rss: process.rss,
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage,
                action: "kill".to_string(),
                timestamp: now.format("%Y-%m-%d %H:%M:%S").to_string(),
            };

            log_proc_list.push(log_process.clone());

            // Matamos el contenedor.
            let _output = kill_container(&process.get_container_id());

            // Enviar el JSON al servidor
            if let Err(e) = send_log_to_server(&log_process) {
                eprintln!("Error al enviar el log: {:?}", e);
            }
        }
    }

    // Hacemos un print de los contenedores que matamos.
    println!("Contenedores matados");
    for process in log_proc_list {
        //println!("PID: {}, Name: {}, Container ID: {}, Memory Usage: {}, CPU Usage: {} ", process.pid, process.name, process.container_id,  process.memory_usage, process.cpu_usage);
        println!("PID: {}, Name: {}, container ID: {}, Memory Usage: {}, CPU Usage: {}, vsz: {}, rss: {}", process.pid, process.name, process.container_id, process.memory_usage, process.cpu_usage, process.vsz, process.rss);
        
    }

    println!("------------------------------");
}

/* 
    Funcion para enviar los datos del contenedor eliminado al log en formato json
*/

//async fn send_log_to_server(log_process: LogProcess) -> Result<(), Box<dyn std::error::Error>> {
fn send_log_to_server(log_process: &LogProcess) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "http://127.0.0.1:8000/logs"; // Cambia esto a la URL de tu servidor

    // Crear un vector que contenga el log_process
    let log_list = vec![log_process.clone()];

    // Convertir el vector a una cadena JSON
    let json_body = serde_json::to_string(&log_list)?;

    // Enviar la solicitud POST
    let response = client.post(url)
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()?;

    

    if response.status().is_success() {
        println!("Log enviado correctamente");
    } else {
        let error_body = response.text()?;
        //println!("Error al enviar el log: {:?}", response.status());
        println!("Cuerpo de error: {}", error_body);
    }

    Ok(())
}
        
/*  
    Función para leer el archivo proc
    - file_name: El nombre del archivo que se quiere leer.
    - Regresa un Result<String> que puede ser un error o el contenido del archivo.
*/
fn read_proc_file(file_name: &str) -> io::Result<String> {
    // Se crea un Path con el nombre del archivo que se quiere leer.
    let path  = Path::new("/proc").join(file_name);

    /* 
        Se abre el archivo en modo lectura y se guarda en la variable file.
        En caso de que haya un error al abrir el archivo, se regresa un error.
        El signo de interrogación es un atajo para regresar un error en caso de que haya uno.
    */
    let mut file = File::open(path)?;

    // Se crea una variable mutable content que se inicializa con un String vacío.
    let mut content = String::new();

    // Se lee el contenido del archivo y se guarda en la variable content.
    file.read_to_string(&mut content)?;


    // Se regresa el contenido del archivo.
    Ok(content)
}

/* 
    Función para deserializar el contenido del archivo proc a un vector de procesos.
    - json_str: El contenido del archivo proc en formato JSON.
    - Regresa un Result<> que puede ser un error o un SystemInfo.
*/
fn parse_proc_to_struct(json_str: &str) -> Result<SystemInfo, serde_json::Error> {
    // Se deserializa el contenido del archivo proc a un SystemInfo.
    let system_info: SystemInfo = serde_json::from_str(json_str)?;

    // Se regresa el SystemInfo.
    Ok(system_info)
}

//Para separar el json de los contenedores con el registro del sistema general
fn extract_json(content: &str) -> Option<(&str,&str)> {
    // Busca el inicio del JSON, que es el primer carácter '{'
    if let Some(start) = content.find('{') {
        // Devuelve la porción del string desde el inicio del JSON hasta el final
        Some((&content[start..], &content[..start]))
    } else {
        // Si no encuentra el inicio del JSON, devuelve None
        None
    }
}

//#[tokio::main]
//async fn main() {
fn main() {

    // TODO: antes de iniciar el loop, ejecutar el docker-compose.yml y obtener el id del contenedor registro.

    // TODO: Utilizar algo para capturar la señal de terminación y matar el contenedor registro y cronjob.

    loop {
        
        // Creamos una estructura de datos SystemInfo con un vector de procesos vacío.
        let system_info: Result<SystemInfo, _>;

        // Leemos el contenido del archivo proc y lo guardamos en la variable json_str.
        //let json_str = read_proc_file("sysinfo_200915348").unwrap();
        let file_content = read_proc_file("sysinfo_200915348").unwrap();

        // Deserializamos el contenido del archivo proc a un SystemInfo.
        if let Some((json_str, extra_info)) = extract_json(&file_content){
            system_info = parse_proc_to_struct(&json_str);
            println!("{}", extra_info);
            // Dependiendo de si se pudo deserializar el contenido del archivo proc o no, se ejecuta una u otra rama.
            match system_info {
                Ok( info) => {
                    analyzer(info);
                }
                Err(e) => println!("Failed to parse JSON: {}", e),
            }
        } else {
            println!("No JSON found in the file content");
        }
        
        // Dormimos el hilo principal por 10 segundos.
        std::thread::sleep(std::time::Duration::from_secs(10));
    }

}