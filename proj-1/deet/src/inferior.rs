use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::process::{Child, Command};
use std::os::unix::process::CommandExt;
use crate::dwarf_data::DwarfData;
use std::mem::size_of;
use std::collections::HashMap;
use crate::debugger::Breakpoint;



pub enum Status {
    /// Indicates inferior stopped. Contains the signal that stopped the process, as well as the
    /// current instruction pointer that it is stopped at.
    Stopped(signal::Signal, usize),

    /// Indicates inferior exited normally. Contains the exit status code.
    Exited(i32),

    /// Indicates the inferior exited due to a signal. Contains the signal that killed the
    /// process.
    Signaled(signal::Signal),
}

/// This function calls ptrace with PTRACE_TRACEME to enable debugging on a process. You should use
/// pre_exec with Command to call this in the child process.
fn child_traceme() -> Result<(), std::io::Error> {
    ptrace::traceme().or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ptrace TRACEME failed",
    )))
}

fn align_addr_to_word(addr: usize) -> usize {
    addr & (-(size_of::<usize>() as isize) as usize)
}

pub struct Inferior {
    child: Child,
}

impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>, breakpoints: &mut HashMap<usize, Breakpoint>) -> Option<Inferior> {
        let mut cmd = Command::new(target);
        cmd.args(args);
        unsafe {
            cmd.pre_exec(child_traceme);
        }
        let mut inferior = Inferior { child: cmd.spawn().ok()? };

        let bp = breakpoints.clone();
        for addr in bp.keys() {
            match inferior.write_byte(*addr, 0xcc) {
                Ok(orig_instr) => { breakpoints.insert(*addr, Breakpoint{ addr: *addr, orig_byte: orig_instr }); },
                Err(_) => println!("Invalid breakpoint address {:#x}", addr),
            }
        }
        let status = inferior.wait(None).ok()?;

        match status {
            Status::Stopped(sign, _) => {
                match sign {
                    signal::Signal::SIGTRAP => Some(inferior),
                    _ => None
                }
            }, 
            Status::Exited(_) => None,
            Status::Signaled(_) => None,
        }
    }

    pub fn write_byte(&mut self, addr: usize, val: u8) -> Result<u8, nix::Error> {
        let aligned_addr = align_addr_to_word(addr);
        let byte_offset = addr - aligned_addr;
        let word = ptrace::read(self.pid(), aligned_addr as ptrace::AddressType)? as u64;
        let orig_byte = (word >> 8 * byte_offset) & 0xff;
        let masked_word = word & !(0xff << 8 * byte_offset);
        let updated_word = masked_word | ((val as u64) << 8 * byte_offset);
        ptrace::write(
            self.pid(),
            aligned_addr as ptrace::AddressType,
            updated_word as *mut std::ffi::c_void,
        )?;
        Ok(orig_byte as u8)
    }

    /// Returns the pid of this inferior.
    pub fn pid(&self) -> Pid {
        nix::unistd::Pid::from_raw(self.child.id() as i32)
    }

    /// Calls waitpid on this inferior and returns a Status to indicate the state of the process
    /// after the waitpid call.
    pub fn wait(&self, options: Option<WaitPidFlag>) -> Result<Status, nix::Error> {
        Ok(match waitpid(self.pid(), options)? {
            WaitStatus::Exited(_pid, exit_code) => Status::Exited(exit_code),
            WaitStatus::Signaled(_pid, signal, _core_dumped) => Status::Signaled(signal),
            WaitStatus::Stopped(_pid, signal) => {
                let regs = ptrace::getregs(self.pid())?;
                Status::Stopped(signal, regs.rip as usize)
            }
            other => panic!("waitpid returned unexpected status: {:?}", other),
        })
    }

    pub fn con(&mut self, breakpoints: &HashMap<usize, Breakpoint>) -> Result<Status, nix::Error> {
        let mut regs = ptrace::getregs(self.pid())?;
        let rip = regs.rip as usize;

        if let Some(bp) = breakpoints.get(&(rip - 1)) {
            self.write_byte(bp.addr, bp.orig_byte).ok();

            regs.rip = (rip - 1) as u64;
            ptrace::setregs(self.pid(), regs).ok();

            ptrace::step(self.pid(), None).ok();

            match self.wait(None).ok().unwrap() {
                Status::Exited(exit_code) => return Ok(Status::Exited(exit_code)),
                Status::Signaled(signal) => return Ok(Status::Signaled(signal)),
                Status::Stopped(_, _) => {
                    self.write_byte(rip - 1, 0xcc).ok();
                }
            }
        }
        match ptrace::cont(self.pid(), None) {
            Ok(_) => self.wait(None),
            Err(err) => Err(err)
        }
    }

    pub fn kill(&mut self) -> Result<std::process::ExitStatus, std::io::Error> {
        match self.child.kill() {
            Ok(_) => {
                let status = self.child.wait()?;
                Ok(status)
            },
            Err(err) => Err(err)
        }
    }

    pub fn print_backtrace(&self, debug_data: &DwarfData) -> Result<(), nix::Error> {
        let regs = ptrace::getregs(self.pid())?;
        
        let mut rip = regs.rip as usize;
        let mut rbp = regs.rbp as usize;

        loop {
            match (debug_data.get_line_from_addr(rip), debug_data.get_function_from_addr(rip)) {
                (Some(line_info), Some(function)) => {
                    println!("{} ({}:{})", function, line_info.file, line_info.number);
                    if function.eq("main") {
                        break;
                    }
                    rip = ptrace::read(self.pid(), (rbp + 8) as ptrace::AddressType)? as usize;
                    rbp = ptrace::read(self.pid(), rbp as ptrace::AddressType)? as usize;   
                },

                (_, _) => {
                    println!("%rip register: {:#x} \n%rbp register: {:#x}", rip, rbp);
                    break;
                }

            }
        }
        Ok(())
    }
}
