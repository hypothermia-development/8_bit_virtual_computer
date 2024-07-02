use std::fs::File;
use std::io::{self, Read};
const RAM_SIZE: usize = 4096;
const ROM_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 4;
const MEMORY_SIZE: usize = 65536;


// define cpu basics
struct CPU
{
    registers: [u8; 4],
    pc: u16, 
    memory: [u8; 65536],
    ram: [u8; RAM_SIZE],
    rom: Vec<u8>,




}


impl CPU
{
    fn new() -> Self
    {
        CPU
        {
            pc: 0,
            registers: [0; REGISTER_COUNT],
            memory: [0; MEMORY_SIZE],
            ram: [0; RAM_SIZE],
            rom: vec![0; ROM_SIZE],
        }
       // cpu.load_rom_from_file("rom_data.txt").expect("Failed to load rom");
       // cpu
    }

    fn fetch_instruction(&mut self, memory: &[u8]) -> Option<(u8, u8, u8)>
    {
        if self.pc as usize + 2 >= memory.len()
        {
            return None;
        }
        let opcode = memory[self.pc as usize];
        let operand1 = memory[(self.pc + 1) as usize];
        let operand2 = memory[(self.pc +2) as usize];
        self.pc += 3;
        Some((opcode, operand1, operand2))
    }

    fn load_rom_from_file(&mut self, filename: &str) -> io::Result<()>
    {
        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        for (i, &byte) in buffer.iter().enumerate()
        {
            if i < ROM_SIZE
            {
                self.rom[i] = byte
            } 
            else 
            {
                break;
            }
        }
        Ok(())
    }


    fn read_rom(&self, address: usize) -> u8
    {
        if address < ROM_SIZE
        {
            self.rom[address]
        }
        else
        {
            panic!("Out of bounds access to ROM!");
        }
    }

    fn read_ram(&self, address: usize) -> u8
    {
        if address < RAM_SIZE
        {
            self.ram[address]
        }
        else
        {
            panic!("Out of bounds access to RAM!");
        }
    }


    fn write_ram(&mut self, address: usize, data: u8)
    {
        if address < RAM_SIZE
        {
            self.ram[address] = data;
        }
        else
        {
            panic!("Out of bounds access to RAM!");
        }
    }


    // CONTROL UNIT

    fn execute_instruction(&mut self, opcode: u8, operand1: u8, operand2: u8) 
    {
        match opcode {
            0x01 => //  LOAD 
            {
                let value = operand2;
                let register_index = operand1 as usize;
                self.registers[register_index] = value;
            },

            0x02 => // ADD
            {
                let dest_index = operand1 as usize;
                let src_index = operand2 as usize;
                self.registers[dest_index] = self.registers[dest_index].wrapping_add(self.registers[src_index]);
            },
            0x03 => // SUB
            {
                let value1 = operand1;
                let value2 = operand2;
                CPU::sub(operand1, operand2);
            },
            0x04 => // STORE
            {
                let register_index = operand1 as usize;
                let mem_address = operand2 as usize;
                self.ram[mem_address] = self.registers[register_index];
            },

            _ => panic!("Unknown opcode: 0x{:02X}", opcode),
        }
    }

    fn run(&mut self, memory: &[u8])
    {
        while let Some((opcode, operand1, operand2)) = self.fetch_instruction(memory)
        {
            self.execute_instruction(opcode, operand1, operand2);
        }        
    }




}




fn main() 
{

    let mut vc = CPU::new();


    let rom = vec![
        0x01, 0x00, 0x05, // LOAD A 5
        0x01, 0x01, 0x0A, // LOAD B 10
        0x02, 0x00, 0x01, // ADD A B
        0x04, 0x00, 0x20, // STORE A 0x20

    ];
    vc.run(&rom);
    let result = vc.ram[0x20];

    println!("result stored in ram at 0x20: {}", result);
    println!("Hooray!");

}
