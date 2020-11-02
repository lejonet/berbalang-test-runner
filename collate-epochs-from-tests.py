#!/usr/bin/env python
# -*- coding: utf-8 -*-

import os, subprocess, sys, argparse

parser = argparse.ArgumentParser(description='Parse the directories that ROPER creates to collate total amount of epochs run')
parser.add_argument('start_directory', metavar='f', type=str, nargs='?', help="path to the directory that holds the output from ROPER tests")
args = parser.parse_args()

if not args.start_directory:
    print("You need to specify a start directory")
    sys.exit(1)

start_directory = args.start_directory

dirs = os.listdir(start_directory)

islands_per_test = dict()
for dir in dirs:
        island_dirs = list()
        for island_dir in os.listdir(f"./{dir}"):
            if "island" in island_dir:
                    island_dirs.append(island_dir)

        islands_per_test[dir] = island_dirs

epochs = 0
for test, dirs in islands_per_test.items():
    for dir in dirs:
        path = f"{test}/{dir}/best_statistics.csv"
        last_line = subprocess.run(["tail","-n","1", path], encoding="UTF-8", stdout=subprocess.PIPE)
        epoch = int(last_line.stdout.rstrip().split(',')[0])
        epochs += epoch

print(f"The tests in {start_directory} totals {epochs} epochs.")
