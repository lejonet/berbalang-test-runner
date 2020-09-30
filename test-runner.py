#!/usr/bin/env python
# -*- coding: utf-8 -*-

import toml, argparse, pylxd
from sys import argv
from os import mkdir

class TestSpecification():
    def __init__(self, **kwargs):
        self.name = kwargs.get('name')
        self.test_cmd = kwargs.get('test_cmd')
        self.nr_of_test_runs = kwargs.get('nr_of_test_runs')
        self.test_length = kwargs.get('test_length')
        self.path_config = kwargs.get('path_config')

    def __str__(self):
        return f"TestSpecification(name='{self.name}', test_cmd='{self.test_cmd}', nr_of_test_runs={self.nr_of_test_runs}, test_length='{self.test_length}', path_config='{self.path_config}')"

class TestOutline():
    def __init__(self, **kwargs):
        self.source_container = kwargs.get('source_container')
        self.container_profiles = kwargs.get('container_profiles')
        self.nodes = kwargs.get('nodes', list())
        self.output_dir = kwargs.get('output_dir')
        self.test_spec = list()
        for test in kwargs.get('test_spec'):
            self.test_spec.append(TestSpecification(**test))

    def __str__(self):
        representation = f"TestOutline(source_container='{self.source_container}', container_profiles={self.container_profiles}, output_dir='{self.output_dir}'"
        for test in self.test_spec:
            representation += f", {test}"
        return representation

def create_berbalang_config(test_name, test, output_dir):
    print(f"Creating directory {output_dir}/{test_name}")
    mkdir(f"{output_dir}/{test_name}", 0o755)
    with open(test.path_config, "r") as f_ro:
        with open(f"{output_dir}/{test_name}/config.toml", "w") as f_wo:
            source_conf = toml.load(f_ro)
            source_conf['timeout'] = test.test_length
            dest_conf = toml.dump(source_conf, f_wo)

def execute_command(instance, test_cmd, output_dir):
    print(f"Instance: {instance.name}, Cmd: {test_cmd}")
    (exit_code, output_stdout, output_stderr) = instance.execute(test_cmd)
    with open(f"{output_dir}/{instance.name}/stdout", "w") as stdout:
        with open(f"{output_dir}/{instance.name}/stderr", "w") as stderr:
            with open(f"{output_dir}/{instance.name}/exitcode", "w") as exitcode:
                if output_stdout == '':
                    stdout.close()
                else:
                    stdout.write(output_stdout)

                if output_stderr == '':
                    stderr.close()
                else:
                    stderr.write(output_stderr)
               
                exitcode.write(f"{exit_code}")
    instance.stop()

def run_test(test, test_outline, client, target):
    for test_nr in range(0,test.nr_of_test_runs):
        test_name = f"{test.name}-{test_nr}"

        create_berbalang_config(test_name, test, test_outline.output_dir)

        print(f"Creating {test_outline.source_container} to {test_name}")
        source = None
        if not target:
            source = {'type': 'image', 'alias': source_container}
        instance_config = {'name': test_name, 'profiles': container_profiles, 'source': source}
        print(instance_config)
        instance = client.containers.create(instance_config, wait=True)
        instance.start()
        print(f"Pushing config file {test_outline.output_dir}/{test_name}/config.toml to {test_name}")
        with open(f"{test_outline.output_dir}/{test_name}/config.toml") as f:
            instance.files.put("/root/config.toml", f)

        print(f"Executing command {test.test_cmd} in container {test_name}")
        execute_command(instance, test.test_cmd, test.output_dir)

def run_tests(test_outline):
    if not test_outline.nodes:
        client = pylxd.Client() 
        for test in test_outline.test_spec:
            run_test(test, test_outline, client, None)
    else:
        pass

def run(test_outline):
    test_outline = TestOutline(**test_outline)
    print("Test outline:")
    print(f"Source container: {test_outline.source_container}, Container profiles: {test_outline.container_profiles}")
    total_nr_of_tests = 0
    for test in test_outline.test_spec:
        print(test)
        total_nr_of_tests += test.nr_of_test_runs

    print(f"Amount of tests: {total_nr_of_tests}")
    run_tests(test_outline)

def main():
    parser = argparse.ArgumentParser(description='Run a bunch of berbalang tests in LXC containers')
    parser.add_argument('test_specification', metavar='f', type=str, nargs='?', default="test_specification.toml", help="path to the test specification TOML file")
    args = parser.parse_args()
    with open(args.test_specification, "r") as f:
        parsed_test_outline = toml.load(f)

        run(parsed_test_outline)

if __name__ == '__main__':
    main()
