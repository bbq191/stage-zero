use std::num::ParseFloatError;

fn main() {
    let result = square("26");
    println!("{:?}", result);
    let result = quiestion_version("26");
    println!("{:?}", result);
}

fn square(val: &str) -> Result<f32, ParseFloatError> {
    match val.parse::<f32>() {
        Ok(num) => Ok(num.powf(2.0)),
        Err(e) => Err(e),
    }
}

fn quiestion_version(val: &str) ->Result<f32,ParseFloatError> {
    let num = val.parse::<f32>()?;
    Ok(num * num)
}
