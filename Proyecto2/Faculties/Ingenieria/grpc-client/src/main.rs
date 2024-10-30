use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use studentgrpc::student_client::StudentClient;
use studentgrpc::StudentRequest;
use tokio::task;

pub mod studentgrpc {
    tonic::include_proto!("studentgrpc");
}

#[derive(Deserialize, Serialize)]
struct StudentData {
    name: String,
    age: i32,
    faculty: String,
    discipline: i32,
}

async fn handle_student(student: web::Json<StudentData>) -> impl Responder {
    let student_data = student.into_inner();

    // Spawn a new thread for each request to handle it concurrently
    task::spawn(async move {
        // Determine the gRPC server address based on the discipline
        let server_addr = match student_data.discipline {
            1 => "http://go-natacion:50051",    // Natación
            2 => "http://go-atletismo:50051",   // Atletismo
            3 => "http://go-boxeo:50051",       // Boxeo
            _ => {
                println!("Invalid discipline");
                return;
            },
        };

        // Connect to the appropriate gRPC server
        let mut client = match StudentClient::connect(server_addr.to_string()).await {
            Ok(client) => client,
            Err(e) => {
                println!("Failed to connect to gRPC server: {}", e);
                return;
            }
        };

        // Create a request for the gRPC call
        let request = tonic::Request::new(StudentRequest {
            name: student_data.name,
            age: student_data.age,
            faculty: student_data.faculty,
            discipline: student_data.discipline,
        });

        // Send the request to the gRPC server and handle the response
        match client.get_student(request).await {
            Ok(response) => {
                println!("Response from gRPC server: {:?}", response.into_inner());
            },
            Err(e) => {
                println!("gRPC call failed: {}", e);
            }
        }
    });

    // Respond immediately without waiting for the spawned thread to finish
    HttpResponse::Ok().body("Request received and being processed")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8080");
    HttpServer::new(|| {
        App::new()
            .route("/ingenieria", web::post().to(handle_student))
    })
    .bind("0.0.0.0:8080")? // Listen on all network interfaces
    .run()
    .await
}
