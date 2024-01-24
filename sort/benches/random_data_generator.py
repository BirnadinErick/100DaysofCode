#!/usr/bin/env python3

import argparse
import random

def generate_random_data(filename, num_items, range_start, range_end):
    with open(filename, 'w') as f:
        for _ in range(num_items):
            f.write(f"{random.randint(range_start, range_end)}\n")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate a file with random integers.")
    parser.add_argument("filename", type=str, help="The name of the output file")
    parser.add_argument("num_items", type=int, help="Number of random integers to generate")
    parser.add_argument("range_start", type=int, help="Start of the range for random integers")
    parser.add_argument("range_end", type=int, help="End of the range for random integers")

    args = parser.parse_args()

    generate_random_data(args.filename, args.num_items, args.range_start, args.range_end)
    print(f"Generated {args.num_items} random integers in {args.filename}")

