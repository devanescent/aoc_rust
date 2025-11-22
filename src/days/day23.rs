use std::collections::VecDeque;
use crate::aoc_result::AoCResult;
use crate::shared::intcode::{InstructionResult, IntcodeProgram, RunMode};

make_day!(Day23);

pub fn solve_part1(input: &String) -> AoCResult {
    let prgm = IntcodeProgram::new(input, None);
    let mut network = vec![NetworkComputer::new(prgm.clone()); 50];

    // Assign network addresses:
    for (addr, net_comp) in network.iter_mut().enumerate() {
        net_comp.boot(addr);
    }

    // Run computers in network:
    let result;
    'network_loop: loop {
        // Packets produced in this cycle will be made available to the network in the next cycle:
        let mut packets = Vec::<Packet>::new();

        // Process next instruction in each network computer:
        for next_computer in 0..50 {
            let next_packet = network[next_computer].run();
            if let Some(packet) = next_packet {
                if packet.address == 255 {
                    result = packet.Y;
                    break 'network_loop;
                } else {
                    packets.push(packet);
                }
            }
        }

        // Cycle complete: transmit packets
        for p in packets {
            network[p.address].receive_packet(p);
        }
    }

    AoCResult::Num(result as u64)
}

pub fn solve_part2(input: &String) -> AoCResult {
    let prgm = IntcodeProgram::new(input, None);
    let mut network = vec![NetworkComputer::new(prgm.clone()); 50];

    // Assign network addresses:
    for (addr, net_comp) in network.iter_mut().enumerate() {
        net_comp.boot(addr);
    }

    // NAT
    let mut nat_packet = None;
    let mut last_nat_transmission = 0;

    // Run computers in network:
    'network_loop: loop {
        // Packets produced in this cycle will be made available to the network in the next cycle:
        let mut packets = Vec::<Packet>::new();

        // Process next instruction in each network computer:
        for next_computer in 0..50 {
            let next_packet = network[next_computer].run();
            if let Some(packet) = next_packet {
                if packet.address == 255 {
                    nat_packet = Some(packet);
                } else {
                    packets.push(packet);
                }
            }
        }

        // Cycle complete: transmit packets
        for p in packets {
            network[p.address].receive_packet(p);
        }

        // Check if all computers in the network are idle:
        if let Some(nat_payload) = &nat_packet {
            if network.iter().all(|c| c.is_idle()) {
                // Was this value already sent?
                if last_nat_transmission == nat_payload.Y {
                    break 'network_loop;
                } else {
                    // Send NAT packet to computer 0:
                    last_nat_transmission = nat_payload.Y;
                    network[0].receive_packet(nat_payload.clone());
                }
            }   
        }
    }

    AoCResult::Num(last_nat_transmission as u64)
}


#[derive(Clone, PartialEq)]
enum NetworkState {
    ACTIVE,     // normal state while receiving and sending packets
    PASSIVE,    // has not received packets, but may send packets themselves
    IDLE        // if packets are neither sent nor received
}

#[derive(Clone)]
struct NetworkComputer {
    nic: IntcodeProgram,
    packet_queue: VecDeque<Packet>,
    address: usize,
    last_state: InstructionResult,
    network_state: NetworkState,
}

impl NetworkComputer {
    pub fn new(nic: IntcodeProgram) -> Self {
        Self {
            nic: nic,
            packet_queue: VecDeque::new(),
            address: 0,
            last_state: InstructionResult::HALT,
            network_state: NetworkState::ACTIVE
        }
    }

    pub fn boot(&mut self, addr: usize) {
        self.nic.input.push_back(addr as i64);
        self.address = addr;
        self.last_state = self.nic.run(RunMode::Step);
    }

    pub fn run(&mut self) -> Option<Packet> {
        if self.last_state == InstructionResult::WAIT_FOR_INPUT {
            // Dequeue packet if available:
            if let Some(packet) = self.packet_queue.pop_front() {
                self.network_state = NetworkState::ACTIVE;
                self.nic.input.push_back(packet.X);
                self.nic.input.push_back(packet.Y);
            } else {
                self.nic.input.push_back(-1);

                if self.network_state == NetworkState::ACTIVE {
                    self.network_state = NetworkState::PASSIVE;
                } else if self.network_state == NetworkState::PASSIVE {
                    self.network_state = NetworkState::IDLE;
                }
            }

            // Continue input step from last cycle:
            self.nic.run_step();
        }

        // Continue execution:
        self.last_state = self.nic.run_step();

        // Check for packets to send:
        if self.nic.output.len() >= 3 {
            let packet = Packet {
                address: self.nic.output[0] as usize,
                X: self.nic.output[1],
                Y: self.nic.output[2],
            };
            self.nic.output.drain(0..3);
            self.network_state = NetworkState::ACTIVE;
            return Some(packet);
        }

        return None;
    }

    pub fn receive_packet(&mut self, p: Packet) {
        self.packet_queue.push_back(p);
    }

    // Computer must read input twice without sending packets in between to be considered idle:
    pub fn is_idle(&self) -> bool {
        self.packet_queue.is_empty() && self.network_state == NetworkState::IDLE
    }
}

#[derive(Clone)]
#[allow(non_snake_case)]
struct Packet {
    address: usize,
    X: i64,
    Y: i64
}