use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::Local;
use reqwest::Client;
use tokio;
use serde_json::json;

/* 
    El #[derive (macro...)] es una característica de Rust que permite a los desarrolladores
    agregar funcionalidades a sus estructuras de datos. En este caso, estamos agregando
    la capacidad de serializar y deserializar la estructura de datos a JSON que es parte de la librería
    serde.
*/

// STRUCT SYSTEMINFO con informacion RAMmemory y Processes

#[derive(Debug, Serialize, Deserialize)]
struct SystemInfo {
    #[serde(rename = "RAMmemory")]
    rammemory: Vec<Rammemory>, // Ahora incluye el vector Rammemory
    #[serde(rename = "Processes")]
    processes: Vec<Process>
}

/* 
    Además de esto, estamos implementando los traits Eq, Ord y PartialOrd para poder comparar
    los procesos en base a su uso de CPU y memoria.

    Serde nos deja implementar macros a acada campo de la estructura de datos para poder renombrar
    los campos en el JSON que se genere.
*/

// STRUCT PROCESO - CONTENEDOR

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Process {
    #[serde(rename = "PID")]
    pid: u32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Cmdline")]
    cmd_line: String,
    #[serde(rename = "Vsz")]
    vsz: u32,
    #[serde(rename = "Rss")]
    rss: u32,
    #[serde(rename = "MemoryUsage")]
    memory_usage: f64,
    #[serde(rename = "CPUUsage")]
    cpu_usage: f64,
}

// STRUCT INFORMACION MEMORIA RAM

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Rammemory {
    #[serde(rename = "TotalRAM")]
    totalram: u32,
    #[serde(rename = "FreeRAM")]
    freeram: u32,
    #[serde(rename = "UsedRAM")]
    usedram: u32,
}

// STRUCT INFORMACION LOG PROCESS 

#[derive(Debug, Serialize, Clone)]
struct LogProcess {
    pid: u32,
    container_id: String,
    name: String,
    vsz: u32,
    rss: u32,
    memory_usage: f64,
    cpu_usage: f64,
}

// STRUCT INFORMACION LOG MEMORY

#[derive(Debug, Serialize, Clone)]
struct LogMemory {
    totalram: u32,
    freeram: u32,
    usedram: u32,
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
    
    ORDENAMIENTO:

    1. RAM
    2. CPU
    3. VSZ
    4. RSS

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

async fn analyzer( system_info:  SystemInfo) {


    // Creamos un vector vacío para guardar los logs de los procesos.
    let mut log_proc_list: Vec<LogProcess> = Vec::new();

    // Se crea un log con la información de memoria. 
    let mut log_mem_list: Vec<LogMemory> = Vec::new();

    /* 
        Creamos un vector vacío para guardar los logs del sistema.
        En este caso, no se guardará nada, pero se puede modificar para guardar
        información del sistema.
    */
    let mut processes_list: Vec<Process> = system_info.processes;

    let mut infomem_list: Vec<Rammemory> = system_info.rammemory;


    /* 
        processes_list.sort(), el método sort usará partial_cmp y cmp para comparar y 
        ordenar los procesos en el vector processes_list basándose en el uso de CPU y memoria.
    */
    processes_list.sort();


    // Dividimos la lista de procesos en dos partes iguales.
    let (lowest_list, highest_list) = processes_list.split_at(processes_list.len() / 2);


    // Hacemos un print de los contenedores de bajo consumo en las listas.
    println!("*-*-*-*-*-*-*-*-*-*-*-*-*-Bajo consumo*-*-*-*-*-*-*-*-*-*-*-*-*-");
    for process in lowest_list {
        println!("PID: {}, Name: {}, container ID: {}, Vsz : {}, Rss : {} , Memory Usage: {}, CPU Usage: {}", process.pid, process.name, process.get_container_id(), process.vsz, process.rss, process.memory_usage, process.cpu_usage);
    }

    println!("------------------------------");

    println!("*-*-*-*-*-*-*-*-*-*-*-*-*-Alto consumo*-*-*-*-*-*-*-*-*-*-*-*-*-");
    for process in highest_list {
        println!("PID: {}, Name: {}, Icontainer ID {}, Vsz : {}, Rss : {} , Memory Usage: {}, CPU Usage: {}", process.pid, process.name, process.get_container_id(), process.vsz, process.rss, process.memory_usage, process.cpu_usage);
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
            let log_process = LogProcess {
                pid: process.pid,
                container_id: process.get_container_id().to_string(),
                name: process.name.clone(),
                vsz: process.vsz,
                rss: process.rss,
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage,
            };
    
            log_proc_list.push(log_process.clone());

            // Matamos el contenedor.
            let _output = kill_container(&process.get_container_id());

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
            let log_process = LogProcess {
                pid: process.pid,
                container_id: process.get_container_id().to_string(),
                name: process.name.clone(),
                vsz: process.vsz,
                rss: process.rss,
                memory_usage: process.memory_usage,
                cpu_usage: process.cpu_usage
            };
    
            log_proc_list.push(log_process.clone());

            // Matamos el contenedor.
            let _output = kill_container(&process.get_container_id());

        }
    }


    // FUNCION SEND_LOGS_TO_SERVER: ayuda para enviar los logs al server python fastapi
    if let Err(e) = send_logs_to_server(&log_proc_list, &infomem_list).await {
        eprintln!("Error al enviar logs: {}", e);
    }

    let now = Local::now(); // Obtiene la fecha y hora actual


    println!("*-*-*-*-*-*-*-*-*-*-*-*-*-Contenedores matados*-*-*-*-*-*-*-*-*-*-*-*-*-");
    for process in &log_proc_list {
        println!("PID: {}, Name: {}, Container ID: {}, Vsz : {}, Rss : {} , Memory Usage: {}, CPU Usage: {}, Fecha: {} ", process.pid, process.name, process.container_id, process.vsz, process.rss, process.memory_usage, process.cpu_usage, now.format("%Y-%m-%d %H:%M:%S"));
    }

    println!("*-*-*-*-*-*-*-*-*-*-*-*-*-Información RAM*-*-*-*-*-*-*-*-*-*-*-*-*-");
    for meminfo in &infomem_list {
        println!("TotalRAM: {}, FreeRAM: {}, UsedRAM: {}, Fecha: {}", meminfo.totalram, meminfo.freeram, meminfo.usedram, now.format("%Y-%m-%d %H:%M:%S"));
    }

    println!("------------------------------");
    
}

async fn send_logs_to_server(log_proc_list: &Vec<LogProcess>, infomem_list: &Vec<Rammemory>) -> Result<(), Box<dyn std::error::Error>> {
    // Crear el timestamp
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

    // Construir el JSON de la memoria RAM
    let ram_memory: Vec<_> = infomem_list.iter().map(|meminfo| {
        json!({
            "totalram": meminfo.totalram,
            "freeram": meminfo.freeram,
            "usedram": meminfo.usedram,
            "timestamp": timestamp  // igual que en el modelo de FastAPI
        })
    }).collect();

    // Construir el JSON de los procesos eliminados
    let killed_processes: Vec<_> = log_proc_list.iter().map(|process| {
        json!({
            "pid": process.pid,
            "name": process.name,
            "container_id": process.container_id,
            "vsz": process.vsz,
            "rss": process.rss,
            "memory_usage": process.memory_usage,
            "cpu_usage": process.cpu_usage,
            "timestamp": timestamp  // igual que en el modelo de FastAPI
        })
    }).collect();

    // Construir el JSON final
    let payload = json!({
        "RAMmemory": ram_memory,
        "KilledProcesses": killed_processes
    });

    // Crear un cliente HTTP
    let client = Client::new();

    // Hacer la petición POST al servidor FastAPI
    let response = client.post("http://127.0.0.1:8000/logs")
        .json(&payload)  // Pasar el JSON como cuerpo de la petición
        .send()
        .await?;

    // Comprobar la respuesta
    if response.status().is_success() {
        println!("Logs enviados exitosamente.");
    } else {
        println!("Error al enviar logs: {}", response.status());
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
    //println!("{}",content);

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


#[tokio::main]
async fn main() {
    loop {
        // Creamos una estructura de datos SystemInfo con un vector de procesos vacío.
        let system_info: Result<SystemInfo, _>;

        // Leemos el contenido del archivo proc y lo guardamos en la variable json_str.
        let json_str = read_proc_file("sysinfo_202110206").unwrap();

        // Deserializamos el contenido del archivo proc a un SystemInfo.
        system_info = parse_proc_to_struct(&json_str);

        // Dependiendo de si se pudo deserializar el contenido del archivo proc o no, se ejecuta una u otra rama.
        match system_info {
            Ok(info) => {
                analyzer(info).await; // Aquí se espera el futuro
            }
            Err(e) => println!("Failed to parse JSON: {}", e),
        }

        // Dormimos el hilo principal por 10 segundos.
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await; // await también para sleep
    }
}
