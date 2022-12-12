#!/bin/bash

for i in {0..100}; do
  autocannon --method GET --connection 100 --amount 10000 --duration 60 http://0.0.0.0:7777/test
done
