
/*

notes

inc_register doesnt work
jmp doesnt work

*/

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
    //program counter
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
    // loops every byte (opcode in this case) in every line, then stores in ROM
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

    //returns ROM of a certain address
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
    // returns RAM of a certain address
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

    // writes data to ram
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


    //bugged: doesntwork
    // increments a register's value
    fn inc_register(&mut self, reg_index: usize)
    {
        self.registers[reg_index] = self.registers[reg_index].wrapping_add(1)
    }

    fn execute_instruction(&mut self, opcode: u8, operand1: u8, operand2: u8) 
    {
        // asm ahh 
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
            0x03 => // JMP
            {
                let starting_address = operand1 as usize;
                let jmp_to_address = operand2 as usize;
                //write this
            },
            0x04 => // STORE
            {
                let register_index = operand1 as usize;
                let mem_address = operand2 as usize;
                self.ram[mem_address] = self.registers[register_index];
            },
            0x05 => // INC RAM
            {
                let starting_index = operand1 as usize;
                let resulting_address = operand2 as usize;
                let resulting_index = self.ram[starting_index + 1];

            },
            0x06 => // INC REGISTER
            {
                self.inc_register(operand1 as usize);
            },

            _ => panic!("Unknown opcode: 0x{:02X}", opcode),
        }
    }

    // honestly i dont know how this works
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
    // init
    let mut vc = CPU::new();


    let rom = vec![
        0x06, 0x00, // doesnt work, supposed to increment value of register A, doesnt do that
    ];
    vc.run(&rom);
    let result = vc.registers[0];

    println!("thing : {}", result);
    println!("Hooray!");

}
