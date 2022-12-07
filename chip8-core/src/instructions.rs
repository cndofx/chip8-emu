// #[derive(Debug)]
// pub(crate) enum Instruction {
//     /// CLS - Clear the display
//     CLS,
//     /// RET - Return from a subroutine
//     RET,
//     // SYS(u16),
//     /// `JP addr` - Jump to `addr`
//     JP(u16),
//     /// `CALL addr` - Call subroutine at `addr`
//     CALL(u16),
//     /// `SE Vx, byte` - Skip next instruction if `Vx == byte`
//     SE((u8, u8)),
//     /// `SNE Vx, byte` - Skip next instruction if `Vx != byte`
//     SNE((u8, u8)),
//     Unrecognized,
// }

// impl Instruction {
//     pub fn decode(instruction: u16) -> Self {
//         let nnn = instruction & 0x0FFF; // 12-bit address, lower 12 bits of instruction
//         let kk = (instruction & 0x00FF) as u8; // 8-bit value, lower 8 bits of instruction
//         // let uu = ((instruction & 0xFF00) >> 8) as u8; // 8-bit value, upper 8 bits of instruction
//         let n = (instruction & 0x000F) as u8; // 4-bit value, lowest 4 bits of instruction
//         let x = ((instruction & 0x0F00) >> 8) as u8; // 4-bit value, lower 4 bits of upper byte
//         let y = ((instruction & 0x00F0) >> 4) as u8; // 4-bit value, upper 4 bits of lower byte

//         // println!("parsed instruction: nnn = {nnn}, uu = {uu}, kk = {kk}, n = {n}, x = {x}, y = {y}");

//         match instruction & 0xF000 {
//             0x00 => {
//                 match kk {
//                     0xE0 => Instruction::CLS,
//                     0xEE => Instruction::RET,
//                     _ => Instruction::Unrecognized,
//                 }
//             },
//             0x01 => Instruction::JP(nnn),
//             0x02 => Instruction::CALL(nnn),
//             0x03 => Instruction::SE((x, kk)),
//             0x04 => Instruction::SNE((x, kk)),
//             0x05 => Instruction::
//             _ => Instruction::Unrecognized,
//         }
//     }
// }
