package com.aoc.Day02;

import com.aoc.Day;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day02 implements Day {
    @Override
    public String part1(List<String> input) {
        int depth = 0, horizontalPosition = 0;
        for(String i : input) {
            String[] tmp = i.split(" ");
            switch (tmp[0]) {
                case "forward" -> horizontalPosition += Integer.parseInt(tmp[1]);
                case "down" -> depth += Integer.parseInt(tmp[1]);
                case "up" -> depth -= Integer.parseInt(tmp[1]);
            }
        }
        return depth * horizontalPosition + "";
    }

    @Override
    public String part2(List<String> input) {
        return input.get(0);
    }
}
