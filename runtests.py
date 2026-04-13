#!/usr/bin/env python3

import sys
import os
import subprocess
import stat

C_BOLD = '\x1b[1m'
C_RED = '\x1b[31m'
C_GREEN = '\x1b[32m'
C_CLEAR = '\x1b[0m'


def get_tool():
    import json
    result = subprocess.run(
        ['cargo', 'metadata', '--format-version', '1'], capture_output=True, text=True)
    result.check_returncode()
    md = json.loads(result.stdout)

    print('building cli tool')
    result = subprocess.run(['cargo', 'build', '-F', 'cli', '--release'])
    result.check_returncode()

    path = os.path.join(md['target_directory'], 'release', 'yin-yang-extract')
    if not os.path.exists(path):
        raise Exception(f'no file at {path}')

    return path


def load_cases(root):
    cases = []
    path = os.path.join(root, 'testcases')

    next_png = None

    print(f'collecting testcases in {path}')
    for ent in sorted(os.listdir(path)):
        if next_png is None and ent.endswith('.png'):
            next_png = ent
        elif next_png is not None and ent.endswith('.txt'):
            if next_png[:-4] != ent[:-4]:
                print(f'skipping {ent}')
            else:
                print(f'loading {next_png},{ent}')
                output_path = os.path.join(path, ent)
                with open(output_path) as f:
                    cases.append((os.path.join(path, next_png), f.read()))
                next_png = None
        else:
            print(f'skipping {ent}')

    return cases


def run_one_test(root, tool, input, output):
    debugout = os.path.join(root, 'testcases', 'output',
                            os.path.basename(input))
    result = subprocess.run(
        [tool, input, '--debug-output', debugout], capture_output=True, text=True)
    if result.returncode != 0:
        print(result.stderr, end='')
        print(f'{C_BOLD}{C_RED}FAIL: {input}: exited with nonzero status{C_CLEAR}')
        return False
    elif output.strip() != result.stdout.strip():
        print(f'{C_BOLD}{C_RED}FAIL: {input}: output differed{C_CLEAR}')
        print(result.stderr, end='')
        expect_lines = output.strip().split('\n')
        actual_lines = result.stdout.strip().split('\n')
        for (expect, actual) in zip(expect_lines, actual_lines):
            for e, a in zip(expect.split(' '), actual.split(' ')):
                if e != a:
                    print(f' {C_BOLD}{C_RED}{e}{C_CLEAR}', end='')
                else:
                    print(f' {e}', end='')
            print('')
        print(f'debug output written to {debugout}')
        return False
    else:
        print(f'{C_GREEN}SUCCESS: {input}{C_CLEAR}')
        return True


if __name__ == '__main__':
    root = os.path.dirname(os.path.abspath(__file__))

    os.chdir(root)
    tool = get_tool()

    cases = load_cases(root)
    failures = 0
    for input, output in cases:
        success = run_one_test(root, tool, input, output)
        if not success:
            failures += 1

    print('{}/{} passed'.format(len(cases) - failures, len(cases)))
    if failures > 0:
        sys.exit(1)
