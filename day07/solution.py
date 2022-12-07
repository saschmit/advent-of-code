#!/usr/bin/python

import sys

class Dirent:
    def __init__(self):
        pass
    def size(self):
        raise NotImplementedError()
    def type(self):
        raise NotImplementedError()
    def disp(self, name, nest):
        raise NotImplementedError()

class Dir(Dirent):
    def __init__(self, parent=None):
        self.contents = {}
        self.parent = parent if parent is not None else self
    def size(self):
        return sum([ent.size() for name, ent in self.contents.items()])
    def add(self, name, ent):
        assert name not in self.contents
        self.contents[name] = ent
    def get(self, name):
        if name == '..':
            return self.parent
        else:
            return self.contents[name]
    def type(self):
        return "dir"
    def disp(self, name, nest):
        lpad = '  ' * nest
        out = "{}- {} (dir)".format(lpad, name)
        for name, ent in self.contents.items():
            out += "\n" + ent.disp(name, nest+1)
        return out

class File(Dirent):
    def __init__(self, size):
        self._sz = size
    def size(self):
        return self._sz
    def type(self):
        return "file"
    def disp(self, name, nest):
        lpad = '  ' * nest
        return "{}- {} (file, size={})".format(lpad, name, self.size())

root = Dir()

cwd = root
command = None
all_dirs = [root]
for line in open(sys.argv[1]):
    tokens = line.strip().split()
    if tokens[0] == '$':
        command = tokens[1]
        if command == 'cd':
            if tokens[2] == '/':
                cwd = root
            else:
                cwd = cwd.get(tokens[2])
        elif command == 'ls':
            pass
        else:
            assert False
    else:
        if command == 'cd':
            assert False
        elif command == 'ls':
            if tokens[0] == 'dir':
                cwd.add(tokens[1], Dir(cwd))
                all_dirs.append(cwd.get(tokens[1]))
            else:
                cwd.add(tokens[1], File(int(tokens[0])))
        else:
            assert False

print(root.disp('/', 0))

if sys.argv[1] == 'sample':
    assert root.get('a').get('e').size() == 584
    assert root.get('a').size() == 94853
    assert root.get('d').size() == 24933642
    assert root.size() == 48381165

part1_total = 0
for dirent in all_dirs:
    size = dirent.size()
    if size <= 100000:
        part1_total += size

print("Part 1: {}".format(part1_total))

total_disk = 70000000
unused_reqd = 30000000

unused = total_disk - root.size()
target_rm = unused_reqd - unused

part2_size = total_disk
for dirent in all_dirs:
    size = dirent.size()
    if size >= target_rm and size < part2_size:
        part2_size = size

print("Part 2: {}".format(part2_size))
