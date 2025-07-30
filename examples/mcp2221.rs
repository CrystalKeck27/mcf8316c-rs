use mcf8316c::{MCF8316C, registers};

fn main() {
    let device = mcp2221_hal::MCP2221::connect().unwrap();

    let mut mcf8316c = MCF8316C::with_i2c_address(device, 0x5A);

    let mut closed_loop: registers::algorithm_configuration::closed_loop2::ClosedLoop2 =
        mcf8316c.read().unwrap();
    println!("Closed Loop 2: {:?}", closed_loop);

    closed_loop
        .set_motor_res(registers::algorithm_configuration::closed_loop2::MotorResistance::R600);

    mcf8316c.write(&closed_loop).unwrap();

    let closed_loop: registers::algorithm_configuration::closed_loop2::ClosedLoop2 =
        mcf8316c.read().unwrap();

    println!("Closed Loop 2: {:?}", closed_loop);
}
