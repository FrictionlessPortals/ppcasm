//! ``ppcasm`` - A PowerPC disassembler.

// Integer Instructions
pub mod integer;

/// Testing Instructions
const INSTRUCTIONS: [u8; 52] = [
    0x94, 0x21, 0xff, 0xf8, // stwu r1, -8(r1)
    0x7c, 0x08, 0x02, 0xa6, // mflr r0
    0x90, 0x01, 0x00, 0x0c, // stw r0, 0xc(r1)
    0x4b, 0xff, 0xfd, 0xe9, // bl 0x80005634
    0x3c, 0x60, 0x80, 0x02, // lis r3, 0x8002
    0x38, 0x63, 0x1c, 0x40, // addi r3, r3, 0x1c40
    0x4c, 0xc6, 0x31, 0x82, // crclr 6
    0x48, 0x01, 0x3b, 0x49, // bl 0x800193a4
    0x80, 0x01, 0x00, 0x0c, // lwz r0, 0xc(r1)
    0x38, 0x60, 0x00, 0x00, // li r3, 0
    0x38, 0x21, 0x00, 0x08, // addi r1, r1, 8
    0x7c, 0x08, 0x03, 0xa6, // mtlr r0
    0x4e, 0x80, 0x00, 0x20, // blr
];

/// Instruction Container
type Container = (u8, u8, u8, u8);

macro_rules! combine_bytes {
    ($i: expr, $j: expr, $k: expr, $z: expr) => {
        (($i as u32) << 24) | (($j as u32) << 16) | (($k as u32) << 8) | ($z as u32)
    };
}

macro_rules! extract_opcode {
    ($i: expr) => {
        // Assumes first byte.
        $i >> 2
    };
}

/// Split given array of bytes into instructions.
fn split_array(bytes: &[u8]) -> Vec<Container> {
    // Check if the given array length is divisible by 4.
    assert!(bytes.len() % 4 == 0);

    // Split the array into chunks of 4 bytes.
    // All PPC instructions are 32-bits in length or 4 bytes.
    bytes
        .chunks(4)
        .map(|b| (b[0], b[1], b[2], b[3]))
        .collect()
}

/// Match a given instruction container.
fn match_instruction(instruction: Container) {
    // Extract the opcode from the instruction.
    let opcode = extract_opcode!(instruction.0);

    // Match the opcode.
    match opcode {
        0b011111 => {
            // Opcode: 0x7C
            println!("0x7C opcode");
        },
        0b001110 => {
            // Opcode: 0x38
            println!("0x38 opcode");
        },
        0b001100 => {
            // Opcode: 0x30
            println!("0x30 opcode");
        },
        0b001101 => {
            // Opcode: 0x34
            println!("0x34 opcode");
        },
        0b001111 => {
            // Opcode: 0x3C
            println!("0x3C opcode");
        },
        0b011100 => {
            // Opcode: 0x70
            println!("0x70 opcode");
        },
        0b011101 => {
            // Opcode: 0x74
            println!("0x74 opcode");
        },
        0b010010 => {
            // Opcode: 0x48
            println!("0x48 opcode");
        },
        0b010000 => {
            // Opcode: 0x40
            println!("0x40 opcode");
        },
        0b010011 => {
            // Opcode: 0x4C
            println!("0x4C opcode");
        },
        0b001011 => {
            // Opcode: 0x2C
            println!("0x2C opcode");
        },
        0b001010 => {
            // Opcode: 0x28
            println!("0x28 opcode");
        },
        0b111111 => {
            // Opcode: 0xFC
            println!("0xFC opcode");
        },
        0b111011 => {
            // Opcode: 0xEC
            println!("0xEC opcode");
        }
        0b100010 => {
            // Opcode: 0x88
            println!("0x88 opcode");
        },
        0b100011 => {
            // Opcode: 0x8C
            println!("0x8C opcode");
        },
        0b110010 => {
            // Opcode: 0xC8
            println!("0xC8 opcode");
        },
        0b110011 => {
            // Opcode: 0xCC
            println!("0xCC opcode");
        },
        0b110000 => {
            // Opcode: 0xC0
            println!("0xC0 opcode");
        },
        0b110001 => {
            // Opcode: 0xC4
            println!("0xC4 opcode");
        },
        0b101010 => {
            // Opcode: 0xA8
            println!("0xA8 opcode");
        },
        0b101011 => {
            // Opcode: 0xAC
            println!("0xAC opcode");
        },
        0b101000 => {
            // Opcode: 0xA0
            println!("0xA0 opcode");
        },
        0b101001 => {
            // Opcode: 0xA4
            println!("0xA4 opcode");
        },
        0b101110 => {
            // Opcode: 0xB8
            println!("0xB8 opcode");
        },
        0b100000 => {
            // Opcode: 0x80
            println!("0x80 opcode");
        },
        0b100001 => {
            // Opcode: 0x84
            println!("0x84 opcode");
        },
        0b000111 => {
            // Opcode: 0x1C
            println!("0x1C opcode");
        },
        0b011000 => {
            // Opcode: 0x60
            println!("0x60 opcode");
        },
        0b011001 => {
            // Opcode: 0x64
            println!("0x64 opcode");
        },
        0b010100 => {
            // Opcode: 0x50
            println!("0x50 opcode"); 
        },
        0b010101 => {
            // Opcode: 0x54
            println!("0x54 opcode");
        },
        0b010111 => {
            // Opcode: 0x5C
            println!("0x5C opcode");
        },
        0b010001 => {
            // Opcode: 0x44
            println!("0x44 opcode");
        },
        0b100110 => {
            // Opcode: 0x98
            println!("0x98 opcode");
        },
        0b100111 => {
            // Opcode: 0x9C
            println!("0x9C opcode");
        },
        0b110110 => {
            // Opcode: 0xD8
            println!("0xD8 opcode");
        },
        0b110111 => {
            // Opcode: 0xDC
            println!("0xDC opcode");
        },
        0b110100 => {
            // Opcode: 0xD0
            println!("0xD0 opcode");
        },
        0b110101 => {
            // Opcode: 0xD4
            println!("0xD4 opcode");
        },
        0b101100 => {
            // Opcode: 0xB0
            println!("0xB0 opcode");
        },
        0b101101 => {
            // Opcode: 0xB4
            println!("0xB4 opcode");
        },
        0b101111 => {
            // Opcode: 0xBC
            println!("0xBC opcode");
        },
        0b100100 => {
            // Opcode: 0x90
            println!("0x90 opcode");
        },
        0b100101 => {
            // Opcode: 0x94
            println!("0x94 opcode");
        },
        0b001000 => {
            // Opcode: 0x20
            println!("0x20 opcode");
        },
        0b000011 => {
            // Opcode: 0x0C
            println!("0x0C opcode");
        },
        0b011010 => {
            // Opcode: 0x68
            println!("0x68 opcode");
        },
        0b011011 => {
            // Opcode: 0x6C
            println!("0x6C opcode");
        }
        _ => println!("Opcode does not exist")
    }
}

fn main() {
    // Convert the array of bytes into instruction containers.
    let instructions = split_array(&INSTRUCTIONS);
    instructions.iter().for_each(|i| {
        match_instruction(*i);
    });
    println!("Instructions: {:#x?}", instructions);
}
