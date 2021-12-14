package com.aoc.Day02;

import com.aoc.Day;

import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day02 implements Day {
    @Override
    public String part1(List<String> input) {
        Map<String, Integer> map = new HashMap<>();
        map.put("horizontalPosition", 0);
        map.put("depth", 0);

        for(String i : input) {
            String[] tmp = i.split(" ");
            switch (tmp[0]) {
                case "forward" -> map.put("horizontalPosition", map.get("horizontalPosition") + Integer.parseInt(tmp[1]));
                case "down" -> map.put("depth", map.get("depth") + Integer.parseInt(tmp[1]));
                case "up" -> map.put("depth", map.get("depth") - Integer.parseInt(tmp[1]));
            }
        }
        return map.get("horizontalPosition") * map.get("depth") + "";
    }

    @Override
    public String part2(List<String> input) {
        return input.get(0);
    }
}
