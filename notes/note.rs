const PTE_V: u64 = 1 << 0;
const PTE_R: u64 = 1 << 1;
const PTE_W: u64 = 1 << 2;
const PTE_X: u64 = 1 << 3;
const PTE_U: u64 = 1 << 4;
const PTE_G: u64 = 1 << 5;
const PTE_A: u64 = 1 << 6;
const PTE_D: u64 = 1 << 7;

let physical_address = 0x8020_0000u64;

let ppn = physical_address >> 12;

let flags = PTE_V | PTE_R | PTE_W | PTE_A | PTE_D;

let pte = (ppn << 10) | flags; // 

let ppn = (pte >> 10) & ((1u64 << 44) - 1) ;

let physical_address = ppn << 12;

let valid = (pte & PTE_V) != 0;
let readable = (pte & PTE_R) != 0;
let writable = (pte & PTE_W) != 0;
let executable = (pte & PTE_X) != 0;

let pte = pte | PTE_X;

let pte = pte & !PTE_W;

let pte = pte ^ PTE_U;


