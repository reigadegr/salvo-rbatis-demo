#!/bin/sh
{
    for i in $(find  ./src -name "*.rs"); do
        echo "这是$i:"
        cat $i
        echo -e "\n ------------"
    done
} >all.rs