package com.aoc.Day01;

import com.aoc.Day;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Day01 implements Day {

    @Override
    public String part1(List<String> input) {
        List<Integer> integerList = new ArrayList<>();
        for(String j : input) {
            integerList.add(Integer.parseInt(j));
        }

        int cpt = 0, before = 0;
        for(int j : integerList) {
            if(j > before && before != 0) {
                cpt++;
            }
            before = j;
        }
        return input.isEmpty() ? "" : cpt + "";
    }

    @Override
    public String part2(List<String> input) {
        return input.isEmpty() ? "" : input.get(0);
    }

}
