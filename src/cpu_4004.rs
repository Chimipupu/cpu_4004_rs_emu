use std::fmt;
use std::result;

// ビットマスク
const OP_FIM_MASK: u8       = 0b00101110;    // FIM [0b0010(3bit RP) 0][8ビット…データ]
const OP_FIM_RP_MASK: u8    = 0b00001110;    // FIM RP部マスク
const OP_LDM_MASK: u8       = 0b11011111;    // LDM [0b1101][下位4bitデータ]
const OP_LDM_DATA_MASK: u8  = 0b00001111;    // LDM データ部マスク

// Intel 4004の命令セット
const OP_NOP: u8 = 0x00; // NOP: 何もしない
const OP_LD: u8 = 0xA0;  // LD: レジスタからアキュムレータにロード
const OP_XCH: u8 = 0xB0; // XCH: レジスタとAccの交換
const OP_FIM: u8 = OP_FIM_MASK; // FIM 即値 -> レジスタ
const OP_LDM: u8 = OP_LDM_MASK; // LDM: 即値 -> ACC

// Intel 4004の未定義命令
const OP_HALT: u8 = 0xFF; // HALT: 停止（0xFFは決め打ち）

pub struct Memory {
    pub rom: [u8; 4096], // ROM 4KB
    pub ram: [u8; 4096], // RAM 4KB
}

impl Memory {
    pub fn new() -> Self {
        Self {
            rom: [0; 4096],
            ram: [0; 4096],
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &value) in data.iter().enumerate() {
            if i < self.rom.len() {
                self.rom[i] = value;
            }
        }
    }

    pub fn read_ram(&self, address: usize) -> u8 {
        self.ram[address]
    }

    pub fn write_ram(&mut self, address: usize, value: u8) {
        self.ram[address] = value;
    }
}

// intel 4004 の構造体
pub struct i4004 {
    acc: u8,             // アキュムレータACC
    idx_reg: [u8; 8],    // インデックスレジスタ (8ビット x 8ペア)
    pub pc: u16,         // プログラムカウンタ (12ビット)
    pub sp: [u16; 3],    // スタックポインタ (12ビット x 3レベル)
    pub carry_flg: bool, // キャリーフラグ
    pub memory: Memory,  // メモリ(ROM, RAM 4KB)
}

impl i4004 {
    pub fn new() -> Self {
        Self {
            acc: 0,
            idx_reg: [0; 8],
            pc: 0,
            sp: [0; 3],
            carry_flg: false,
            memory: Memory::new(),
        }
    }

    fn refetch(&mut self) -> u8 {
        self.pc = self.pc.wrapping_sub(1);
        let instruction = self.memory.rom[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        instruction
    }

    fn fetch(&mut self) -> u8 {
        let instruction = self.memory.rom[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        instruction
    }

    fn decode(&self, instruction: u8) -> String {
        match instruction {
            // Intel 4004の命令セット
            OP_NOP => "NOP".to_string(),
            OP_LD =>  "LD".to_string(),
            OP_XCH => "XCH".to_string(),
            OP_FIM => "FIM".to_string(),
            OP_LDM => "LDM".to_string(),

            // Intel 4004の未定義命令
            OP_HALT => "HALT".to_string(),

            // TODO: 他の命令セットのデコード

            _ => "UNKNOWN".to_string(),
        }
    }

    fn execute(&mut self, opcode: u8) -> bool {
        let mut result = true;

        match opcode {
            // Intel 4004の命令セット
            OP_NOP => {
                println!("NOP");
            }
            OP_LD => {
                let rn: u8 = self.fetch();
                self.acc = self.idx_reg[rn as usize];
                println!("LD R{}", rn);
            }
            OP_XCH => {
                let rn: u8 = self.fetch();
                let acc_val: u8 = self.acc;
                let rn_val: u8 = self.idx_reg[rn as usize];
                self.acc = rn_val;
                self.idx_reg[rn as usize] = acc_val;
                println!("XCH R{}", rn);
            }
            OP_FIM => {
                let val: u8 = self.fetch();
                let rp: u8 = (self.refetch() & OP_FIM_RP_MASK) >> 1;
                self.idx_reg[rp as usize] = val;
                println!("FIM {} (R{} <- {})", val, rp, val);
            }
            OP_LDM => {
                let val: u8 = self.refetch() & OP_LDM_DATA_MASK;
                self.acc = val;
                println!("LDM {}", val);
            }

            // Intel 4004の未定義命令
            OP_HALT => {
                println!("HALT");
                return false;
            }

            // TODO: 他の命令セットの実行

            _ => {
                println!("Unknown opcode: 0x{:02X}", opcode);
                result = false;
            }
        }
        result
    }


    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.load_rom(program);
        self.pc = 0;
    }

    pub fn run(&mut self) {
        loop {
            let instruction = self.fetch();
            self.decode(instruction);

            if !self.execute(instruction) {
                break;
            }
        }
    }
}

impl fmt::Display for i4004 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Acc: 0x{:02X}", self.acc)?;
        writeln!(f, "IDX Reg: {:X?}", self.idx_reg)?;
        writeln!(f, "Carry Flag: {:?}", self.carry_flg)?;
        writeln!(f, "SP: {:X?}", self.sp)?;
        writeln!(f, "PC: 0x{:02X}", self.pc)?;
        Ok(())
    }
}