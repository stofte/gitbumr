from pathlib import Path
from subprocess import run
from random import randint
import os
import argparse
import json
import re
import fileinput

# Argument handling
parser = argparse.ArgumentParser()
parser.add_argument("cmds", help="json command script")
parser.add_argument("path", help="git repository that will be created")
args = parser.parse_args()

if os.path.exists(args.path):
    print("Path already exists", args.path)
    exit()

edit_cnt = 0
commit_cnt = 0
def edit_val():
    global edit_cnt
    edit_cnt += 1
    return "[edit "+ str(edit_cnt) + "]"

def commit_msg():
    global commit_cnt
    commit_cnt += 1
    return "commit nr. " + str(commit_cnt)

def execute_cmd(cmd_type, cmd, cmd_arg):
    print(cmd, cmd_arg)
    if cmd_type == "git":
        if cmd_arg == "init":
            
            run(["git.exe", "init"])
            return True
        elif cmd_arg == "commit":
            run(["git.exe", "add", "-A"])
            run(["git.exe", "commit", "-m", commit_msg()])
            return True
    if cmd_type == "file":
        filename = cmd + '.txt'
        fo_mode = 'r+' if os.path.exists(filename) else 'w+'

        fo = open(filename, fo_mode)
        file_lines = fo.readlines() # goto eof
        if cmd_arg == "add":
            fo.write(edit_val() + "\n")
            fo.close()
            return True
        if cmd_arg == "update":
            rand_file_idx = randint(0, len(file_lines) - 1)
            file_lines[rand_file_idx] = file_lines[rand_file_idx].rstrip() + edit_val() + "\n"
            fo.seek(0)
            fo.write("".join(file_lines))
            fo.close()
            return True
        if cmd_arg.startswith("hugefile"):
            line_count = int(cmd_arg[8:])
            for i in range(0, line_count):
                fo.write("Line nr " + str(i + 1) + "\n")
            fo.close()
            return True
        if cmd_arg == "unicode":
            fo.close() # reopen for binary
            fo = open(filename, 'w+b')
            fo.write(b''.join([bytearray("".join(file_lines), "utf-8"), unicode_data]))
            fo.close()
            return True
    return False


json_content = Path(args.cmds).read_text()
cmd_script = json.loads(json_content)

ufo = open('unicode.txt', 'rb')
unicode_data = ufo.read()
ufo.close()

# goto folder for cmds
os.makedirs(args.path)
os.chdir(args.path)

regex = re.compile("^([^:]+):([^:]+):?([^:]+)?$")
# Normal processing starts
for cmd_str in cmd_script:
    m = regex.match(cmd_str)
    cmd_type = m.group(1)
    if cmd_type == "git":
        cmd = cmd_type
        cmd_arg = m.group(2)
    else:
        cmd = m.group(2)
        cmd_arg = m.group(3)
    if not execute_cmd(cmd_type, cmd, cmd_arg):
        print("unhandled", cmd_str)
        break
