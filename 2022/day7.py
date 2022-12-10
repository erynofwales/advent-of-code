#!/usr/bin/env python3
# Eryn Wells <eryn@erynwells.me>

'''
New script.
'''

import argparse

TOTAL_DISK_SIZE = 70000000
SIZE_REQUIRED_FOR_UPDATE = 30000000

class File:
    def __init__(self, name, size):
        self.name = name
        self.size = size

    def __repr__(self):
        return f'{self.__class__.__name__}({self.name!r}, {self.size!r})'

class Directory:
    def __init__(self, name):
        self.name = name
        self.directories = {}
        self.files = []
        self.parent = None

    def __repr__(self):
        return f'{self.__class__.__name__}({self.name!r})'

    def add_subdirectory(self, subdirectory):
        assert subdirectory.name not in self.directories
        self.directories[subdirectory.name] = subdirectory
        subdirectory.parent = self

    def add_file(self, file):
        self.files.append(file)

    @property
    def size(self):
        size_of_directories = sum(d.size for d in self.directories.values())
        size_of_files = sum(f.size for f in self.files)
        return size_of_directories + size_of_files

    def pretty_print(self, indent=0):
        print(' ' * indent, self.name, f'({self.size})')
        for d in self.directories.values():
            d.pretty_print(indent=indent+2)
        for f in self.files:
            print(' ' * (indent + 2), repr(f))

def parse_args(argv, *a, **kw):
    parser = argparse.ArgumentParser(*a, **kw)
    parser.add_argument('input')
    args = parser.parse_args(argv)
    return args

def main(argv):
    args = parse_args(argv[1:], prog=argv[0])

    with open(args.input) as f:
        lines = f.readlines()

    print(f'{len(lines)} total lines in input file {args.input}')

    root = Directory('/')
    cwd = root

    for line in lines:
        split_line = line.strip().split(' ')
        if split_line[0] == '$':
            if split_line[1] == 'cd':
                dir_name = split_line[2].strip()
                if dir_name == '/':
                    cwd = root
                elif dir_name == '..':
                    cwd = cwd.parent
                else:
                    cwd = cwd.directories[dir_name]
                print(f'Set cwd to {cwd}')
            elif split_line[1] == 'ls':
                print(f'Nothing to do for `ls` in {cwd}')
                # Nothing to do here.
                pass
        elif split_line[0] == 'dir':
            # dir [name]
            name = split_line[1]
            print(f'Found subdirectory {name} in {cwd}')
            cwd.add_subdirectory(Directory(name))
        else:
            # [size] [name]
            name = split_line[1]
            size = int(split_line[0])
            print(f'Found file {name} with size {size} in {cwd}')
            cwd.add_file(File(name, size))

    root.pretty_print()
    print(f'Part 0: total size of the entire tree: {root.size}')

    root_size = root.size
    free_space = TOTAL_DISK_SIZE - root_size
    minimum_size_of_directory_to_delete = SIZE_REQUIRED_FOR_UPDATE - free_space

    part1_sum = 0
    part2_directories = []
    dfs_stack = [root]
    while len(dfs_stack):
        d = dfs_stack.pop()
        dfs_stack.extend(d.directories.values())
        dir_size = d.size
        if dir_size <= 100000:
            part1_sum += dir_size
        if dir_size > minimum_size_of_directory_to_delete:
            part2_directories.append(d)

    print(f'Part 1: total size of directories with size less than 100,000: {part1_sum}')

    directory_to_delete = min(part2_directories, key=lambda d: d.size)
    print(f'Part 2: size of directory to delete: {directory_to_delete.size}')


if __name__ == '__main__':
    import sys
    result = main(sys.argv)
    sys.exit(0 if not result else result)
