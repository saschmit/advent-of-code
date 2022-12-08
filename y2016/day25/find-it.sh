#!/bin/bash

for i in $(./xrange.py 1000); do echo $i; ./assembunny.py $i < input ; done
