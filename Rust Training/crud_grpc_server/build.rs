use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("./proto/employee.proto")?;
    tonic_build::compile_protos("./proto/student.proto")?;
    tonic_build::compile_protos("./proto/customerService.proto")?;
    
    Ok(())
}
