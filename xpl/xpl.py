#! /usr/bin/python3

import angr
import claripy

proj = angr.Project('./shellcode', main_opts={'base_addr': 0}, auto_load_libs=False)

input_length = 45
prefix = b'TISC{th1s_1s_n0t_th3_ac7u4l_fl4g_lM40}_MJd_U2'

# Creating a symbolic bitvector for each character:
input_chars = [claripy.BVS("flag_%d" % i, 8) for i in range(input_length)]
inpt = claripy.Concat(*input_chars)

state = proj.factory.entry_state(addr=0x41c8, stdin=inpt)

index = 0
for byte in input_chars:
    state.solver.add(byte >= 0x20, byte <= 0x7e)
    if index < 38:
        state.solver.add(byte == prefix[index])
        index += 1

state.memory.store(addr=(state.regs.rbp-0x110), data=inpt, endness=proj.arch.memory_endness)

# Establish the simulation with the entry state
simgr = proj.factory.simulation_manager(state)

# Finding a state that prints the correct prompt
simgr.explore(find=lambda s: b'Morbed' in s.posix.dumps(1))
print("len(simgr.found) = {}".format(len(simgr.found)))

if len(simgr.found) > 0:
    s = simgr.found[0]
    print(s.solver.eval(inpt, cast_to=bytes))
    print(s.posix.dumps(1))
