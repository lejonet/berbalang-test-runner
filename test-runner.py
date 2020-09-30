#!/usr/bin/env python
# -*- coding: utf-8 -*-

import toml, argparse, pylxd
from sys import argv

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
        self.test_spec = list()
        for test in kwargs.get('test_spec'):
            self.test_spec.append(TestSpecification(**test))

    def __str__(self):
        representation = f"TestOutline(source_container='{self.source_container}', container_profiles={self.container_profiles}"
        for test in self.test_spec:
            representation += f", {test}"
        return representation

def create_berbalang_config(test_name, test):
    with open(test.path_config, "r") as f_ro:
        with open(f"{test_name}-config.toml", "w") as f_wo:
            source_conf = toml.load(f_ro)
            source_conf['timeout'] = test.test_length
            dest_conf = toml.dump(source_conf, f_wo)
def run_test(instance, test_cmd):
    print(f"Instance: {instance.name}, Cmd: {test_cmd}")
    output = instance.execute(["ls", "-l", "-a", "-h", "config.toml"])
    print(output)

def run_tests(test_outline):
    client = pylxd.Client()
    for test in test_outline.test_spec:
        for test_nr in range(0,test.nr_of_test_runs):
            test_name = f"{test.name}-{test_nr}"

            create_berbalang_config(test_name, test)

            print(f"Creating {test_outline.source_container} to {test_name}")
            instance_config = {'name': test_name, 'profiles': test_outline.container_profiles, 'source': {'type': 'image', 'alias': test_outline.source_container}}
            print(instance_config)
            instance = client.containers.create(instance_config, wait=True)
            instance.start()
            print(f"Pushing config file {test_name}-config.toml to {test_name}")
            with open(f"{test_name}-config.toml") as f:
                instance.files.put("/root/config.toml", f)
            print(f"Executing command {test.test_cmd} in container {test_name}")
            run_test(instance, test.test_cmd)


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
