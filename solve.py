#!/usr/bin/env python

import pwn
import ast
from Crypto.Cipher import AES
from Crypto.Util.Padding import unpad

pwn.context.timeout = 1

io = pwn.process("target/release/insanely-floaty")
io.recvuntil(b"token: ")
token_ct_floats = ast.literal_eval(io.recvline().strip().decode())
token_ct = int("".join("1" if x == 1.0 else "0" for x in token_ct_floats), 2).to_bytes(32, "big")

def enc(io, pt: list[float]) -> list[float]:
    io.sendlineafter(b"midnight:", f"{pt}".encode())
    io.readuntil(b"thats: ")
    return ast.literal_eval(io.readline().strip().decode())

epsilon = 0.01
key_bits = [False] * 128
for i, _ in enumerate(key_bits):
    pt = [1.0 if x else 0.0 for x in key_bits]

    pt[i] = 1.0 + epsilon
    x = enc(io, pt)

    pt[i] = 1.0 - epsilon
    xp = enc(io, pt)

    if x == xp:
        key_bits[i] = True

key = int("".join("1" if x else "0" for x in key_bits), 2).to_bytes(16, "big")
cipher = AES.new(key=key, mode=AES.MODE_ECB)
pt = unpad(cipher.decrypt(token_ct), 16)
token = int.from_bytes(pt, "big")
token_bits = [1.0 if x == "1" else 0.0 for x in f"{token:0192b}"]

io.sendlineafter(b"midnight:", f"{token_bits}".encode())

io.readuntil(b"flag: ")
flag = io.readline().strip().decode()
io.close()

print(flag)
