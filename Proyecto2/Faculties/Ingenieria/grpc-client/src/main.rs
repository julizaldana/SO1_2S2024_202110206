use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use studentgrpc::student_client::StudentClient;
use studentgrpc::StudentRequest;

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
    // Determine the gRPC server address based on the discipline
    let server_addr = match student.discipline {
        1 => "http://go-natacion:50051",    // Natación
        2 => "http://go-atletismo:50051",   // Atletismo
        3 => "http://go-boxeo:50051",       // Boxeo
        _ => return HttpResponse::BadRequest().body("Invalid discipline"),
    };

    // Connect to the appropriate gRPC server
    let mut client = match StudentClient::connect(server_addr.to_string()).await {
        Ok(client) => client,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Failed to connect to gRPC server: {}", e)),
    };

    // Create a request for the gRPC call
    let request = tonic::Request::new(StudentRequest {
        name: student.name.clone(),
        age: student.age,
        faculty: student.faculty.clone(),
        discipline: student.discipline,
    });

    // Send the request to the gRPC server and handle the response
    match client.get_student(request).await {
        Ok(response) => {
            println!("RESPONSE={:?}", response);
            HttpResponse::Ok().json(format!("Student: {:?}", response.into_inner()))
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("gRPC call failed: {}", e)),
    }
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
