#!/usr/bin/env python3

import factlib


for i in range(20):
    fact = factlib.fact(i)
    print(f"{i:>2}! = {fact:>20}")

