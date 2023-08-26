use tonic::{transport::Server, Request, Response, Status};

use mysql::prelude::*;
use mysql::*;

use ppc::user_service_server::{UserService, UserServiceServer};
use ppc::{QueryRequest, QueryResponse, UpdateRequest, UpdateResponse, InsertRequest, InsertResponse, DeleteRequest, DeleteResponse};

pub mod ppc {
    tonic::include_proto!("myapp"); // The string specified here must match the proto package name
}

#[derive(Clone, Debug)]
struct User {
    id: Option<i32>,
    username: String,
    email: String,
    password: String,
}

impl User {
    fn new_empty_user() -> User {
        User {
            id: None,
            username: "".to_string(),
            email: "".to_string(),
            password: "".to_string()
        }
    }

    fn some_id(&self) -> i32 {
        match self.id {
            Some(data) => data,
            None => -1,
        }
    }

}

#[derive(Debug, Default)]
pub struct MyUserService {}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn query(&self, request: Request<QueryRequest>) -> Result<Response<QueryResponse>, Status> {
        println!("Got a request: {:?}", request);

        let binding = select_user(request.into_inner().search.as_str());
        let user = binding.first().expect("No user");
        let reply = ppc::QueryResponse {
            id: user.some_id().to_string(),
            username: user.username.to_string(), 
            email: user.email.to_string(),
            password: user.password.to_string()
        
        };

        Ok(Response::new(reply))
    }
    async fn update(&self, request: Request<UpdateRequest>) -> Result<Response<UpdateResponse>, Status> {
        println!("Got a request: {:?}", request);
        
        let user = request.into_inner();
        let user = update_user(user.username.as_str(), user.email.as_str(), user.password.as_str(), user.id.parse::<i32>().expect("Not parsed"));
        let reply = ppc::UpdateResponse {
            id: user.some_id().to_string()
        };

        Ok(Response::new(reply))
    }
    async fn insert(&self, request: Request<InsertRequest>) -> Result<Response<InsertResponse>, Status> {
        println!("Got a request: {:?}", request);
        
        let user = request.into_inner();
        let user = insert_user(user.username.as_str(), user.email.as_str(), user.password.as_str());
        let reply = ppc::InsertResponse {
            id: user.some_id().to_string()
        };

        Ok(Response::new(reply))
    }
    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        println!("Got a request: {:?}", request);
        
        let user_id = request.into_inner();
        let deleted = delete_user(user_id.id.parse::<i32>().expect("Not parsed"));
        let reply = ppc::DeleteResponse {
            success: deleted
        };

        Ok(Response::new(reply))
    }
    
}

/// Select user
fn select_user(search_value: &str) -> Vec<User> {
    let pool = Pool::new(mysql::Opts::from_url("mysql://root:root@localhost/ppc").expect("No pool")).expect("No pool");

    // DB Connection
    let mut conn = pool.get_conn().expect("Pool not working");

    let query = "SELECT * FROM users WHERE username = ";
    let query = &(query.to_string() + search_value).to_string();
    let query = &(query.to_string() + " OR email = " + search_value).to_string();
    let query = &(query.to_string() + " OR id = " + search_value).to_string();

    let users_res = conn.query_map(query, |(id, username, email, password)| {
        User {id: Some(id), username, email, password}
    });
    // let user_res = conn.query(query);
    let users = match users_res {
        Ok(data) => {
            data
        }
        Err(_er) => {
            vec![User::new_empty_user()]
        }
    };

    users
}
/// Create a new user
fn insert_user(username: &str, email: &str, password: &str) -> User {
    let pool = Pool::new(mysql::Opts::from_url("mysql://root:root@localhost/ppc").expect("No pool")).expect("No pool");

    // DB Connection
    let mut conn = pool.get_conn().expect("Pool not working");

    let _ = conn.exec_drop(
        r"INSERT INTO users (username, email, password) VALUES (:username, :email, :password)",
        params! {
            "username" => &username,
            "email" => &email,
            "password" => &password
        },
    );

    User {
        username: username.to_string(),
        email: email.to_string(),
        password: password.to_string(),
        id: Some(conn.last_insert_id() as i32),
    }
}

/// Update users
fn update_user(username: &str, email: &str, password: &str, id: i32) -> User {
    let pool = Pool::new(mysql::Opts::from_url("mysql://root:root@localhost/ppc").expect("No pool")).expect("No pool");

    // DB Connection
    let mut conn = pool.get_conn().expect("Pool not working");

    let _ = conn.exec_drop(r"UPDATE users SET username = :username, email = :email, password = :password WHERE id = :id", params! {
        "username" => &username,
        "email" => &email,
        "password" => &password,
        "id" => id
    });

    let user_id = conn.last_insert_id();
    let users = select_user( user_id.to_string().as_str());
    match users.first() {
        Some(data) => data.clone(),
        None => User::new_empty_user(),
    }
}

/// Delete user
fn delete_user(id: i32) -> bool {
    let pool = Pool::new(mysql::Opts::from_url("mysql://root:root@localhost/ppc").expect("No pool")).expect("No pool");

    // DB Connection
    let mut conn = pool.get_conn().expect("Pool not working");

    let res = conn.exec_drop(r"DELETE FROM users WHERE id = :id", params!{
        "id" => id
    });

    match res {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let user_service = MyUserService::default();

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}

// #[tokio::main]
// async fn _main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "[::1]:50051".parse()?;
//     let greeter = MyUserService::default();

//     Server::builder()
//         .add_service(UserServiceServer::new(greeter))
//         .serve(addr)
//         .await?;

//     Ok(())
// }

