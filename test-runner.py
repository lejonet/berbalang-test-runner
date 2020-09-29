#!/usr/bin/env python
# -*- coding: utf-8 -*-

import toml, argparse
from sys import argv

class TestSpecification():
    def __init__(self, **kwargs):
        self.name = kwargs.get('name')
        self.test_cmd = kwargs.get('test_cmd')
        self.nr_of_test_runs = kwargs.get('nr_of_test_runs')
        self.test_length = kwargs.get('test_length')
        self.path_config = kwargs.get('path_config')

    def __str__(self):
        return f"TestSpecification(name={self.name}, test_cmd={self.test_cmd}, nr_of_test_runs={self.nr_of_test_runs}, test_length={self.test_length}, path_config={self.path_config})"

class TestOutline():
    def __init__(self, **kwargs):
        self.source_container = kwargs.get('source_container')
        self.container_profiles = kwargs.get('container_profiles')
        self.test_spec = list()
        for test in kwargs.get('test_spec'):
            self.test_spec.append(TestSpecification(**test))

    def __str__(self):
        representation = f"TestOutline(source_container={self.source_container}, container_profiles={self.container_profiles}"
        for test in self.test_spec:
            representation += f", {test}"
        return representation

def run_tests(test_specification):
    test_outline = TestOutline(**test_specification)
    print(test_outline)

def run():
    parser = argparse.ArgumentParser(description='Run a bunch of berbalang tests in LXC containers')
    parser.add_argument('test_specification', metavar='f', type=str, nargs='?', default="test_specification.toml", help="path to the test specification TOML file")
    args = parser.parse_args()
    with open(args.test_specification, "r") as f:
        parsed_test_specification = toml.load(f)

        run_tests(parsed_test_specification)

if __name__ == '__main__':
    run()
