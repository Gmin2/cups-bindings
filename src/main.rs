use exercise::Connection;

fn main() {
    println!("tesing bindings memory safety");
    
    match Connection::connect("http://localhost:631") {
        Ok(conn) => {
            println!("Connected successfully");
            
            match conn.print("Test document") {
                Ok(()) => println!("Printed successfully"),
                Err(e) => println!("Print error: {}", e),
            }
            
            // connection will be automatically dropped when it goes out of scope
        },
        Err(e) => println!("Connection error: {}", e),
    }
    
    println!("testing complete");
}