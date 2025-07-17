#!/usr/bin/env python3

import os
import re

def extract_score(file_path):
    with open(file_path, 'r') as f:
        content = f.read()
    match = re.search(r'score\s*=\s*(\d+)', content)
    return int(match.group(1)) if match else None

def find_best_result(directory='./results'):
    best_score = float('inf')
    best_file = None

    for fname in os.listdir(directory):
        if fname.startswith('params_') and fname.endswith('.txt'):
            full_path = os.path.join(directory, fname)
            score = extract_score(full_path)
            if score is not None and score < best_score:
                best_score = score
                best_file = full_path

    if best_file:
        print(f"Best score: {best_score}")
        print(f"File: {best_file}")
    else:
        print("No valid score found.")

if __name__ == "__main__":
    find_best_result()
